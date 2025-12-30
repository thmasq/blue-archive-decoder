extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ParcelType, RewardTag};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioModeRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioModeRewardExcel<'a> {
    type Inner = ScenarioModeRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioModeRewardExcel<'a> {
    pub const VT_SCENARIO_MODE_REWARD_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_REWARD_TAG: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_PROB: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 14;
    pub const VT_IS_DISPLAYED: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn scenario_mode_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeRewardExcel::VT_SCENARIO_MODE_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_tag(&self) -> RewardTag {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<RewardTag>(
                    ScenarioModeRewardExcel::VT_REWARD_TAG,
                    Some(RewardTag::Default),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_prob(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioModeRewardExcel::VT_REWARD_PROB, Some(0))
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
                    ScenarioModeRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                .get::<i64>(ScenarioModeRewardExcel::VT_REWARD_PARCEL_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_amount(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioModeRewardExcel::VT_REWARD_PARCEL_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_displayed(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeRewardExcel::VT_IS_DISPLAYED, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioModeRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "scenario_mode_reward_id",
                Self::VT_SCENARIO_MODE_REWARD_ID,
                false,
            )?
            .visit_field::<RewardTag>("reward_tag", Self::VT_REWARD_TAG, false)?
            .visit_field::<i32>("reward_prob", Self::VT_REWARD_PROB, false)?
            .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
            .visit_field::<i64>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
            .visit_field::<i32>("reward_parcel_amount", Self::VT_REWARD_PARCEL_AMOUNT, false)?
            .visit_field::<bool>("is_displayed", Self::VT_IS_DISPLAYED, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioModeRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioModeRewardExcel", 7)?;
        s.serialize_field("scenario_mode_reward_id", &self.scenario_mode_reward_id())?;
        s.serialize_field("reward_tag", &self.reward_tag())?;
        s.serialize_field("reward_prob", &self.reward_prob())?;
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_parcel_id", &self.reward_parcel_id())?;
        s.serialize_field("reward_parcel_amount", &self.reward_parcel_amount())?;
        s.serialize_field("is_displayed", &self.is_displayed())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioModeRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioModeRewardExcel");
        ds.field("scenario_mode_reward_id", &self.scenario_mode_reward_id());
        ds.field("reward_tag", &self.reward_tag());
        ds.field("reward_prob", &self.reward_prob());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.field("is_displayed", &self.is_displayed());
        ds.finish()
    }
}
