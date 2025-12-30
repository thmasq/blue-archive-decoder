extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameRoadPuzzleRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameRoadPuzzleRewardExcel<'a> {
    type Inner = MiniGameRoadPuzzleRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameRoadPuzzleRewardExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleRewardExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleRewardExcel::VT_UNIQUE_ID, Some(0))
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
                    MiniGameRoadPuzzleRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                    MiniGameRoadPuzzleRewardExcel::VT_REWARD_PARCEL_ID,
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
                    MiniGameRoadPuzzleRewardExcel::VT_REWARD_PARCEL_AMOUNT,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameRoadPuzzleRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
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

impl Serialize for MiniGameRoadPuzzleRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameRoadPuzzleRewardExcel", 5)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("unique_id", &self.unique_id())?;
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

impl ::core::fmt::Debug for MiniGameRoadPuzzleRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameRoadPuzzleRewardExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("unique_id", &self.unique_id());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.finish()
    }
}
