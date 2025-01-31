// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::dtype::*;
use crate::scalar::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_VERSION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_VERSION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_VERSION: [Version; 1] = [
  Version::V0,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Version(pub u8);
#[allow(non_upper_case_globals)]
impl Version {
  pub const V0: Self = Self(0);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 0;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::V0,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::V0 => Some("V0"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Version {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Version {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Version {
    type Output = Version;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Version {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Version {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Version {}
pub enum ArrayOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Array<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Array<'a> {
  type Inner = Array<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Array<'a> {
  pub const VT_VERSION: flatbuffers::VOffsetT = 4;
  pub const VT_BUFFER_INDEX: flatbuffers::VOffsetT = 6;
  pub const VT_ENCODING: flatbuffers::VOffsetT = 8;
  pub const VT_METADATA: flatbuffers::VOffsetT = 10;
  pub const VT_STATS: flatbuffers::VOffsetT = 12;
  pub const VT_CHILDREN: flatbuffers::VOffsetT = 14;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Array { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ArrayArgs<'args>
  ) -> flatbuffers::WIPOffset<Array<'bldr>> {
    let mut builder = ArrayBuilder::new(_fbb);
    if let Some(x) = args.buffer_index { builder.add_buffer_index(x); }
    if let Some(x) = args.children { builder.add_children(x); }
    if let Some(x) = args.stats { builder.add_stats(x); }
    if let Some(x) = args.metadata { builder.add_metadata(x); }
    builder.add_encoding(args.encoding);
    builder.add_version(args.version);
    builder.finish()
  }


  #[inline]
  pub fn version(&self) -> Version {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Version>(Array::VT_VERSION, Some(Version::V0)).unwrap()}
  }
  #[inline]
  pub fn buffer_index(&self) -> Option<u64> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Array::VT_BUFFER_INDEX, None)}
  }
  #[inline]
  pub fn encoding(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(Array::VT_ENCODING, Some(0)).unwrap()}
  }
  #[inline]
  pub fn metadata(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Array::VT_METADATA, None)}
  }
  #[inline]
  pub fn stats(&self) -> Option<ArrayStats<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<ArrayStats>>(Array::VT_STATS, None)}
  }
  #[inline]
  pub fn children(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Array<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Array>>>>(Array::VT_CHILDREN, None)}
  }
}

impl flatbuffers::Verifiable for Array<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Version>("version", Self::VT_VERSION, false)?
     .visit_field::<u64>("buffer_index", Self::VT_BUFFER_INDEX, false)?
     .visit_field::<u16>("encoding", Self::VT_ENCODING, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("metadata", Self::VT_METADATA, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<ArrayStats>>("stats", Self::VT_STATS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Array>>>>("children", Self::VT_CHILDREN, false)?
     .finish();
    Ok(())
  }
}
pub struct ArrayArgs<'a> {
    pub version: Version,
    pub buffer_index: Option<u64>,
    pub encoding: u16,
    pub metadata: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub stats: Option<flatbuffers::WIPOffset<ArrayStats<'a>>>,
    pub children: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Array<'a>>>>>,
}
impl<'a> Default for ArrayArgs<'a> {
  #[inline]
  fn default() -> Self {
    ArrayArgs {
      version: Version::V0,
      buffer_index: None,
      encoding: 0,
      metadata: None,
      stats: None,
      children: None,
    }
  }
}

