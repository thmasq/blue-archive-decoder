extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{GetStickerConditionType, StickerCheckPassType, StickerGetConditionType, Tag};

#[derive(Copy, Clone, PartialEq)]
pub struct StickerPageContentExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for StickerPageContentExcel<'a> {
    type Inner = StickerPageContentExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> StickerPageContentExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_STICKER_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_STICKER_PAGE_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_STICKER_SLOT: ::flatbuffers::VOffsetT = 10;
    pub const VT_STICKER_GET_CONDITION_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_STICKER_CHECK_PASS_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_GET_STICKER_CONDITION_TYPE: ::flatbuffers::VOffsetT = 16;
    pub const VT_STICKER_GET_CONDITION_COUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_STICKER_GET_CONDITION_PARAMETER: ::flatbuffers::VOffsetT = 20;
    pub const VT_STICKER_GET_CONDITION_PARAMETER_TAG: ::flatbuffers::VOffsetT = 22;
    pub const VT_PACKED_STICKER_ICON_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 24;
    pub const VT_PACKED_STICKER_ICON_PATH: ::flatbuffers::VOffsetT = 26;
    pub const VT_ICON_PATH: ::flatbuffers::VOffsetT = 28;
    pub const VT_STICKER_DETAIL_PATH: ::flatbuffers::VOffsetT = 30;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerPageContentExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerPageContentExcel::VT_STICKER_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_page_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerPageContentExcel::VT_STICKER_PAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_slot(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StickerPageContentExcel::VT_STICKER_SLOT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_get_condition_type(&self) -> StickerGetConditionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StickerGetConditionType>(
                    StickerPageContentExcel::VT_STICKER_GET_CONDITION_TYPE,
                    Some(StickerGetConditionType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_check_pass_type(&self) -> StickerCheckPassType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StickerCheckPassType>(
                    StickerPageContentExcel::VT_STICKER_CHECK_PASS_TYPE,
                    Some(StickerCheckPassType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn get_sticker_condition_type(&self) -> GetStickerConditionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<GetStickerConditionType>(
                    StickerPageContentExcel::VT_GET_STICKER_CONDITION_TYPE,
                    Some(GetStickerConditionType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_get_condition_count(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    StickerPageContentExcel::VT_STICKER_GET_CONDITION_COUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn sticker_get_condition_parameter(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    StickerPageContentExcel::VT_STICKER_GET_CONDITION_PARAMETER,
                    None,
                )
        }
    }
    #[inline]
    pub fn sticker_get_condition_parameter_tag(&self) -> Option<::flatbuffers::Vector<'a, Tag>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Tag>>>(
                    StickerPageContentExcel::VT_STICKER_GET_CONDITION_PARAMETER_TAG,
                    None,
                )
        }
    }
    #[inline]
    pub fn packed_sticker_icon_localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    StickerPageContentExcel::VT_PACKED_STICKER_ICON_LOCALIZE_ETC_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn packed_sticker_icon_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerPageContentExcel::VT_PACKED_STICKER_ICON_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn icon_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerPageContentExcel::VT_ICON_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn sticker_detail_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StickerPageContentExcel::VT_STICKER_DETAIL_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for StickerPageContentExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("sticker_group_id", Self::VT_STICKER_GROUP_ID, false)?
            .visit_field::<i64>("sticker_page_id", Self::VT_STICKER_PAGE_ID, false)?
            .visit_field::<i64>("sticker_slot", Self::VT_STICKER_SLOT, false)?
            .visit_field::<StickerGetConditionType>(
                "sticker_get_condition_type",
                Self::VT_STICKER_GET_CONDITION_TYPE,
                false,
            )?
            .visit_field::<StickerCheckPassType>(
                "sticker_check_pass_type",
                Self::VT_STICKER_CHECK_PASS_TYPE,
                false,
            )?
            .visit_field::<GetStickerConditionType>(
                "get_sticker_condition_type",
                Self::VT_GET_STICKER_CONDITION_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "sticker_get_condition_count",
                Self::VT_STICKER_GET_CONDITION_COUNT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "sticker_get_condition_parameter",
                Self::VT_STICKER_GET_CONDITION_PARAMETER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, Tag>>>(
                "sticker_get_condition_parameter_tag",
                Self::VT_STICKER_GET_CONDITION_PARAMETER_TAG,
                false,
            )?
            .visit_field::<u32>(
                "packed_sticker_icon_localize_etc_id",
                Self::VT_PACKED_STICKER_ICON_LOCALIZE_ETC_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "packed_sticker_icon_path",
                Self::VT_PACKED_STICKER_ICON_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "icon_path",
                Self::VT_ICON_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sticker_detail_path",
                Self::VT_STICKER_DETAIL_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for StickerPageContentExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StickerPageContentExcel", 14)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("sticker_group_id", &self.sticker_group_id())?;
        s.serialize_field("sticker_page_id", &self.sticker_page_id())?;
        s.serialize_field("sticker_slot", &self.sticker_slot())?;
        s.serialize_field(
            "sticker_get_condition_type",
            &self.sticker_get_condition_type(),
        )?;
        s.serialize_field("sticker_check_pass_type", &self.sticker_check_pass_type())?;
        s.serialize_field(
            "get_sticker_condition_type",
            &self.get_sticker_condition_type(),
        )?;
        s.serialize_field(
            "sticker_get_condition_count",
            &self.sticker_get_condition_count(),
        )?;
        if let Some(f) = self.sticker_get_condition_parameter() {
            s.serialize_field("sticker_get_condition_parameter", &f)?;
        } else {
            s.skip_field("sticker_get_condition_parameter")?;
        }
        if let Some(f) = self.sticker_get_condition_parameter_tag() {
            s.serialize_field("sticker_get_condition_parameter_tag", &f)?;
        } else {
            s.skip_field("sticker_get_condition_parameter_tag")?;
        }
        s.serialize_field(
            "packed_sticker_icon_localize_etc_id",
            &self.packed_sticker_icon_localize_etc_id(),
        )?;
        if let Some(f) = self.packed_sticker_icon_path() {
            s.serialize_field("packed_sticker_icon_path", &f)?;
        } else {
            s.skip_field("packed_sticker_icon_path")?;
        }
        if let Some(f) = self.icon_path() {
            s.serialize_field("icon_path", &f)?;
        } else {
            s.skip_field("icon_path")?;
        }
        if let Some(f) = self.sticker_detail_path() {
            s.serialize_field("sticker_detail_path", &f)?;
        } else {
            s.skip_field("sticker_detail_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for StickerPageContentExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("StickerPageContentExcel");
        ds.field("id", &self.id());
        ds.field("sticker_group_id", &self.sticker_group_id());
        ds.field("sticker_page_id", &self.sticker_page_id());
        ds.field("sticker_slot", &self.sticker_slot());
        ds.field(
            "sticker_get_condition_type",
            &self.sticker_get_condition_type(),
        );
        ds.field("sticker_check_pass_type", &self.sticker_check_pass_type());
        ds.field(
            "get_sticker_condition_type",
            &self.get_sticker_condition_type(),
        );
        ds.field(
            "sticker_get_condition_count",
            &self.sticker_get_condition_count(),
        );
        ds.field(
            "sticker_get_condition_parameter",
            &self.sticker_get_condition_parameter(),
        );
        ds.field(
            "sticker_get_condition_parameter_tag",
            &self.sticker_get_condition_parameter_tag(),
        );
        ds.field(
            "packed_sticker_icon_localize_etc_id",
            &self.packed_sticker_icon_localize_etc_id(),
        );
        ds.field("packed_sticker_icon_path", &self.packed_sticker_icon_path());
        ds.field("icon_path", &self.icon_path());
        ds.field("sticker_detail_path", &self.sticker_detail_path());
        ds.finish()
    }
}
