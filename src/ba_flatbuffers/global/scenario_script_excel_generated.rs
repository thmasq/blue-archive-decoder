extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioScriptExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioScriptExcel<'a> {
    type Inner = ScenarioScriptExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioScriptExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SELECTION_GROUP: ::flatbuffers::VOffsetT = 6;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_SOUND: ::flatbuffers::VOffsetT = 10;
    pub const VT_TRANSITION: ::flatbuffers::VOffsetT = 12;
    pub const VT_BG_NAME: ::flatbuffers::VOffsetT = 14;
    pub const VT_BG_EFFECT: ::flatbuffers::VOffsetT = 16;
    pub const VT_POPUP_FILE_NAME: ::flatbuffers::VOffsetT = 18;
    pub const VT_SCRIPT_KR: ::flatbuffers::VOffsetT = 20;
    pub const VT_TEXT_JP: ::flatbuffers::VOffsetT = 22;
    pub const VT_TEXT_TH: ::flatbuffers::VOffsetT = 24;
    pub const VT_TEXT_TW: ::flatbuffers::VOffsetT = 26;
    pub const VT_TEXT_EN: ::flatbuffers::VOffsetT = 28;
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 30;
    pub const VT_TEEN_MODE: ::flatbuffers::VOffsetT = 32;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioScriptExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn selection_group(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioScriptExcel::VT_SELECTION_GROUP, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioScriptExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sound(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioScriptExcel::VT_SOUND, None)
        }
    }
    #[inline]
    pub fn transition(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioScriptExcel::VT_TRANSITION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bg_name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioScriptExcel::VT_BG_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bg_effect(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioScriptExcel::VT_BG_EFFECT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn popup_file_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioScriptExcel::VT_POPUP_FILE_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn script_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioScriptExcel::VT_SCRIPT_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn text_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioScriptExcel::VT_TEXT_JP, None)
        }
    }
    #[inline]
    pub fn text_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioScriptExcel::VT_TEXT_TH, None)
        }
    }
    #[inline]
    pub fn text_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioScriptExcel::VT_TEXT_TW, None)
        }
    }
    #[inline]
    pub fn text_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioScriptExcel::VT_TEXT_EN, None)
        }
    }
    #[inline]
    pub fn voice_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioScriptExcel::VT_VOICE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn teen_mode(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioScriptExcel::VT_TEEN_MODE, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioScriptExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<i64>("selection_group", Self::VT_SELECTION_GROUP, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("sound", Self::VT_SOUND, false)?
            .visit_field::<u32>("transition", Self::VT_TRANSITION, false)?
            .visit_field::<u32>("bg_name", Self::VT_BG_NAME, false)?
            .visit_field::<u32>("bg_effect", Self::VT_BG_EFFECT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "popup_file_name",
                Self::VT_POPUP_FILE_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "script_kr",
                Self::VT_SCRIPT_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "text_jp",
                Self::VT_TEXT_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "text_th",
                Self::VT_TEXT_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "text_tw",
                Self::VT_TEXT_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "text_en",
                Self::VT_TEXT_EN,
                false,
            )?
            .visit_field::<u32>("voice_id", Self::VT_VOICE_ID, false)?
            .visit_field::<bool>("teen_mode", Self::VT_TEEN_MODE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioScriptExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioScriptExcel", 15)?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("selection_group", &self.selection_group())?;
        s.serialize_field("bgm_id", &self.bgm_id())?;
        if let Some(f) = self.sound() {
            s.serialize_field("sound", &f)?;
        } else {
            s.skip_field("sound")?;
        }
        s.serialize_field("transition", &self.transition())?;
        s.serialize_field("bg_name", &self.bg_name())?;
        s.serialize_field("bg_effect", &self.bg_effect())?;
        if let Some(f) = self.popup_file_name() {
            s.serialize_field("popup_file_name", &f)?;
        } else {
            s.skip_field("popup_file_name")?;
        }
        if let Some(f) = self.script_kr() {
            s.serialize_field("script_kr", &f)?;
        } else {
            s.skip_field("script_kr")?;
        }
        if let Some(f) = self.text_jp() {
            s.serialize_field("text_jp", &f)?;
        } else {
            s.skip_field("text_jp")?;
        }
        if let Some(f) = self.text_th() {
            s.serialize_field("text_th", &f)?;
        } else {
            s.skip_field("text_th")?;
        }
        if let Some(f) = self.text_tw() {
            s.serialize_field("text_tw", &f)?;
        } else {
            s.skip_field("text_tw")?;
        }
        if let Some(f) = self.text_en() {
            s.serialize_field("text_en", &f)?;
        } else {
            s.skip_field("text_en")?;
        }
        s.serialize_field("voice_id", &self.voice_id())?;
        s.serialize_field("teen_mode", &self.teen_mode())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioScriptExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioScriptExcel");
        ds.field("group_id", &self.group_id());
        ds.field("selection_group", &self.selection_group());
        ds.field("bgm_id", &self.bgm_id());
        ds.field("sound", &self.sound());
        ds.field("transition", &self.transition());
        ds.field("bg_name", &self.bg_name());
        ds.field("bg_effect", &self.bg_effect());
        ds.field("popup_file_name", &self.popup_file_name());
        ds.field("script_kr", &self.script_kr());
        ds.field("text_jp", &self.text_jp());
        ds.field("text_th", &self.text_th());
        ds.field("text_tw", &self.text_tw());
        ds.field("text_en", &self.text_en());
        ds.field("voice_id", &self.voice_id());
        ds.field("teen_mode", &self.teen_mode());
        ds.finish()
    }
}
