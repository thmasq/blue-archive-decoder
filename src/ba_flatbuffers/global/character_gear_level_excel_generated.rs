extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterGearLevelExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterGearLevelExcel<'a> {
    type Inner = CharacterGearLevelExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterGearLevelExcel<'a> {
    pub const VT_LEVEL: ::flatbuffers::VOffsetT = 4;
    pub const VT_TIER_LEVEL_EXP: ::flatbuffers::VOffsetT = 6;
    pub const VT_TOTAL_EXP: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CharacterGearLevelExcel::VT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn tier_level_exp(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CharacterGearLevelExcel::VT_TIER_LEVEL_EXP,
                    None,
                )
        }
    }
    #[inline]
    pub fn total_exp(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CharacterGearLevelExcel::VT_TOTAL_EXP,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterGearLevelExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i32>("level", Self::VT_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "tier_level_exp",
                Self::VT_TIER_LEVEL_EXP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "total_exp",
                Self::VT_TOTAL_EXP,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterGearLevelExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterGearLevelExcel", 3)?;
        s.serialize_field("level", &self.level())?;
        if let Some(f) = self.tier_level_exp() {
            s.serialize_field("tier_level_exp", &f)?;
        } else {
            s.skip_field("tier_level_exp")?;
        }
        if let Some(f) = self.total_exp() {
            s.serialize_field("total_exp", &f)?;
        } else {
            s.skip_field("total_exp")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterGearLevelExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterGearLevelExcel");
        ds.field("level", &self.level());
        ds.field("tier_level_exp", &self.tier_level_exp());
        ds.field("total_exp", &self.total_exp());
        ds.finish()
    }
}
