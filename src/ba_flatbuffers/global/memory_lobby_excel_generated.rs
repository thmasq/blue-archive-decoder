extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{MemoryLobbyCategory, ProductionStep};

#[derive(Copy, Clone, PartialEq)]
pub struct MemoryLobbyExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MemoryLobbyExcel<'a> {
    type Inner = MemoryLobbyExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MemoryLobbyExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_PRODUCTION_STEP: ::flatbuffers::VOffsetT = 6;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_PREFAB_NAME: ::flatbuffers::VOffsetT = 12;
    pub const VT_MEMORY_LOBBY_CATEGORY: ::flatbuffers::VOffsetT = 14;
    pub const VT_SLOT_TEXTURE_NAME: ::flatbuffers::VOffsetT = 16;
    pub const VT_REWARD_TEXTURE_NAME: ::flatbuffers::VOffsetT = 18;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 20;
    pub const VT_AUDIO_CLIP_JP: ::flatbuffers::VOffsetT = 22;
    pub const VT_AUDIO_CLIP_KR: ::flatbuffers::VOffsetT = 24;
    pub const VT_AUDIO_CLIP_TH: ::flatbuffers::VOffsetT = 26;
    pub const VT_AUDIO_CLIP_TW: ::flatbuffers::VOffsetT = 28;
    pub const VT_AUDIO_CLIP_EN: ::flatbuffers::VOffsetT = 30;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MemoryLobbyExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn production_step(&self) -> ProductionStep {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ProductionStep>(
                    MemoryLobbyExcel::VT_PRODUCTION_STEP,
                    Some(ProductionStep::ToDo),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MemoryLobbyExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MemoryLobbyExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn prefab_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(MemoryLobbyExcel::VT_PREFAB_NAME, None)
        }
    }
    #[inline]
    pub fn memory_lobby_category(&self) -> MemoryLobbyCategory {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MemoryLobbyCategory>(
                    MemoryLobbyExcel::VT_MEMORY_LOBBY_CATEGORY,
                    Some(MemoryLobbyCategory::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn slot_texture_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_SLOT_TEXTURE_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn reward_texture_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_REWARD_TEXTURE_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn bgm_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MemoryLobbyExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn audio_clip_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_AUDIO_CLIP_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn audio_clip_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_AUDIO_CLIP_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn audio_clip_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_AUDIO_CLIP_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn audio_clip_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_AUDIO_CLIP_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn audio_clip_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobbyExcel::VT_AUDIO_CLIP_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for MemoryLobbyExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<ProductionStep>("production_step", Self::VT_PRODUCTION_STEP, false)?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name",
                Self::VT_PREFAB_NAME,
                false,
            )?
            .visit_field::<MemoryLobbyCategory>(
                "memory_lobby_category",
                Self::VT_MEMORY_LOBBY_CATEGORY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "slot_texture_name",
                Self::VT_SLOT_TEXTURE_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "reward_texture_name",
                Self::VT_REWARD_TEXTURE_NAME,
                false,
            )?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_clip_jp",
                Self::VT_AUDIO_CLIP_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_clip_kr",
                Self::VT_AUDIO_CLIP_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_clip_th",
                Self::VT_AUDIO_CLIP_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_clip_tw",
                Self::VT_AUDIO_CLIP_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_clip_en",
                Self::VT_AUDIO_CLIP_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MemoryLobbyExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MemoryLobbyExcel", 14)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("production_step", &self.production_step())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        s.serialize_field("character_id", &self.character_id())?;
        if let Some(f) = self.prefab_name() {
            s.serialize_field("prefab_name", &f)?;
        } else {
            s.skip_field("prefab_name")?;
        }
        s.serialize_field("memory_lobby_category", &self.memory_lobby_category())?;
        if let Some(f) = self.slot_texture_name() {
            s.serialize_field("slot_texture_name", &f)?;
        } else {
            s.skip_field("slot_texture_name")?;
        }
        if let Some(f) = self.reward_texture_name() {
            s.serialize_field("reward_texture_name", &f)?;
        } else {
            s.skip_field("reward_texture_name")?;
        }
        s.serialize_field("bgm_id", &self.bgm_id())?;
        if let Some(f) = self.audio_clip_jp() {
            s.serialize_field("audio_clip_jp", &f)?;
        } else {
            s.skip_field("audio_clip_jp")?;
        }
        if let Some(f) = self.audio_clip_kr() {
            s.serialize_field("audio_clip_kr", &f)?;
        } else {
            s.skip_field("audio_clip_kr")?;
        }
        if let Some(f) = self.audio_clip_th() {
            s.serialize_field("audio_clip_th", &f)?;
        } else {
            s.skip_field("audio_clip_th")?;
        }
        if let Some(f) = self.audio_clip_tw() {
            s.serialize_field("audio_clip_tw", &f)?;
        } else {
            s.skip_field("audio_clip_tw")?;
        }
        if let Some(f) = self.audio_clip_en() {
            s.serialize_field("audio_clip_en", &f)?;
        } else {
            s.skip_field("audio_clip_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MemoryLobbyExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MemoryLobbyExcel");
        ds.field("id", &self.id());
        ds.field("production_step", &self.production_step());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("character_id", &self.character_id());
        ds.field("prefab_name", &self.prefab_name());
        ds.field("memory_lobby_category", &self.memory_lobby_category());
        ds.field("slot_texture_name", &self.slot_texture_name());
        ds.field("reward_texture_name", &self.reward_texture_name());
        ds.field("bgm_id", &self.bgm_id());
        ds.field("audio_clip_jp", &self.audio_clip_jp());
        ds.field("audio_clip_kr", &self.audio_clip_kr());
        ds.field("audio_clip_th", &self.audio_clip_th());
        ds.field("audio_clip_tw", &self.audio_clip_tw());
        ds.field("audio_clip_en", &self.audio_clip_en());
        ds.finish()
    }
}