pub struct ArrayBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ArrayBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_version(&mut self, version: Version) {
    self.fbb_.push_slot::<Version>(Array::VT_VERSION, version, Version::V0);
  }
  #[inline]
  pub fn add_buffer_index(&mut self, buffer_index: u64) {
    self.fbb_.push_slot_always::<u64>(Array::VT_BUFFER_INDEX, buffer_index);
  }
  #[inline]
  pub fn add_encoding(&mut self, encoding: u16) {
    self.fbb_.push_slot::<u16>(Array::VT_ENCODING, encoding, 0);
  }
  #[inline]
  pub fn add_metadata(&mut self, metadata: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Array::VT_METADATA, metadata);
  }
  #[inline]
  pub fn add_stats(&mut self, stats: flatbuffers::WIPOffset<ArrayStats<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ArrayStats>>(Array::VT_STATS, stats);
  }
  #[inline]
  pub fn add_children(&mut self, children: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Array<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Array::VT_CHILDREN, children);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ArrayBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    ArrayBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Array<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Array<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Array");
      ds.field("version", &self.version());
      ds.field("buffer_index", &self.buffer_index());
      ds.field("encoding", &self.encoding());
      ds.field("metadata", &self.metadata());
      ds.field("stats", &self.stats());
      ds.field("children", &self.children());
      ds.finish()
  }
}
pub enum ArrayStatsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ArrayStats<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ArrayStats<'a> {
  type Inner = ArrayStats<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ArrayStats<'a> {
  pub const VT_MIN: flatbuffers::VOffsetT = 4;
  pub const VT_MAX: flatbuffers::VOffsetT = 6;
  pub const VT_IS_SORTED: flatbuffers::VOffsetT = 8;
  pub const VT_IS_STRICT_SORTED: flatbuffers::VOffsetT = 10;
  pub const VT_IS_CONSTANT: flatbuffers::VOffsetT = 12;
  pub const VT_RUN_COUNT: flatbuffers::VOffsetT = 14;
  pub const VT_TRUE_COUNT: flatbuffers::VOffsetT = 16;
  pub const VT_NULL_COUNT: flatbuffers::VOffsetT = 18;
  pub const VT_BIT_WIDTH_FREQ: flatbuffers::VOffsetT = 20;
  pub const VT_TRAILING_ZERO_FREQ: flatbuffers::VOffsetT = 22;
  pub const VT_UNCOMPRESSED_SIZE_IN_BYTES: flatbuffers::VOffsetT = 24;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ArrayStats { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ArrayStatsArgs<'args>
  ) -> flatbuffers::WIPOffset<ArrayStats<'bldr>> {
    let mut builder = ArrayStatsBuilder::new(_fbb);
    if let Some(x) = args.uncompressed_size_in_bytes { builder.add_uncompressed_size_in_bytes(x); }
    if let Some(x) = args.null_count { builder.add_null_count(x); }
    if let Some(x) = args.true_count { builder.add_true_count(x); }
    if let Some(x) = args.run_count { builder.add_run_count(x); }
    if let Some(x) = args.trailing_zero_freq { builder.add_trailing_zero_freq(x); }
    if let Some(x) = args.bit_width_freq { builder.add_bit_width_freq(x); }
    if let Some(x) = args.max { builder.add_max(x); }
    if let Some(x) = args.min { builder.add_min(x); }
    if let Some(x) = args.is_constant { builder.add_is_constant(x); }
    if let Some(x) = args.is_strict_sorted { builder.add_is_strict_sorted(x); }
    if let Some(x) = args.is_sorted { builder.add_is_sorted(x); }
    builder.finish()
  }


  #[inline]
  pub fn min(&self) -> Option<ScalarValue<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<ScalarValue>>(ArrayStats::VT_MIN, None)}
  }
  #[inline]
  pub fn max(&self) -> Option<ScalarValue<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<ScalarValue>>(ArrayStats::VT_MAX, None)}
  }
  #[inline]
  pub fn is_sorted(&self) -> Option<bool> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(ArrayStats::VT_IS_SORTED, None)}
  }
  #[inline]
  pub fn is_strict_sorted(&self) -> Option<bool> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(ArrayStats::VT_IS_STRICT_SORTED, None)}
  }
  #[inline]
  pub fn is_constant(&self) -> Option<bool> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(ArrayStats::VT_IS_CONSTANT, None)}
  }
  #[inline]
  pub fn run_count(&self) -> Option<u64> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ArrayStats::VT_RUN_COUNT, None)}
  }
  #[inline]
  pub fn true_count(&self) -> Option<u64> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ArrayStats::VT_TRUE_COUNT, None)}
  }
  #[inline]
  pub fn null_count(&self) -> Option<u64> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ArrayStats::VT_NULL_COUNT, None)}
  }
  #[inline]
  pub fn bit_width_freq(&self) -> Option<flatbuffers::Vector<'a, u64>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(ArrayStats::VT_BIT_WIDTH_FREQ, None)}
  }
  #[inline]
  pub fn trailing_zero_freq(&self) -> Option<flatbuffers::Vector<'a, u64>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(ArrayStats::VT_TRAILING_ZERO_FREQ, None)}
  }
  #[inline]
  pub fn uncompressed_size_in_bytes(&self) -> Option<u64> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ArrayStats::VT_UNCOMPRESSED_SIZE_IN_BYTES, None)}
  }
}

impl flatbuffers::Verifiable for ArrayStats<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<ScalarValue>>("min", Self::VT_MIN, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<ScalarValue>>("max", Self::VT_MAX, false)?
     .visit_field::<bool>("is_sorted", Self::VT_IS_SORTED, false)?
     .visit_field::<bool>("is_strict_sorted", Self::VT_IS_STRICT_SORTED, false)?
     .visit_field::<bool>("is_constant", Self::VT_IS_CONSTANT, false)?
     .visit_field::<u64>("run_count", Self::VT_RUN_COUNT, false)?
     .visit_field::<u64>("true_count", Self::VT_TRUE_COUNT, false)?
     .visit_field::<u64>("null_count", Self::VT_NULL_COUNT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>("bit_width_freq", Self::VT_BIT_WIDTH_FREQ, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>("trailing_zero_freq", Self::VT_TRAILING_ZERO_FREQ, false)?
     .visit_field::<u64>("uncompressed_size_in_bytes", Self::VT_UNCOMPRESSED_SIZE_IN_BYTES, false)?
     .finish();
    Ok(())
  }
}
pub struct ArrayStatsArgs<'a> {
    pub min: Option<flatbuffers::WIPOffset<ScalarValue<'a>>>,
    pub max: Option<flatbuffers::WIPOffset<ScalarValue<'a>>>,
    pub is_sorted: Option<bool>,
    pub is_strict_sorted: Option<bool>,
    pub is_constant: Option<bool>,
    pub run_count: Option<u64>,
    pub true_count: Option<u64>,
    pub null_count: Option<u64>,
    pub bit_width_freq: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u64>>>,
    pub trailing_zero_freq: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u64>>>,
    pub uncompressed_size_in_bytes: Option<u64>,
}
impl<'a> Default for ArrayStatsArgs<'a> {
  #[inline]
  fn default() -> Self {
    ArrayStatsArgs {
      min: None,
      max: None,
      is_sorted: None,
      is_strict_sorted: None,
      is_constant: None,
      run_count: None,
      true_count: None,
      null_count: None,
      bit_width_freq: None,
      trailing_zero_freq: None,
      uncompressed_size_in_bytes: None,
    }
  }
}

