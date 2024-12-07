use std::collections::BTreeSet;
use std::mem;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::Stream;
use futures_util::{stream, FutureExt, StreamExt, TryStreamExt};
use vortex_array::array::ChunkedArray;
use vortex_array::{ArrayData, IntoArrayData};
use vortex_dtype::DType;
use vortex_error::{
    vortex_bail, vortex_panic, VortexError, VortexResult, VortexUnwrap,
};
use vortex_io::{IoDispatcher, VortexReadAt};

use crate::read::cache::LayoutMessageCache;
use crate::read::mask::RowMask;
use crate::read::splits::{FixedSplitIterator, MaskIterator, PruningSplitIterator, SplitMask};
use crate::read::{BatchRead, LayoutReader, MessageId, MessageLocator};
use crate::LazyDType;

/// An asynchronous Vortex file that returns a [`Stream`] of [`ArrayData`]s.
///
/// The file may be read from any source implementing [`VortexReadAt`], such
/// as memory, disk, and object storage.
///
/// Use [VortexReadBuilder][crate::read::builder::VortexReadBuilder] to build one
/// from a reader.
pub struct VortexFileArrayStream<R> {
    dtype: Arc<LazyDType>,
    row_count: u64,
    layout_reader: Box<dyn LayoutReader>,
    messages_cache: Arc<RwLock<LayoutMessageCache>>,
    state: Option<StreamingState>,
    input: R,
    _dispatcher: Arc<IoDispatcher>,
}

impl<R: VortexReadAt> VortexFileArrayStream<R> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        input: R,
        layout_reader: Box<dyn LayoutReader>,
        filter_reader: Option<Box<dyn LayoutReader>>,
        messages_cache: Arc<RwLock<LayoutMessageCache>>,
        dtype: Arc<LazyDType>,
        row_count: u64,
        row_mask: Option<RowMask>,
        dispatcher: Arc<IoDispatcher>,
    ) -> Self {
        let mask_iterator = if let Some(fr) = filter_reader {
            Box::new(PruningSplitIterator::new(fr, row_count, row_mask)) as MaskIteratorRef
        } else {
            Box::new(FixedSplitIterator::new(row_count, row_mask))
        };

        Self {
            dtype,
            row_count,
            layout_reader,
            messages_cache,
            state: Some(StreamingState::AddSplits(mask_iterator)),
            input,
            _dispatcher: dispatcher,
        }
    }

    pub fn dtype(&self) -> &DType {
        self.dtype.value().vortex_unwrap()
    }

    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    fn store_messages(&self, messages: Vec<Message>) {
        let mut write_cache_guard = self
            .messages_cache
            .write()
            .unwrap_or_else(|poison| vortex_panic!("Failed to write to message cache: {poison}"));
        for Message(message_id, buf) in messages {
            write_cache_guard.set(message_id, buf);
        }
    }
}

/// A message that has had its bytes materialized onto the heap.
#[derive(Debug, Clone)]
pub(crate) struct Message(pub MessageId, pub Bytes);

pub(crate) type StreamMessages = Vec<Message>;

type MaskIteratorRef = Box<dyn MaskIterator>;

/// State of vortex file stream
///
/// The flow starts with `AddSplits` which produces all horizontal row splits in the file
/// Main read loop goes from `NextSplit` -> `Read`
/// `NextSplit` and `Read` states transition to `Reading` when they're blocked on an io operation which resumes back to
/// the previous state.
enum StreamingState {
    AddSplits(MaskIteratorRef),
    NextSplit(MaskIteratorRef),
    Read(RowMask, MaskIteratorRef),
    EndOfStream,
    Error,
}

enum StreamingTransition {
    GoTo(StreamingState),
    #[allow(dead_code)]
    YieldTo(StreamingState),
    Produce(StreamingState, ArrayData),
    Finished,
}

fn goto(next_state: StreamingState) -> VortexResult<StreamingTransition> {
    Ok(StreamingTransition::GoTo(next_state))
}

fn produce(next_state: StreamingState, array: ArrayData) -> VortexResult<StreamingTransition> {
    Ok(StreamingTransition::Produce(next_state, array))
}

fn finished() -> VortexResult<StreamingTransition> {
    Ok(StreamingTransition::Finished)
}

