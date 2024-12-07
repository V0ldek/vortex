use super::Dispatch;
use futures::channel::oneshot;
use vortex_error::{vortex_bail, VortexResult};

#[derive(Debug)]
pub(super) struct SyncDispatcher { }

impl SyncDispatcher {
    pub fn new() -> Self {
        Self { }
    }
}

impl Dispatch for SyncDispatcher {
    fn dispatch<F, Fut, R>(&self, _task: F) -> VortexResult<oneshot::Receiver<R>>
    where
        F: (FnOnce() -> Fut) + Send + 'static,
        Fut: std::future::Future<Output = R> + 'static,
        R: Send + 'static {
        vortex_bail!("dupa")
    }

    fn shutdown(self) -> VortexResult<()> {
        Ok(())
    }
}