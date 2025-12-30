extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{Motion, MoveEnd};

#[allow(dead_code)]
pub struct Form<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for Form<'a> {
    type Inner = Form<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

#[allow(dead_code)]
impl<'a> Form<'a> {
    pub const VT_MOVE_END_OFFSET: ::flatbuffers::VOffsetT = 4;
    pub const VT_PUBLIC_SKILL_OFFSET: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn move_end_offset(&self) -> Option<MoveEnd<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<MoveEnd>>(Form::VT_MOVE_END_OFFSET, None)
        }
    }
    #[inline]
    pub fn public_skill_offset(&self) -> Option<Motion<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<Motion>>(Form::VT_PUBLIC_SKILL_OFFSET, None)
        }
    }
}

impl ::flatbuffers::Verifiable for Form<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<::flatbuffers::ForwardsUOffset<MoveEnd>>(
                "move_end_offset",
                Self::VT_MOVE_END_OFFSET,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<Motion>>(
                "public_skill_offset",
                Self::VT_PUBLIC_SKILL_OFFSET,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for Form<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Form", 2)?;
        if let Some(f) = self.move_end_offset() {
            s.serialize_field("move_end_offset", &f)?;
        } else {
            s.skip_field("move_end_offset")?;
        }
        if let Some(f) = self.public_skill_offset() {
            s.serialize_field("public_skill_offset", &f)?;
        } else {
            s.skip_field("public_skill_offset")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for Form<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("Form");
        ds.field("move_end_offset", &self.move_end_offset());
        ds.field("public_skill_offset", &self.public_skill_offset());
        ds.finish()
    }
}
