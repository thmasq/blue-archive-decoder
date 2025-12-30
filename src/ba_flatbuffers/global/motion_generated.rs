extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{Position, PositionT};

#[derive(Copy, Clone, PartialEq)]
pub struct Motion<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for Motion<'a> {
    type Inner = Motion<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

#[allow(dead_code)]
impl<'a> Motion<'a> {
    pub const VT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_POSITIONS: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: ::flatbuffers::Table<'a>) -> Self {
        Motion { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<
        'bldr: 'args,
        'args: 'mut_bldr,
        'mut_bldr,
        A: ::flatbuffers::Allocator + 'bldr,
    >(
        _fbb: &'mut_bldr mut ::flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args MotionArgs<'args>,
    ) -> ::flatbuffers::WIPOffset<Motion<'bldr>> {
        let mut builder = MotionBuilder::new(_fbb);
        if let Some(x) = args.positions {
            builder.add_positions(x);
        }
        if let Some(x) = args.name {
            builder.add_name(x);
        }
        builder.finish()
    }

    pub fn unpack(&self) -> MotionT {
        let name = self.name().map(|x| alloc::string::ToString::to_string(x));
        let positions = self
            .positions()
            .map(|x| x.iter().map(|t| t.unpack()).collect());
        MotionT { name, positions }
    }

    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(Motion::VT_NAME, None)
        }
    }
    #[inline]
    pub fn positions(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<Position<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<Position>>,
            >>(Motion::VT_POSITIONS, None)
        }
    }
}

impl ::flatbuffers::Verifiable for Motion<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<Position>>,
            >>("positions", Self::VT_POSITIONS, false)?
            .finish();
        Ok(())
    }
}
pub struct MotionArgs<'a> {
    pub name: Option<::flatbuffers::WIPOffset<&'a str>>,
    pub positions: Option<
        ::flatbuffers::WIPOffset<
            ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<Position<'a>>>,
        >,
    >,
}
impl<'a> Default for MotionArgs<'a> {
    #[inline]
    fn default() -> Self {
        MotionArgs {
            name: None,
            positions: None,
        }
    }
}

impl Serialize for Motion<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Motion", 2)?;
        if let Some(f) = self.name() {
            s.serialize_field("name", &f)?;
        } else {
            s.skip_field("name")?;
        }
        if let Some(f) = self.positions() {
            s.serialize_field("positions", &f)?;
        } else {
            s.skip_field("positions")?;
        }
        s.end()
    }
}

pub struct MotionBuilder<'a: 'b, 'b, A: ::flatbuffers::Allocator + 'a> {
    fbb_: &'b mut ::flatbuffers::FlatBufferBuilder<'a, A>,
    start_: ::flatbuffers::WIPOffset<::flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: ::flatbuffers::Allocator + 'a> MotionBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_name(&mut self, name: ::flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<::flatbuffers::WIPOffset<_>>(Motion::VT_NAME, name);
    }
    #[inline]
    pub fn add_positions(
        &mut self,
        positions: ::flatbuffers::WIPOffset<
            ::flatbuffers::Vector<'b, ::flatbuffers::ForwardsUOffset<Position<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<::flatbuffers::WIPOffset<_>>(Motion::VT_POSITIONS, positions);
    }
    #[inline]
    pub fn new(_fbb: &'b mut ::flatbuffers::FlatBufferBuilder<'a, A>) -> MotionBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        MotionBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> ::flatbuffers::WIPOffset<Motion<'a>> {
        let o = self.fbb_.end_table(self.start_);
        ::flatbuffers::WIPOffset::new(o.value())
    }
}

impl ::core::fmt::Debug for Motion<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("Motion");
        ds.field("name", &self.name());
        ds.field("positions", &self.positions());
        ds.finish()
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MotionT {
    pub name: Option<alloc::string::String>,
    pub positions: Option<alloc::vec::Vec<PositionT>>,
}
impl Default for MotionT {
    fn default() -> Self {
        Self {
            name: None,
            positions: None,
        }
    }
}

#[allow(dead_code)]
impl MotionT {
    pub fn pack<'b, A: ::flatbuffers::Allocator + 'b>(
        &self,
        _fbb: &mut ::flatbuffers::FlatBufferBuilder<'b, A>,
    ) -> ::flatbuffers::WIPOffset<Motion<'b>> {
        let name = self.name.as_ref().map(|x| _fbb.create_string(x));
        let positions = self.positions.as_ref().map(|x| {
            let w: alloc::vec::Vec<_> = x.iter().map(|t| t.pack(_fbb)).collect();
            _fbb.create_vector(&w)
        });
        Motion::create(_fbb, &MotionArgs { name, positions })
    }
}
