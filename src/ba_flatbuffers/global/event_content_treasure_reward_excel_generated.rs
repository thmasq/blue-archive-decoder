extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentTreasureRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentTreasureRewardExcel<'a> {
    type Inner = EventContentTreasureRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentTreasureRewardExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCALIZE_CODE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_CELL_UNDER_IMAGE_WIDTH: ::flatbuffers::VOffsetT = 8;
    pub const VT_CELL_UNDER_IMAGE_HEIGHT: ::flatbuffers::VOffsetT = 10;
    pub const VT_HIDDEN_IMAGE: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_CELL_UNDER_IMAGE_PATH: ::flatbuffers::VOffsetT = 20;
    pub const VT_TREASURE_SMALL_IMAGE_PATH: ::flatbuffers::VOffsetT = 22;
    pub const VT_TREASURE_SIZE_ICON_PATH: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentTreasureRewardExcel::VT_ID, Some(0))
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
                EventContentTreasureRewardExcel::VT_LOCALIZE_CODE_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn cell_under_image_width(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    EventContentTreasureRewardExcel::VT_CELL_UNDER_IMAGE_WIDTH,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn cell_under_image_height(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    EventContentTreasureRewardExcel::VT_CELL_UNDER_IMAGE_HEIGHT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn hidden_image(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    EventContentTreasureRewardExcel::VT_HIDDEN_IMAGE,
                    Some(false),
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
                    EventContentTreasureRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                    EventContentTreasureRewardExcel::VT_REWARD_PARCEL_ID,
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
                    EventContentTreasureRewardExcel::VT_REWARD_PARCEL_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn cell_under_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureRewardExcel::VT_CELL_UNDER_IMAGE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn treasure_small_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureRewardExcel::VT_TREASURE_SMALL_IMAGE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn treasure_size_icon_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureRewardExcel::VT_TREASURE_SIZE_ICON_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentTreasureRewardExcel<'_> {
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
            .visit_field::<i32>(
                "cell_under_image_width",
                Self::VT_CELL_UNDER_IMAGE_WIDTH,
                false,
            )?
            .visit_field::<i32>(
                "cell_under_image_height",
                Self::VT_CELL_UNDER_IMAGE_HEIGHT,
                false,
            )?
            .visit_field::<bool>("hidden_image", Self::VT_HIDDEN_IMAGE, false)?
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
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "cell_under_image_path",
                Self::VT_CELL_UNDER_IMAGE_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "treasure_small_image_path",
                Self::VT_TREASURE_SMALL_IMAGE_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "treasure_size_icon_path",
                Self::VT_TREASURE_SIZE_ICON_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for EventContentTreasureRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentTreasureRewardExcel", 11)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.localize_code_id() {
            s.serialize_field("localize_code_id", &f)?;
        } else {
            s.skip_field("localize_code_id")?;
        }
        s.serialize_field("cell_under_image_width", &self.cell_under_image_width())?;
        s.serialize_field("cell_under_image_height", &self.cell_under_image_height())?;
        s.serialize_field("hidden_image", &self.hidden_image())?;
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
        if let Some(f) = self.cell_under_image_path() {
            s.serialize_field("cell_under_image_path", &f)?;
        } else {
            s.skip_field("cell_under_image_path")?;
        }
        if let Some(f) = self.treasure_small_image_path() {
            s.serialize_field("treasure_small_image_path", &f)?;
        } else {
            s.skip_field("treasure_small_image_path")?;
        }
        if let Some(f) = self.treasure_size_icon_path() {
            s.serialize_field("treasure_size_icon_path", &f)?;
        } else {
            s.skip_field("treasure_size_icon_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentTreasureRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentTreasureRewardExcel");
        ds.field("id", &self.id());
        ds.field("localize_code_id", &self.localize_code_id());
        ds.field("cell_under_image_width", &self.cell_under_image_width());
        ds.field("cell_under_image_height", &self.cell_under_image_height());
        ds.field("hidden_image", &self.hidden_image());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.field("cell_under_image_path", &self.cell_under_image_path());
        ds.field(
            "treasure_small_image_path",
            &self.treasure_small_image_path(),
        );
        ds.field("treasure_size_icon_path", &self.treasure_size_icon_path());
        ds.finish()
    }
}