impl<R: VortexReadAt + Unpin> VortexFileArrayStream<R> {
    fn step(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        current_state: StreamingState,
    ) -> VortexResult<StreamingTransition> {
        match current_state {
            StreamingState::AddSplits(mut mask_iter) => {
                let mut reader_splits = BTreeSet::new();
                self.layout_reader.add_splits(0, &mut reader_splits)?;
                mask_iter.additional_splits(&mut reader_splits)?;
                goto(StreamingState::NextSplit(mask_iter))
            }
            StreamingState::NextSplit(mut mask_iter) => {
                let Some(mask) = mask_iter.next() else {
                    return finished();
                };
                match mask? {
                    SplitMask::ReadMore(messages) => {
                        let read_future = self.read_ranges(messages);
                        let messages = match pin!(read_future).poll(cx) {
                            Poll::Ready(r) => r?,
                            Poll::Pending => panic!("sync only"),
                        };
                        let next_state = StreamingState::NextSplit(mask_iter);
                        self.store_messages(messages);
                        goto(next_state)
                    },
                    SplitMask::Mask(m) => goto(StreamingState::Read(m, mask_iter)),
                }
            }
            StreamingState::Read(selector, filter_reader) => {
                match self.layout_reader.read_selection(&selector)? {
                    Some(BatchRead::ReadMore(message_ranges)) => {
                        let read_future = self.read_ranges(message_ranges);
                        let messages = match pin!(read_future).poll(cx) {
                            Poll::Ready(r) => r?,
                            Poll::Pending => panic!("sync only"),
                        };
                        let next_state = StreamingState::Read(selector, filter_reader);
                        self.store_messages(messages);
                        goto(next_state)
                    }
                    Some(BatchRead::Batch(array)) => {
                        produce(StreamingState::NextSplit(filter_reader), array)
                    }
                    None => goto(StreamingState::NextSplit(filter_reader)),
                }
            }
            StreamingState::Error => vortex_bail!("you polled a stream that previously erred"),
            StreamingState::EndOfStream => finished(),
        }
    }

    /// Schedule an asynchronous read of several byte ranges.
    ///
    /// IO is scheduled on the provided IO dispatcher.
    fn read_ranges(
        &self,
        ranges: Vec<MessageLocator>,
    ) -> impl Future<Output = VortexResult<StreamMessages>> {
        let reader = self.input.clone();

        read_ranges(reader, ranges)
    }
}

impl<R: VortexReadAt + Unpin> Stream for VortexFileArrayStream<R> {
    type Item = VortexResult<ArrayData>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let Some(current_state) = mem::take(&mut self.state) else {
                vortex_panic!("called poll_next while in poll_next");
            };

            match self.as_mut().step(cx, current_state) {
                Ok(StreamingTransition::GoTo(next_state)) => self.state = Some(next_state),
                Ok(StreamingTransition::YieldTo(next_state)) => {
                    self.state = Some(next_state);
                    return Poll::Pending;
                }
                Ok(StreamingTransition::Produce(next_state, array)) => {
                    self.state = Some(next_state);
                    return Poll::Ready(Some(Ok(array)));
                }
                Ok(StreamingTransition::Finished) => {
                    self.state = Some(StreamingState::EndOfStream);
                    return Poll::Ready(None);
                }
                Err(error) => {
                    self.state = Some(StreamingState::Error);
                    return Poll::Ready(Some(Err(error)));
                }
            }
        }
    }
}

impl<R: VortexReadAt + Unpin> VortexFileArrayStream<R> {
    pub async fn read_all(self) -> VortexResult<ArrayData> {
        let dtype = self.dtype().clone();
        let vecs: Vec<ArrayData> = self.try_collect().await?;
        if vecs.len() == 1 {
            vecs.into_iter().next().ok_or_else(|| {
                vortex_panic!(
                    "Should be impossible: vecs.len() == 1 but couldn't get first element"
                )
            })
        } else {
            ChunkedArray::try_new(vecs, dtype).map(|e| e.into_array())
        }
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
pub async fn read_ranges<R: VortexReadAt>(
    reader: R,
    ranges: Vec<MessageLocator>,
) -> VortexResult<Vec<Message>> {
    stream::iter(ranges.into_iter())
        .map(|MessageLocator(id, range)| {
            let read_ft = reader.read_byte_range(range.begin, range.len());

            read_ft.map(|result| {
                result
                    .map(|res| Message(id, res))
                    .map_err(VortexError::from)
            })
        })
        .buffered(10)
        .try_collect()
        .await
}
