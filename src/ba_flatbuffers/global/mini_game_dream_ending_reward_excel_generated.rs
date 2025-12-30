extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{DreamMakerEndingRewardType, DreamMakerEndingType, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamEndingRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamEndingRewardExcel<'a> {
    type Inner = MiniGameDreamEndingRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamEndingRewardExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ENDING_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_DREAM_MAKER_ENDING_REWARD_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_DREAM_MAKER_ENDING_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamEndingRewardExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ending_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamEndingRewardExcel::VT_ENDING_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MiniGameDreamEndingRewardExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_ending_reward_type(&self) -> DreamMakerEndingRewardType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerEndingRewardType>(
                    MiniGameDreamEndingRewardExcel::VT_DREAM_MAKER_ENDING_REWARD_TYPE,
                    Some(DreamMakerEndingRewardType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_ending_type(&self) -> DreamMakerEndingType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerEndingType>(
                    MiniGameDreamEndingRewardExcel::VT_DREAM_MAKER_ENDING_TYPE,
                    Some(DreamMakerEndingType::None),
                )
                .unwrap()
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
                    MiniGameDreamEndingRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                    MiniGameDreamEndingRewardExcel::VT_REWARD_PARCEL_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_parcel_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDreamEndingRewardExcel::VT_REWARD_PARCEL_AMOUNT,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamEndingRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("ending_id", Self::VT_ENDING_ID, false)?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<DreamMakerEndingRewardType>(
                "dream_maker_ending_reward_type",
                Self::VT_DREAM_MAKER_ENDING_REWARD_TYPE,
                false,
            )?
            .visit_field::<DreamMakerEndingType>(
                "dream_maker_ending_type",
                Self::VT_DREAM_MAKER_ENDING_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ParcelType>>>(
                "reward_parcel_type",
                Self::VT_REWARD_PARCEL_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "reward_parcel_id",
                Self::VT_REWARD_PARCEL_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "reward_parcel_amount",
                Self::VT_REWARD_PARCEL_AMOUNT,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamEndingRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamEndingRewardExcel", 8)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("ending_id", &self.ending_id())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        s.serialize_field(
            "dream_maker_ending_reward_type",
            &self.dream_maker_ending_reward_type(),
        )?;
        s.serialize_field("dream_maker_ending_type", &self.dream_maker_ending_type())?;
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
        if let Some(f) = self.reward_parcel_amount() {
            s.serialize_field("reward_parcel_amount", &f)?;
        } else {
            s.skip_field("reward_parcel_amount")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamEndingRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamEndingRewardExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("ending_id", &self.ending_id());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field(
            "dream_maker_ending_reward_type",
            &self.dream_maker_ending_reward_type(),
        );
        ds.field("dream_maker_ending_type", &self.dream_maker_ending_type());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.finish()
    }
}
