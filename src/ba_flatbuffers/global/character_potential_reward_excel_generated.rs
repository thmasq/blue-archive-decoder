extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ParcelType, PotentialStatBonusRateType};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterPotentialRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterPotentialRewardExcel<'a> {
    type Inner = CharacterPotentialRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterPotentialRewardExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_REQUIRE_POTENTIAL_STAT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_REQUIRE_POTENTIAL_STAT_LEVEL: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialRewardExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn require_potential_stat_type(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, PotentialStatBonusRateType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, PotentialStatBonusRateType>,
            >>(
                CharacterPotentialRewardExcel::VT_REQUIRE_POTENTIAL_STAT_TYPE,
                None,
            )
        }
    }
    #[inline]
    pub fn require_potential_stat_level(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CharacterPotentialRewardExcel::VT_REQUIRE_POTENTIAL_STAT_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    CharacterPotentialRewardExcel::VT_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterPotentialRewardExcel::VT_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_amount(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CharacterPotentialRewardExcel::VT_REWARD_AMOUNT, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterPotentialRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, PotentialStatBonusRateType>>>("require_potential_stat_type", Self::VT_REQUIRE_POTENTIAL_STAT_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("require_potential_stat_level", Self::VT_REQUIRE_POTENTIAL_STAT_LEVEL, false)?
     .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
     .visit_field::<i64>("reward_id", Self::VT_REWARD_ID, false)?
     .visit_field::<i32>("reward_amount", Self::VT_REWARD_AMOUNT, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for CharacterPotentialRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterPotentialRewardExcel", 6)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.require_potential_stat_type() {
            s.serialize_field("require_potential_stat_type", &f)?;
        } else {
            s.skip_field("require_potential_stat_type")?;
        }
        if let Some(f) = self.require_potential_stat_level() {
            s.serialize_field("require_potential_stat_level", &f)?;
        } else {
            s.skip_field("require_potential_stat_level")?;
        }
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_id", &self.reward_id())?;
        s.serialize_field("reward_amount", &self.reward_amount())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterPotentialRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterPotentialRewardExcel");
        ds.field("id", &self.id());
        ds.field(
            "require_potential_stat_type",
            &self.require_potential_stat_type(),
        );
        ds.field(
            "require_potential_stat_level",
            &self.require_potential_stat_level(),
        );
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_id", &self.reward_id());
        ds.field("reward_amount", &self.reward_amount());
        ds.finish()
    }
}
