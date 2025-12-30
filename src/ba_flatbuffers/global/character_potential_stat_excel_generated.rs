extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterPotentialStatExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterPotentialStatExcel<'a> {
    type Inner = CharacterPotentialStatExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterPotentialStatExcel<'a> {
    pub const VT_POTENTIAL_STAT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_POTENTIAL_LEVEL: ::flatbuffers::VOffsetT = 6;
    pub const VT_RECIPE_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_STAT_BONUS_RATE: ::flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn potential_stat_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    CharacterPotentialStatExcel::VT_POTENTIAL_STAT_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn potential_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CharacterPotentialStatExcel::VT_POTENTIAL_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn recipe_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialStatExcel::VT_RECIPE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stat_bonus_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialStatExcel::VT_STAT_BONUS_RATE, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterPotentialStatExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "potential_stat_group_id",
                Self::VT_POTENTIAL_STAT_GROUP_ID,
                false,
            )?
            .visit_field::<i32>("potential_level", Self::VT_POTENTIAL_LEVEL, false)?
            .visit_field::<i64>("recipe_id", Self::VT_RECIPE_ID, false)?
            .visit_field::<i64>("stat_bonus_rate", Self::VT_STAT_BONUS_RATE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterPotentialStatExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterPotentialStatExcel", 4)?;
        s.serialize_field("potential_stat_group_id", &self.potential_stat_group_id())?;
        s.serialize_field("potential_level", &self.potential_level())?;
        s.serialize_field("recipe_id", &self.recipe_id())?;
        s.serialize_field("stat_bonus_rate", &self.stat_bonus_rate())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterPotentialStatExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterPotentialStatExcel");
        ds.field("potential_stat_group_id", &self.potential_stat_group_id());
        ds.field("potential_level", &self.potential_level());
        ds.field("recipe_id", &self.recipe_id());
        ds.field("stat_bonus_rate", &self.stat_bonus_rate());
        ds.finish()
    }
}