pub struct ArrayStatsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ArrayStatsBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_min(&mut self, min: flatbuffers::WIPOffset<ScalarValue<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ScalarValue>>(ArrayStats::VT_MIN, min);
  }
  #[inline]
  pub fn add_max(&mut self, max: flatbuffers::WIPOffset<ScalarValue<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<ScalarValue>>(ArrayStats::VT_MAX, max);
  }
  #[inline]
  pub fn add_is_sorted(&mut self, is_sorted: bool) {
    self.fbb_.push_slot_always::<bool>(ArrayStats::VT_IS_SORTED, is_sorted);
  }
  #[inline]
  pub fn add_is_strict_sorted(&mut self, is_strict_sorted: bool) {
    self.fbb_.push_slot_always::<bool>(ArrayStats::VT_IS_STRICT_SORTED, is_strict_sorted);
  }
  #[inline]
  pub fn add_is_constant(&mut self, is_constant: bool) {
    self.fbb_.push_slot_always::<bool>(ArrayStats::VT_IS_CONSTANT, is_constant);
  }
  #[inline]
  pub fn add_run_count(&mut self, run_count: u64) {
    self.fbb_.push_slot_always::<u64>(ArrayStats::VT_RUN_COUNT, run_count);
  }
  #[inline]
  pub fn add_true_count(&mut self, true_count: u64) {
    self.fbb_.push_slot_always::<u64>(ArrayStats::VT_TRUE_COUNT, true_count);
  }
  #[inline]
  pub fn add_null_count(&mut self, null_count: u64) {
    self.fbb_.push_slot_always::<u64>(ArrayStats::VT_NULL_COUNT, null_count);
  }
  #[inline]
  pub fn add_bit_width_freq(&mut self, bit_width_freq: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ArrayStats::VT_BIT_WIDTH_FREQ, bit_width_freq);
  }
  #[inline]
  pub fn add_trailing_zero_freq(&mut self, trailing_zero_freq: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ArrayStats::VT_TRAILING_ZERO_FREQ, trailing_zero_freq);
  }
  #[inline]
  pub fn add_uncompressed_size_in_bytes(&mut self, uncompressed_size_in_bytes: u64) {
    self.fbb_.push_slot_always::<u64>(ArrayStats::VT_UNCOMPRESSED_SIZE_IN_BYTES, uncompressed_size_in_bytes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ArrayStatsBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    ArrayStatsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ArrayStats<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ArrayStats<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ArrayStats");
      ds.field("min", &self.min());
      ds.field("max", &self.max());
      ds.field("is_sorted", &self.is_sorted());
      ds.field("is_strict_sorted", &self.is_strict_sorted());
      ds.field("is_constant", &self.is_constant());
      ds.field("run_count", &self.run_count());
      ds.field("true_count", &self.true_count());
      ds.field("null_count", &self.null_count());
      ds.field("bit_width_freq", &self.bit_width_freq());
      ds.field("trailing_zero_freq", &self.trailing_zero_freq());
      ds.field("uncompressed_size_in_bytes", &self.uncompressed_size_in_bytes());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Array`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_unchecked`.
pub fn root_as_array(buf: &[u8]) -> Result<Array, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Array>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Array` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_array_unchecked`.
pub fn size_prefixed_root_as_array(buf: &[u8]) -> Result<Array, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Array>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Array` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_unchecked`.
pub fn root_as_array_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Array<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Array<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Array` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_unchecked`.
pub fn size_prefixed_root_as_array_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Array<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Array<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Array and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Array`.
pub unsafe fn root_as_array_unchecked(buf: &[u8]) -> Array {
  flatbuffers::root_unchecked::<Array>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Array and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Array`.
pub unsafe fn size_prefixed_root_as_array_unchecked(buf: &[u8]) -> Array {
  flatbuffers::size_prefixed_root_unchecked::<Array>(buf)
}
#[inline]
pub fn finish_array_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Array<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_array_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Array<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
