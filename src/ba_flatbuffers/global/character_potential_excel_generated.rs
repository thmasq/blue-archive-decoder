extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::PotentialStatBonusRateType;

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterPotentialExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterPotentialExcel<'a> {
    type Inner = CharacterPotentialExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterPotentialExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_POTENTIAL_STAT_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_POTENTIAL_STAT_BONUS_RATE_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_IS_UNNECESSARY_STAT: ::flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn potential_stat_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialExcel::VT_POTENTIAL_STAT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn potential_stat_bonus_rate_type(&self) -> PotentialStatBonusRateType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<PotentialStatBonusRateType>(
                    CharacterPotentialExcel::VT_POTENTIAL_STAT_BONUS_RATE_TYPE,
                    Some(PotentialStatBonusRateType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn is_unnecessary_stat(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterPotentialExcel::VT_IS_UNNECESSARY_STAT, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterPotentialExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>(
                "potential_stat_group_id",
                Self::VT_POTENTIAL_STAT_GROUP_ID,
                false,
            )?
            .visit_field::<PotentialStatBonusRateType>(
                "potential_stat_bonus_rate_type",
                Self::VT_POTENTIAL_STAT_BONUS_RATE_TYPE,
                false,
            )?
            .visit_field::<bool>("is_unnecessary_stat", Self::VT_IS_UNNECESSARY_STAT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterPotentialExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterPotentialExcel", 4)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("potential_stat_group_id", &self.potential_stat_group_id())?;
        s.serialize_field(
            "potential_stat_bonus_rate_type",
            &self.potential_stat_bonus_rate_type(),
        )?;
        s.serialize_field("is_unnecessary_stat", &self.is_unnecessary_stat())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterPotentialExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterPotentialExcel");
        ds.field("id", &self.id());
        ds.field("potential_stat_group_id", &self.potential_stat_group_id());
        ds.field(
            "potential_stat_bonus_rate_type",
            &self.potential_stat_bonus_rate_type(),
        );
        ds.field("is_unnecessary_stat", &self.is_unnecessary_stat());
        ds.finish()
    }
}
