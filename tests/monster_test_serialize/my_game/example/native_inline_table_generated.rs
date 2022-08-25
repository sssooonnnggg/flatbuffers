// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
extern crate serde;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum NativeInlineTableOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct NativeInlineTable<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for NativeInlineTable<'a> {
  type Inner = NativeInlineTable<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> NativeInlineTable<'a> {
  pub const VT_A: flatbuffers::VOffsetT = 4;

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Example.NativeInlineTable"
  }

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    NativeInlineTable { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args NativeInlineTableArgs
  ) -> flatbuffers::WIPOffset<NativeInlineTable<'bldr>> {
    let mut builder = NativeInlineTableBuilder::new(_fbb);
    builder.add_a(args.a);
    builder.finish()
  }

  pub fn unpack(&self) -> NativeInlineTableT {
    let a = self.a();
    NativeInlineTableT {
      a,
    }
  }

  #[inline]
  pub fn a(&self) -> i32 {
    self._tab.get::<i32>(NativeInlineTable::VT_A, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for NativeInlineTable<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("a", Self::VT_A, false)?
     .finish();
    Ok(())
  }
}
pub struct NativeInlineTableArgs {
    pub a: i32,
}
impl<'a> Default for NativeInlineTableArgs {
  #[inline]
  fn default() -> Self {
    NativeInlineTableArgs {
      a: 0,
    }
  }
}

impl Serialize for NativeInlineTable<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("NativeInlineTable", 1)?;
      s.serialize_field("a", &self.a())?;
    s.end()
  }
}

pub struct NativeInlineTableBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NativeInlineTableBuilder<'a, 'b> {
  #[inline]
  pub fn add_a(&mut self, a: i32) {
    self.fbb_.push_slot::<i32>(NativeInlineTable::VT_A, a, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NativeInlineTableBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NativeInlineTableBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<NativeInlineTable<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for NativeInlineTable<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("NativeInlineTable");
      ds.field("a", &self.a());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct NativeInlineTableT {
  pub a: i32,
}
impl Default for NativeInlineTableT {
  fn default() -> Self {
    Self {
      a: 0,
    }
  }
}
impl NativeInlineTableT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<NativeInlineTable<'b>> {
    let a = self.a;
    NativeInlineTable::create(_fbb, &NativeInlineTableArgs{
      a,
    })
  }
}
