extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterDialogSubtitleExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterDialogSubtitleExcel<'a> {
    type Inner = CharacterDialogSubtitleExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterDialogSubtitleExcel<'a> {
    pub const VT_LOCALIZE_CV_GROUP: ::flatbuffers::VOffsetT = 4;
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_TLMID: ::flatbuffers::VOffsetT = 8;
    pub const VT_DURATION: ::flatbuffers::VOffsetT = 10;
    pub const VT_DURATION_KR: ::flatbuffers::VOffsetT = 12;
    pub const VT_SEPARATE: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOCALIZE_KR: ::flatbuffers::VOffsetT = 16;
    pub const VT_LOCALIZE_JP: ::flatbuffers::VOffsetT = 18;
    pub const VT_LOCALIZE_TH: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOCALIZE_TW: ::flatbuffers::VOffsetT = 22;
    pub const VT_LOCALIZE_EN: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn localize_cv_group(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogSubtitleExcel::VT_LOCALIZE_CV_GROUP,
                None,
            )
        }
    }
    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogSubtitleExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn tlmid(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogSubtitleExcel::VT_TLMID,
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
                .get::<i64>(CharacterDialogSubtitleExcel::VT_DURATION, Some(0))
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
                .get::<i64>(CharacterDialogSubtitleExcel::VT_DURATION_KR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn separate(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogSubtitleExcel::VT_SEPARATE, Some(false))
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
                CharacterDialogSubtitleExcel::VT_LOCALIZE_KR,
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
                CharacterDialogSubtitleExcel::VT_LOCALIZE_JP,
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
                CharacterDialogSubtitleExcel::VT_LOCALIZE_TH,
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
                CharacterDialogSubtitleExcel::VT_LOCALIZE_TW,
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
                CharacterDialogSubtitleExcel::VT_LOCALIZE_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterDialogSubtitleExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_cv_group",
                Self::VT_LOCALIZE_CV_GROUP,
                false,
            )?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("tlmid", Self::VT_TLMID, false)?
            .visit_field::<i64>("duration", Self::VT_DURATION, false)?
            .visit_field::<i64>("duration_kr", Self::VT_DURATION_KR, false)?
            .visit_field::<bool>("separate", Self::VT_SEPARATE, false)?
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

impl Serialize for CharacterDialogSubtitleExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterDialogSubtitleExcel", 11)?;
        if let Some(f) = self.localize_cv_group() {
            s.serialize_field("localize_cv_group", &f)?;
        } else {
            s.skip_field("localize_cv_group")?;
        }
        s.serialize_field("character_id", &self.character_id())?;
        if let Some(f) = self.tlmid() {
            s.serialize_field("tlmid", &f)?;
        } else {
            s.skip_field("tlmid")?;
        }
        s.serialize_field("duration", &self.duration())?;
        s.serialize_field("duration_kr", &self.duration_kr())?;
        s.serialize_field("separate", &self.separate())?;
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

impl ::core::fmt::Debug for CharacterDialogSubtitleExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterDialogSubtitleExcel");
        ds.field("localize_cv_group", &self.localize_cv_group());
        ds.field("character_id", &self.character_id());
        ds.field("tlmid", &self.tlmid());
        ds.field("duration", &self.duration());
        ds.field("duration_kr", &self.duration_kr());
        ds.field("separate", &self.separate());
        ds.field("localize_kr", &self.localize_kr());
        ds.field("localize_jp", &self.localize_jp());
        ds.field("localize_th", &self.localize_th());
        ds.field("localize_tw", &self.localize_tw());
        ds.field("localize_en", &self.localize_en());
        ds.finish()
    }
}
