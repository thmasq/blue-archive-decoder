extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ClanRewardType, EchelonType, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct ClanRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ClanRewardExcel<'a> {
    type Inner = ClanRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ClanRewardExcel<'a> {
    pub const VT_CLAN_REWARD_TYPE: ::flatbuffers::VOffsetT = 4;
    pub const VT_ECHELON_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn clan_reward_type(&self) -> ClanRewardType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ClanRewardType>(
                    ClanRewardExcel::VT_CLAN_REWARD_TYPE,
                    Some(ClanRewardType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn echelon_type(&self) -> EchelonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EchelonType>(ClanRewardExcel::VT_ECHELON_TYPE, Some(EchelonType::None))
                .unwrap()
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
                    ClanRewardExcel::VT_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanRewardExcel::VT_REWARD_PARCEL_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanRewardExcel::VT_REWARD_PARCEL_AMOUNT, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ClanRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<ClanRewardType>("clan_reward_type", Self::VT_CLAN_REWARD_TYPE, false)?
            .visit_field::<EchelonType>("echelon_type", Self::VT_ECHELON_TYPE, false)?
            .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
            .visit_field::<i64>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
            .visit_field::<i64>("reward_parcel_amount", Self::VT_REWARD_PARCEL_AMOUNT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ClanRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ClanRewardExcel", 5)?;
        s.serialize_field("clan_reward_type", &self.clan_reward_type())?;
        s.serialize_field("echelon_type", &self.echelon_type())?;
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_parcel_id", &self.reward_parcel_id())?;
        s.serialize_field("reward_parcel_amount", &self.reward_parcel_amount())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ClanRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ClanRewardExcel");
        ds.field("clan_reward_type", &self.clan_reward_type());
        ds.field("echelon_type", &self.echelon_type());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.finish()
    }
}
