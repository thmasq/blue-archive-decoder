extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct MultiFloorRaidRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MultiFloorRaidRewardExcel<'a> {
    type Inner = MultiFloorRaidRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MultiFloorRaidRewardExcel<'a> {
    pub const VT_REWARD_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CLEAR_STAGE_REWARD_PROB: ::flatbuffers::VOffsetT = 6;
    pub const VT_CLEAR_STAGE_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_CLEAR_STAGE_REWARD_PARCEL_UNIQUE_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_CLEAR_STAGE_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn reward_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidRewardExcel::VT_REWARD_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn clear_stage_reward_prob(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MultiFloorRaidRewardExcel::VT_CLEAR_STAGE_REWARD_PROB,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn clear_stage_reward_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MultiFloorRaidRewardExcel::VT_CLEAR_STAGE_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn clear_stage_reward_parcel_unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MultiFloorRaidRewardExcel::VT_CLEAR_STAGE_REWARD_PARCEL_UNIQUE_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn clear_stage_reward_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MultiFloorRaidRewardExcel::VT_CLEAR_STAGE_REWARD_AMOUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MultiFloorRaidRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("reward_group_id", Self::VT_REWARD_GROUP_ID, false)?
            .visit_field::<i64>(
                "clear_stage_reward_prob",
                Self::VT_CLEAR_STAGE_REWARD_PROB,
                false,
            )?
            .visit_field::<ParcelType>(
                "clear_stage_reward_parcel_type",
                Self::VT_CLEAR_STAGE_REWARD_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "clear_stage_reward_parcel_unique_id",
                Self::VT_CLEAR_STAGE_REWARD_PARCEL_UNIQUE_ID,
                false,
            )?
            .visit_field::<i64>(
                "clear_stage_reward_amount",
                Self::VT_CLEAR_STAGE_REWARD_AMOUNT,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MultiFloorRaidRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MultiFloorRaidRewardExcel", 5)?;
        s.serialize_field("reward_group_id", &self.reward_group_id())?;
        s.serialize_field("clear_stage_reward_prob", &self.clear_stage_reward_prob())?;
        s.serialize_field(
            "clear_stage_reward_parcel_type",
            &self.clear_stage_reward_parcel_type(),
        )?;
        s.serialize_field(
            "clear_stage_reward_parcel_unique_id",
            &self.clear_stage_reward_parcel_unique_id(),
        )?;
        s.serialize_field(
            "clear_stage_reward_amount",
            &self.clear_stage_reward_amount(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for MultiFloorRaidRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MultiFloorRaidRewardExcel");
        ds.field("reward_group_id", &self.reward_group_id());
        ds.field("clear_stage_reward_prob", &self.clear_stage_reward_prob());
        ds.field(
            "clear_stage_reward_parcel_type",
            &self.clear_stage_reward_parcel_type(),
        );
        ds.field(
            "clear_stage_reward_parcel_unique_id",
            &self.clear_stage_reward_parcel_unique_id(),
        );
        ds.field(
            "clear_stage_reward_amount",
            &self.clear_stage_reward_amount(),
        );
        ds.finish()
    }
}
