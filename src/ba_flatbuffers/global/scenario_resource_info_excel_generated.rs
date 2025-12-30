extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioResourceInfoExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioResourceInfoExcel<'a> {
    type Inner = ScenarioResourceInfoExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioResourceInfoExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SCENARIO_MODE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_PRIORITY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_PV_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 10;
    pub const VT_VIDEO_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_AUDIO_NAME: ::flatbuffers::VOffsetT = 16;
    pub const VT_SPINE_PATH: ::flatbuffers::VOffsetT = 18;
    pub const VT_RATIO: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOBBY_ANI_PATH: ::flatbuffers::VOffsetT = 22;
    pub const VT_MOVIE_CG_PATH: ::flatbuffers::VOffsetT = 24;
    pub const VT_LOCALIZE_ID: ::flatbuffers::VOffsetT = 26;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioResourceInfoExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioResourceInfoExcel::VT_SCENARIO_MODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn priority_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioResourceInfoExcel::VT_PRIORITY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn pv_display_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioResourceInfoExcel::VT_PV_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn video_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioResourceInfoExcel::VT_VIDEO_ID, Some(0))
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
                .get::<i64>(ScenarioResourceInfoExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn audio_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioResourceInfoExcel::VT_AUDIO_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn spine_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioResourceInfoExcel::VT_SPINE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn ratio(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioResourceInfoExcel::VT_RATIO, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn lobby_ani_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioResourceInfoExcel::VT_LOBBY_ANI_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn movie_cg_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioResourceInfoExcel::VT_MOVIE_CG_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioResourceInfoExcel::VT_LOCALIZE_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioResourceInfoExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("scenario_mode_id", Self::VT_SCENARIO_MODE_ID, false)?
            .visit_field::<i64>("priority_order", Self::VT_PRIORITY_ORDER, false)?
            .visit_field::<i64>("pv_display_order", Self::VT_PV_DISPLAY_ORDER, false)?
            .visit_field::<i64>("video_id", Self::VT_VIDEO_ID, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "audio_name",
                Self::VT_AUDIO_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "spine_path",
                Self::VT_SPINE_PATH,
                false,
            )?
            .visit_field::<i32>("ratio", Self::VT_RATIO, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "lobby_ani_path",
                Self::VT_LOBBY_ANI_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "movie_cg_path",
                Self::VT_MOVIE_CG_PATH,
                false,
            )?
            .visit_field::<u32>("localize_id", Self::VT_LOCALIZE_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioResourceInfoExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioResourceInfoExcel", 12)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("scenario_mode_id", &self.scenario_mode_id())?;
        s.serialize_field("priority_order", &self.priority_order())?;
        s.serialize_field("pv_display_order", &self.pv_display_order())?;
        s.serialize_field("video_id", &self.video_id())?;
        s.serialize_field("bgm_id", &self.bgm_id())?;
        if let Some(f) = self.audio_name() {
            s.serialize_field("audio_name", &f)?;
        } else {
            s.skip_field("audio_name")?;
        }
        if let Some(f) = self.spine_path() {
            s.serialize_field("spine_path", &f)?;
        } else {
            s.skip_field("spine_path")?;
        }
        s.serialize_field("ratio", &self.ratio())?;
        if let Some(f) = self.lobby_ani_path() {
            s.serialize_field("lobby_ani_path", &f)?;
        } else {
            s.skip_field("lobby_ani_path")?;
        }
        if let Some(f) = self.movie_cg_path() {
            s.serialize_field("movie_cg_path", &f)?;
        } else {
            s.skip_field("movie_cg_path")?;
        }
        s.serialize_field("localize_id", &self.localize_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioResourceInfoExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioResourceInfoExcel");
        ds.field("id", &self.id());
        ds.field("scenario_mode_id", &self.scenario_mode_id());
        ds.field("priority_order", &self.priority_order());
        ds.field("pv_display_order", &self.pv_display_order());
        ds.field("video_id", &self.video_id());
        ds.field("bgm_id", &self.bgm_id());
        ds.field("audio_name", &self.audio_name());
        ds.field("spine_path", &self.spine_path());
        ds.field("ratio", &self.ratio());
        ds.field("lobby_ani_path", &self.lobby_ani_path());
        ds.field("movie_cg_path", &self.movie_cg_path());
        ds.field("localize_id", &self.localize_id());
        ds.finish()
    }
}
