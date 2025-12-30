extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ContentType, StageTopography, StrategyEnvironment};

#[derive(Copy, Clone, PartialEq)]
pub struct StoryStrategyExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for StoryStrategyExcel<'a> {
    type Inner = StoryStrategyExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> StoryStrategyExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_LOCALIZE: ::flatbuffers::VOffsetT = 8;
    pub const VT_STAGE_ENTER_ECHELON_COUNT: ::flatbuffers::VOffsetT = 10;
    pub const VT_BATTLE_DURATION: ::flatbuffers::VOffsetT = 12;
    pub const VT_WHITE_LIST_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_STRATEGY_MAP: ::flatbuffers::VOffsetT = 16;
    pub const VT_STRATEGY_MAP_BG: ::flatbuffers::VOffsetT = 18;
    pub const VT_MAX_TURN: ::flatbuffers::VOffsetT = 20;
    pub const VT_STAGE_TOPOGRAPHY: ::flatbuffers::VOffsetT = 22;
    pub const VT_STRATEGY_ENVIRONMENT: ::flatbuffers::VOffsetT = 24;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 26;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 28;
    pub const VT_FIRST_CLEAR_REPORT_EVENT_NAME: ::flatbuffers::VOffsetT = 30;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StoryStrategyExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(StoryStrategyExcel::VT_NAME, None)
        }
    }
    #[inline]
    pub fn localize(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(StoryStrategyExcel::VT_LOCALIZE, None)
        }
    }
    #[inline]
    pub fn stage_enter_echelon_count(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(StoryStrategyExcel::VT_STAGE_ENTER_ECHELON_COUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn battle_duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StoryStrategyExcel::VT_BATTLE_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn white_list_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StoryStrategyExcel::VT_WHITE_LIST_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn strategy_map(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StoryStrategyExcel::VT_STRATEGY_MAP,
                None,
            )
        }
    }
    #[inline]
    pub fn strategy_map_bg(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StoryStrategyExcel::VT_STRATEGY_MAP_BG,
                None,
            )
        }
    }
    #[inline]
    pub fn max_turn(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(StoryStrategyExcel::VT_MAX_TURN, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_topography(&self) -> StageTopography {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StageTopography>(
                    StoryStrategyExcel::VT_STAGE_TOPOGRAPHY,
                    Some(StageTopography::Street),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn strategy_environment(&self) -> StrategyEnvironment {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StrategyEnvironment>(
                    StoryStrategyExcel::VT_STRATEGY_ENVIRONMENT,
                    Some(StrategyEnvironment::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn content_type(&self) -> ContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ContentType>(StoryStrategyExcel::VT_CONTENT_TYPE, Some(ContentType::None))
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
                .get::<i64>(StoryStrategyExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn first_clear_report_event_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                StoryStrategyExcel::VT_FIRST_CLEAR_REPORT_EVENT_NAME,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for StoryStrategyExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize",
                Self::VT_LOCALIZE,
                false,
            )?
            .visit_field::<i32>(
                "stage_enter_echelon_count",
                Self::VT_STAGE_ENTER_ECHELON_COUNT,
                false,
            )?
            .visit_field::<i64>("battle_duration", Self::VT_BATTLE_DURATION, false)?
            .visit_field::<i64>("white_list_id", Self::VT_WHITE_LIST_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "strategy_map",
                Self::VT_STRATEGY_MAP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "strategy_map_bg",
                Self::VT_STRATEGY_MAP_BG,
                false,
            )?
            .visit_field::<i32>("max_turn", Self::VT_MAX_TURN, false)?
            .visit_field::<StageTopography>("stage_topography", Self::VT_STAGE_TOPOGRAPHY, false)?
            .visit_field::<StrategyEnvironment>(
                "strategy_environment",
                Self::VT_STRATEGY_ENVIRONMENT,
                false,
            )?
            .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "first_clear_report_event_name",
                Self::VT_FIRST_CLEAR_REPORT_EVENT_NAME,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for StoryStrategyExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StoryStrategyExcel", 14)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.name() {
            s.serialize_field("name", &f)?;
        } else {
            s.skip_field("name")?;
        }
        if let Some(f) = self.localize() {
            s.serialize_field("localize", &f)?;
        } else {
            s.skip_field("localize")?;
        }
        s.serialize_field(
            "stage_enter_echelon_count",
            &self.stage_enter_echelon_count(),
        )?;
        s.serialize_field("battle_duration", &self.battle_duration())?;
        s.serialize_field("white_list_id", &self.white_list_id())?;
        if let Some(f) = self.strategy_map() {
            s.serialize_field("strategy_map", &f)?;
        } else {
            s.skip_field("strategy_map")?;
        }
        if let Some(f) = self.strategy_map_bg() {
            s.serialize_field("strategy_map_bg", &f)?;
        } else {
            s.skip_field("strategy_map_bg")?;
        }
        s.serialize_field("max_turn", &self.max_turn())?;
        s.serialize_field("stage_topography", &self.stage_topography())?;
        s.serialize_field("strategy_environment", &self.strategy_environment())?;
        s.serialize_field("content_type", &self.content_type())?;
        s.serialize_field("bgm_id", &self.bgm_id())?;
        if let Some(f) = self.first_clear_report_event_name() {
            s.serialize_field("first_clear_report_event_name", &f)?;
        } else {
            s.skip_field("first_clear_report_event_name")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for StoryStrategyExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("StoryStrategyExcel");
        ds.field("id", &self.id());
        ds.field("name", &self.name());
        ds.field("localize", &self.localize());
        ds.field(
            "stage_enter_echelon_count",
            &self.stage_enter_echelon_count(),
        );
        ds.field("battle_duration", &self.battle_duration());
        ds.field("white_list_id", &self.white_list_id());
        ds.field("strategy_map", &self.strategy_map());
        ds.field("strategy_map_bg", &self.strategy_map_bg());
        ds.field("max_turn", &self.max_turn());
        ds.field("stage_topography", &self.stage_topography());
        ds.field("strategy_environment", &self.strategy_environment());
        ds.field("content_type", &self.content_type());
        ds.field("bgm_id", &self.bgm_id());
        ds.field(
            "first_clear_report_event_name",
            &self.first_clear_report_event_name(),
        );
        ds.finish()
    }
}
