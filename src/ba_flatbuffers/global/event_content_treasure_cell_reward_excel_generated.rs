extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentTreasureCellRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentTreasureCellRewardExcel<'a> {
    type Inner = EventContentTreasureCellRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentTreasureCellRewardExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCALIZE_CODE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentTreasureCellRewardExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_code_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureCellRewardExcel::VT_LOCALIZE_CODE_ID,
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
                    EventContentTreasureCellRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                    EventContentTreasureCellRewardExcel::VT_REWARD_PARCEL_ID,
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
                    EventContentTreasureCellRewardExcel::VT_REWARD_PARCEL_AMOUNT,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentTreasureCellRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_code_id",
                Self::VT_LOCALIZE_CODE_ID,
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

impl Serialize for EventContentTreasureCellRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentTreasureCellRewardExcel", 5)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.localize_code_id() {
            s.serialize_field("localize_code_id", &f)?;
        } else {
            s.skip_field("localize_code_id")?;
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
        if let Some(f) = self.reward_parcel_amount() {
            s.serialize_field("reward_parcel_amount", &f)?;
        } else {
            s.skip_field("reward_parcel_amount")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentTreasureCellRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentTreasureCellRewardExcel");
        ds.field("id", &self.id());
        ds.field("localize_code_id", &self.localize_code_id());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.finish()
    }
}
