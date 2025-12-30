extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{EquipmentOptionType, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct FavorLevelRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for FavorLevelRewardExcel<'a> {
    type Inner = FavorLevelRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> FavorLevelRewardExcel<'a> {
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_FAVOR_LEVEL: ::flatbuffers::VOffsetT = 6;
    pub const VT_STAT_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_STAT_VALUE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FavorLevelRewardExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn favor_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FavorLevelRewardExcel::VT_FAVOR_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stat_type(&self) -> Option<::flatbuffers::Vector<'a, EquipmentOptionType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, EquipmentOptionType>>>(FavorLevelRewardExcel::VT_STAT_TYPE, None)
        }
    }
    #[inline]
    pub fn stat_value(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FavorLevelRewardExcel::VT_STAT_VALUE,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_parcel_type(&self) -> Option<::flatbuffers::Vector<'a, ParcelType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, ParcelType>>>(
                    FavorLevelRewardExcel::VT_REWARD_PARCEL_TYPE,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_parcel_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FavorLevelRewardExcel::VT_REWARD_PARCEL_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FavorLevelRewardExcel::VT_REWARD_AMOUNT,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for FavorLevelRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
     .visit_field::<i64>("favor_level", Self::VT_FAVOR_LEVEL, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, EquipmentOptionType>>>("stat_type", Self::VT_STAT_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("stat_value", Self::VT_STAT_VALUE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ParcelType>>>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("reward_amount", Self::VT_REWARD_AMOUNT, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for FavorLevelRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FavorLevelRewardExcel", 7)?;
        s.serialize_field("character_id", &self.character_id())?;
        s.serialize_field("favor_level", &self.favor_level())?;
        if let Some(f) = self.stat_type() {
            s.serialize_field("stat_type", &f)?;
        } else {
            s.skip_field("stat_type")?;
        }
        if let Some(f) = self.stat_value() {
            s.serialize_field("stat_value", &f)?;
        } else {
            s.skip_field("stat_value")?;
        }
        if let Some(f) = self.reward_parcel_type() {
            s.serialize_field("reward_parcel_type", &f)?;
        } else {
            s.skip_field("reward_parcel_type")?;
        }
        if let Some(f) = self.reward_parcel_id() {
            s.serialize_field("reward_parcel_id", &f)?;
        } else {
            s.skip_field("reward_parcel_id")?;
        }
        if let Some(f) = self.reward_amount() {
            s.serialize_field("reward_amount", &f)?;
        } else {
            s.skip_field("reward_amount")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for FavorLevelRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("FavorLevelRewardExcel");
        ds.field("character_id", &self.character_id());
        ds.field("favor_level", &self.favor_level());
        ds.field("stat_type", &self.stat_type());
        ds.field("stat_value", &self.stat_value());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_amount", &self.reward_amount());
        ds.finish()
    }
}
