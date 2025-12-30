extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterDialogEmojiExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterDialogEmojiExcel<'a> {
    type Inner = CharacterDialogEmojiExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterDialogEmojiExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TARGET_INDEX: ::flatbuffers::VOffsetT = 6;
    pub const VT_DIALOG_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_DURATION: ::flatbuffers::VOffsetT = 10;
    pub const VT_DURATION_KR: ::flatbuffers::VOffsetT = 12;
    pub const VT_HIDE_UI: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOCALIZE_KR: ::flatbuffers::VOffsetT = 16;
    pub const VT_LOCALIZE_JP: ::flatbuffers::VOffsetT = 18;
    pub const VT_LOCALIZE_TH: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOCALIZE_TW: ::flatbuffers::VOffsetT = 22;
    pub const VT_LOCALIZE_EN: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogEmojiExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn target_index(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CharacterDialogEmojiExcel::VT_TARGET_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dialog_type(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_DIALOG_TYPE,
                None,
            )
        }
    }
    #[inline]
    pub fn duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogEmojiExcel::VT_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn duration_kr(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogEmojiExcel::VT_DURATION_KR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn hide_ui(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogEmojiExcel::VT_HIDE_UI, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_LOCALIZE_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_LOCALIZE_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_LOCALIZE_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_LOCALIZE_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogEmojiExcel::VT_LOCALIZE_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterDialogEmojiExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<i32>("target_index", Self::VT_TARGET_INDEX, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "dialog_type",
                Self::VT_DIALOG_TYPE,
                false,
            )?
            .visit_field::<i64>("duration", Self::VT_DURATION, false)?
            .visit_field::<i64>("duration_kr", Self::VT_DURATION_KR, false)?
            .visit_field::<bool>("hide_ui", Self::VT_HIDE_UI, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_kr",
                Self::VT_LOCALIZE_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_jp",
                Self::VT_LOCALIZE_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_th",
                Self::VT_LOCALIZE_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_tw",
                Self::VT_LOCALIZE_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_en",
                Self::VT_LOCALIZE_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterDialogEmojiExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterDialogEmojiExcel", 11)?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("target_index", &self.target_index())?;
        if let Some(f) = self.dialog_type() {
            s.serialize_field("dialog_type", &f)?;
        } else {
            s.skip_field("dialog_type")?;
        }
        s.serialize_field("duration", &self.duration())?;
        s.serialize_field("duration_kr", &self.duration_kr())?;
        s.serialize_field("hide_ui", &self.hide_ui())?;
        if let Some(f) = self.localize_kr() {
            s.serialize_field("localize_kr", &f)?;
        } else {
            s.skip_field("localize_kr")?;
        }
        if let Some(f) = self.localize_jp() {
            s.serialize_field("localize_jp", &f)?;
        } else {
            s.skip_field("localize_jp")?;
        }
        if let Some(f) = self.localize_th() {
            s.serialize_field("localize_th", &f)?;
        } else {
            s.skip_field("localize_th")?;
        }
        if let Some(f) = self.localize_tw() {
            s.serialize_field("localize_tw", &f)?;
        } else {
            s.skip_field("localize_tw")?;
        }
        if let Some(f) = self.localize_en() {
            s.serialize_field("localize_en", &f)?;
        } else {
            s.skip_field("localize_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterDialogEmojiExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterDialogEmojiExcel");
        ds.field("group_id", &self.group_id());
        ds.field("target_index", &self.target_index());
        ds.field("dialog_type", &self.dialog_type());
        ds.field("duration", &self.duration());
        ds.field("duration_kr", &self.duration_kr());
        ds.field("hide_ui", &self.hide_ui());
        ds.field("localize_kr", &self.localize_kr());
        ds.field("localize_jp", &self.localize_jp());
        ds.field("localize_th", &self.localize_th());
        ds.field("localize_tw", &self.localize_tw());
        ds.field("localize_en", &self.localize_en());
        ds.finish()
    }
}
