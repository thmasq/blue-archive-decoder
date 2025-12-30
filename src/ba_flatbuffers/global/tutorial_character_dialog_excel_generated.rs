extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct TutorialCharacterDialogExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for TutorialCharacterDialogExcel<'a> {
    type Inner = TutorialCharacterDialogExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> TutorialCharacterDialogExcel<'a> {
    pub const VT_TALK_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ANIMATION_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_LOCALIZE_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_LOCALIZE_JP: ::flatbuffers::VOffsetT = 10;
    pub const VT_LOCALIZE_TH: ::flatbuffers::VOffsetT = 12;
    pub const VT_LOCALIZE_TW: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOCALIZE_EN: ::flatbuffers::VOffsetT = 16;
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn talk_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(TutorialCharacterDialogExcel::VT_TALK_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn animation_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialCharacterDialogExcel::VT_ANIMATION_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialCharacterDialogExcel::VT_LOCALIZE_KR,
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
                TutorialCharacterDialogExcel::VT_LOCALIZE_JP,
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
                TutorialCharacterDialogExcel::VT_LOCALIZE_TH,
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
                TutorialCharacterDialogExcel::VT_LOCALIZE_TW,
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
                TutorialCharacterDialogExcel::VT_LOCALIZE_EN,
                None,
            )
        }
    }
    #[inline]
    pub fn voice_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(TutorialCharacterDialogExcel::VT_VOICE_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for TutorialCharacterDialogExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("talk_id", Self::VT_TALK_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "animation_name",
                Self::VT_ANIMATION_NAME,
                false,
            )?
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
            .visit_field::<u32>("voice_id", Self::VT_VOICE_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for TutorialCharacterDialogExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TutorialCharacterDialogExcel", 8)?;
        s.serialize_field("talk_id", &self.talk_id())?;
        if let Some(f) = self.animation_name() {
            s.serialize_field("animation_name", &f)?;
        } else {
            s.skip_field("animation_name")?;
        }
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
        s.serialize_field("voice_id", &self.voice_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for TutorialCharacterDialogExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("TutorialCharacterDialogExcel");
        ds.field("talk_id", &self.talk_id());
        ds.field("animation_name", &self.animation_name());
        ds.field("localize_kr", &self.localize_kr());
        ds.field("localize_jp", &self.localize_jp());
        ds.field("localize_th", &self.localize_th());
        ds.field("localize_tw", &self.localize_tw());
        ds.field("localize_en", &self.localize_en());
        ds.field("voice_id", &self.voice_id());
        ds.finish()
    }
}
