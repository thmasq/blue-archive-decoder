extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
#[derive(Copy, Clone, PartialEq)]

pub struct ClanChattingEmojiExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ClanChattingEmojiExcel<'a> {
    type Inner = ClanChattingEmojiExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ClanChattingEmojiExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TAB_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_IMAGE_PATH_KR: ::flatbuffers::VOffsetT = 10;
    pub const VT_IMAGE_PATH_JP: ::flatbuffers::VOffsetT = 12;
    pub const VT_IMAGE_PATH_TH: ::flatbuffers::VOffsetT = 14;
    pub const VT_IMAGE_PATH_TW: ::flatbuffers::VOffsetT = 16;
    pub const VT_IMAGE_PATH_EN: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanChattingEmojiExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn tab_group_id(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ClanChattingEmojiExcel::VT_TAB_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn display_order(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ClanChattingEmojiExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn image_path_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ClanChattingEmojiExcel::VT_IMAGE_PATH_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ClanChattingEmojiExcel::VT_IMAGE_PATH_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ClanChattingEmojiExcel::VT_IMAGE_PATH_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ClanChattingEmojiExcel::VT_IMAGE_PATH_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ClanChattingEmojiExcel::VT_IMAGE_PATH_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ClanChattingEmojiExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i32>("tab_group_id", Self::VT_TAB_GROUP_ID, false)?
            .visit_field::<i32>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_kr",
                Self::VT_IMAGE_PATH_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_jp",
                Self::VT_IMAGE_PATH_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_th",
                Self::VT_IMAGE_PATH_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_tw",
                Self::VT_IMAGE_PATH_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_en",
                Self::VT_IMAGE_PATH_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ClanChattingEmojiExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ClanChattingEmojiExcel", 8)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("tab_group_id", &self.tab_group_id())?;
        s.serialize_field("display_order", &self.display_order())?;
        if let Some(f) = self.image_path_kr() {
            s.serialize_field("image_path_kr", &f)?;
        } else {
            s.skip_field("image_path_kr")?;
        }
        if let Some(f) = self.image_path_jp() {
            s.serialize_field("image_path_jp", &f)?;
        } else {
            s.skip_field("image_path_jp")?;
        }
        if let Some(f) = self.image_path_th() {
            s.serialize_field("image_path_th", &f)?;
        } else {
            s.skip_field("image_path_th")?;
        }
        if let Some(f) = self.image_path_tw() {
            s.serialize_field("image_path_tw", &f)?;
        } else {
            s.skip_field("image_path_tw")?;
        }
        if let Some(f) = self.image_path_en() {
            s.serialize_field("image_path_en", &f)?;
        } else {
            s.skip_field("image_path_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ClanChattingEmojiExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ClanChattingEmojiExcel");
        ds.field("id", &self.id());
        ds.field("tab_group_id", &self.tab_group_id());
        ds.field("display_order", &self.display_order());
        ds.field("image_path_kr", &self.image_path_kr());
        ds.field("image_path_jp", &self.image_path_jp());
        ds.field("image_path_th", &self.image_path_th());
        ds.field("image_path_tw", &self.image_path_tw());
        ds.field("image_path_en", &self.image_path_en());
        ds.finish()
    }
}
