extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct Position<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for Position<'a> {
    type Inner = Position<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

#[allow(dead_code)]
impl<'a> Position<'a> {
    pub const VT_X: ::flatbuffers::VOffsetT = 4;
    pub const VT_Z: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: ::flatbuffers::Table<'a>) -> Self {
        Position { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<
        'bldr: 'args,
        'args: 'mut_bldr,
        'mut_bldr,
        A: ::flatbuffers::Allocator + 'bldr,
    >(
        _fbb: &'mut_bldr mut ::flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args PositionArgs,
    ) -> ::flatbuffers::WIPOffset<Position<'bldr>> {
        let mut builder = PositionBuilder::new(_fbb);
        builder.add_z(args.z);
        builder.add_x(args.x);
        builder.finish()
    }

    pub fn unpack(&self) -> PositionT {
        let x = self.x();
        let z = self.z();
        PositionT { x, z }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<f32>(Position::VT_X, Some(0.0)).unwrap() }
    }
    #[inline]
    pub fn z(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<f32>(Position::VT_Z, Some(0.0)).unwrap() }
    }
}

impl ::flatbuffers::Verifiable for Position<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<f32>("x", Self::VT_X, false)?
            .visit_field::<f32>("z", Self::VT_Z, false)?
            .finish();
        Ok(())
    }
}
pub struct PositionArgs {
    pub x: f32,
    pub z: f32,
}
impl<'a> Default for PositionArgs {
    #[inline]
    fn default() -> Self {
        PositionArgs { x: 0.0, z: 0.0 }
    }
}

impl Serialize for Position<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Position", 2)?;
        s.serialize_field("x", &self.x())?;
        s.serialize_field("z", &self.z())?;
        s.end()
    }
}

pub struct PositionBuilder<'a: 'b, 'b, A: ::flatbuffers::Allocator + 'a> {
    fbb_: &'b mut ::flatbuffers::FlatBufferBuilder<'a, A>,
    start_: ::flatbuffers::WIPOffset<::flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: ::flatbuffers::Allocator + 'a> PositionBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_x(&mut self, x: f32) {
        self.fbb_.push_slot::<f32>(Position::VT_X, x, 0.0);
    }
    #[inline]
    pub fn add_z(&mut self, z: f32) {
        self.fbb_.push_slot::<f32>(Position::VT_Z, z, 0.0);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut ::flatbuffers::FlatBufferBuilder<'a, A>,
    ) -> PositionBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        PositionBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> ::flatbuffers::WIPOffset<Position<'a>> {
        let o = self.fbb_.end_table(self.start_);
        ::flatbuffers::WIPOffset::new(o.value())
    }
}

impl ::core::fmt::Debug for Position<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("Position");
        ds.field("x", &self.x());
        ds.field("z", &self.z());
        ds.finish()
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PositionT {
    pub x: f32,
    pub z: f32,
}
impl Default for PositionT {
    fn default() -> Self {
        Self { x: 0.0, z: 0.0 }
    }
}

#[allow(dead_code)]
impl PositionT {
    pub fn pack<'b, A: ::flatbuffers::Allocator + 'b>(
        &self,
        _fbb: &mut ::flatbuffers::FlatBufferBuilder<'b, A>,
    ) -> ::flatbuffers::WIPOffset<Position<'b>> {
        let x = self.x;
        let z = self.z;
        Position::create(_fbb, &PositionArgs { x, z })
    }
}
