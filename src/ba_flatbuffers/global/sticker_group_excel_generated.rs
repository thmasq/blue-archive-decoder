extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct StickerGroupExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for StickerGroupExcel<'a> {
    type Inner = StickerGroupExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> StickerGroupExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LAYOUT: ::flatbuffers::VOffsetT = 6;
    pub const VT_UNIQUE_LAYOUT_PATH: ::flatbuffers::VOffsetT = 8;
    pub const VT_STICKER_GROUP_ICONPATH: ::flatbuffers::VOffsetT = 10;
    pub const VT_PAGE_COMPLETE_SLOT: ::flatbuffers::VOffsetT = 12;
    pub const VT_PAGE_COMPLETE_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_PAGE_COMPLETE_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_PAGE_COMPLETE_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_LOCALIZE_TITLE: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOCALIZE_DESCRIPTION: ::flatbuffers::VOffsetT = 22;
    pub const VT_STICKER_GROUP_COVERPATH: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerGroupExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn layout(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(StickerGroupExcel::VT_LAYOUT, None)
        }
    }
    #[inline]
    pub fn unique_layout_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerGroupExcel::VT_UNIQUE_LAYOUT_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn sticker_group_iconpath(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerGroupExcel::VT_STICKER_GROUP_ICONPATH,
                None,
            )
        }
    }
    #[inline]
    pub fn page_complete_slot(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerGroupExcel::VT_PAGE_COMPLETE_SLOT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn page_complete_reward_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    StickerGroupExcel::VT_PAGE_COMPLETE_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn page_complete_reward_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    StickerGroupExcel::VT_PAGE_COMPLETE_REWARD_PARCEL_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn page_complete_reward_amount(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(StickerGroupExcel::VT_PAGE_COMPLETE_REWARD_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_title(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(StickerGroupExcel::VT_LOCALIZE_TITLE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_description(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(StickerGroupExcel::VT_LOCALIZE_DESCRIPTION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_group_coverpath(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerGroupExcel::VT_STICKER_GROUP_COVERPATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for StickerGroupExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("layout", Self::VT_LAYOUT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "unique_layout_path",
                Self::VT_UNIQUE_LAYOUT_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sticker_group_iconpath",
                Self::VT_STICKER_GROUP_ICONPATH,
                false,
            )?
            .visit_field::<i64>("page_complete_slot", Self::VT_PAGE_COMPLETE_SLOT, false)?
            .visit_field::<ParcelType>(
                "page_complete_reward_parcel_type",
                Self::VT_PAGE_COMPLETE_REWARD_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "page_complete_reward_parcel_id",
                Self::VT_PAGE_COMPLETE_REWARD_PARCEL_ID,
                false,
            )?
            .visit_field::<i32>(
                "page_complete_reward_amount",
                Self::VT_PAGE_COMPLETE_REWARD_AMOUNT,
                false,
            )?
            .visit_field::<u32>("localize_title", Self::VT_LOCALIZE_TITLE, false)?
            .visit_field::<u32>("localize_description", Self::VT_LOCALIZE_DESCRIPTION, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sticker_group_coverpath",
                Self::VT_STICKER_GROUP_COVERPATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for StickerGroupExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StickerGroupExcel", 11)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.layout() {
            s.serialize_field("layout", &f)?;
        } else {
            s.skip_field("layout")?;
        }
        if let Some(f) = self.unique_layout_path() {
            s.serialize_field("unique_layout_path", &f)?;
        } else {
            s.skip_field("unique_layout_path")?;
        }
        if let Some(f) = self.sticker_group_iconpath() {
            s.serialize_field("sticker_group_iconpath", &f)?;
        } else {
            s.skip_field("sticker_group_iconpath")?;
        }
        s.serialize_field("page_complete_slot", &self.page_complete_slot())?;
        s.serialize_field(
            "page_complete_reward_parcel_type",
            &self.page_complete_reward_parcel_type(),
        )?;
        s.serialize_field(
            "page_complete_reward_parcel_id",
            &self.page_complete_reward_parcel_id(),
        )?;
        s.serialize_field(
            "page_complete_reward_amount",
            &self.page_complete_reward_amount(),
        )?;
        s.serialize_field("localize_title", &self.localize_title())?;
        s.serialize_field("localize_description", &self.localize_description())?;
        if let Some(f) = self.sticker_group_coverpath() {
            s.serialize_field("sticker_group_coverpath", &f)?;
        } else {
            s.skip_field("sticker_group_coverpath")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for StickerGroupExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("StickerGroupExcel");
        ds.field("id", &self.id());
        ds.field("layout", &self.layout());
        ds.field("unique_layout_path", &self.unique_layout_path());
        ds.field("sticker_group_iconpath", &self.sticker_group_iconpath());
        ds.field("page_complete_slot", &self.page_complete_slot());
        ds.field(
            "page_complete_reward_parcel_type",
            &self.page_complete_reward_parcel_type(),
        );
        ds.field(
            "page_complete_reward_parcel_id",
            &self.page_complete_reward_parcel_id(),
        );
        ds.field(
            "page_complete_reward_amount",
            &self.page_complete_reward_amount(),
        );
        ds.field("localize_title", &self.localize_title());
        ds.field("localize_description", &self.localize_description());
        ds.field("sticker_group_coverpath", &self.sticker_group_coverpath());
        ds.finish()
    }
}
