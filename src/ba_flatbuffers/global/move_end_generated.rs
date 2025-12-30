extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::Motion;

#[derive(Copy, Clone, PartialEq)]
pub struct MoveEnd<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MoveEnd<'a> {
    type Inner = MoveEnd<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MoveEnd<'a> {
    pub const VT_NORMAL_OFFSET: ::flatbuffers::VOffsetT = 4;
    pub const VT_STAND_OFFSET: ::flatbuffers::VOffsetT = 6;
    pub const VT_KNEEL_OFFSET: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn normal_offset(&self) -> Option<Motion<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<Motion>>(MoveEnd::VT_NORMAL_OFFSET, None)
        }
    }
    #[inline]
    pub fn stand_offset(&self) -> Option<Motion<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<Motion>>(MoveEnd::VT_STAND_OFFSET, None)
        }
    }
    #[inline]
    pub fn kneel_offset(&self) -> Option<Motion<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<Motion>>(MoveEnd::VT_KNEEL_OFFSET, None)
        }
    }
}

impl ::flatbuffers::Verifiable for MoveEnd<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<::flatbuffers::ForwardsUOffset<Motion>>(
                "normal_offset",
                Self::VT_NORMAL_OFFSET,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<Motion>>(
                "stand_offset",
                Self::VT_STAND_OFFSET,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<Motion>>(
                "kneel_offset",
                Self::VT_KNEEL_OFFSET,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MoveEnd<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MoveEnd", 3)?;
        if let Some(f) = self.normal_offset() {
            s.serialize_field("normal_offset", &f)?;
        } else {
            s.skip_field("normal_offset")?;
        }
        if let Some(f) = self.stand_offset() {
            s.serialize_field("stand_offset", &f)?;
        } else {
            s.skip_field("stand_offset")?;
        }
        if let Some(f) = self.kneel_offset() {
            s.serialize_field("kneel_offset", &f)?;
        } else {
            s.skip_field("kneel_offset")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MoveEnd<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MoveEnd");
        ds.field("normal_offset", &self.normal_offset());
        ds.field("stand_offset", &self.stand_offset());
        ds.field("kneel_offset", &self.kneel_offset());
        ds.finish()
    }
}
