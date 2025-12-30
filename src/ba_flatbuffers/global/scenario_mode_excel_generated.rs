extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{
    Club, EchelonExtensionType, EventContentType, ScenarioModeSubTypes, ScenarioModeTypes,
    StageDifficulty, StageTopography,
};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioModeExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioModeExcel<'a> {
    type Inner = ScenarioModeExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioModeExcel<'a> {
    pub const VT_MODE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_MODE_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_SUB_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_VOLUME_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_CHAPTER_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_EPISODE_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_EXPOSED_TIME: ::flatbuffers::VOffsetT = 16;
    pub const VT_HIDE: ::flatbuffers::VOffsetT = 18;
    pub const VT_OPEN: ::flatbuffers::VOffsetT = 20;
    pub const VT_SCENARIO_OPEN_DATE: ::flatbuffers::VOffsetT = 22;
    pub const VT_SCENARIO_CLOSE_DATE: ::flatbuffers::VOffsetT = 24;
    pub const VT_IS_CONTINUE: ::flatbuffers::VOffsetT = 26;
    pub const VT_EPISODE_CONTINUE_MODE_ID: ::flatbuffers::VOffsetT = 28;
    pub const VT_FRONT_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 30;
    pub const VT_STRATEGY_ID: ::flatbuffers::VOffsetT = 32;
    pub const VT_GROUND_ID: ::flatbuffers::VOffsetT = 34;
    pub const VT_IS_DEFEAT_BATTLE: ::flatbuffers::VOffsetT = 36;
    pub const VT_BATTLE_DURATION: ::flatbuffers::VOffsetT = 38;
    pub const VT_BACK_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 40;
    pub const VT_CLEARED_MODE_ID: ::flatbuffers::VOffsetT = 42;
    pub const VT_SCENARIO_MODE_REWARD_ID: ::flatbuffers::VOffsetT = 44;
    pub const VT_IS_SCENARIO_SPECIAL_REWARD: ::flatbuffers::VOffsetT = 46;
    pub const VT_ACCOUNT_LEVEL_LIMIT: ::flatbuffers::VOffsetT = 48;
    pub const VT_CLEARED_STAGE_ID: ::flatbuffers::VOffsetT = 50;
    pub const VT_NEED_CLUB: ::flatbuffers::VOffsetT = 52;
    pub const VT_NEED_CLUB_STUDENT_COUNT: ::flatbuffers::VOffsetT = 54;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 56;
    pub const VT_EVENT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 58;
    pub const VT_EVENT_CONTENT_CONDITION: ::flatbuffers::VOffsetT = 60;
    pub const VT_EVENT_CONTENT_CONDITION_GROUP: ::flatbuffers::VOffsetT = 62;
    pub const VT_MAP_DIFFICULTY: ::flatbuffers::VOffsetT = 64;
    pub const VT_STEP_INDEX: ::flatbuffers::VOffsetT = 66;
    pub const VT_RECOMMEND_LEVEL: ::flatbuffers::VOffsetT = 68;
    pub const VT_EVENT_ICON_PARCEL_PATH: ::flatbuffers::VOffsetT = 70;
    pub const VT_EVENT_BANNER_TITLE: ::flatbuffers::VOffsetT = 72;
    pub const VT_LOF: ::flatbuffers::VOffsetT = 74;
    pub const VT_STAGE_TOPOGRAPHY: ::flatbuffers::VOffsetT = 76;
    pub const VT_FIXED_ECHELON_ID: ::flatbuffers::VOffsetT = 78;
    pub const VT_COMPLETE_REPORT_EVENT_NAME: ::flatbuffers::VOffsetT = 80;
    pub const VT_ECHELON_EXTENSION_TYPE: ::flatbuffers::VOffsetT = 82;
    pub const VT_COLLECTION_GROUP_ID: ::flatbuffers::VOffsetT = 84;
    pub const VT_FIRST_CLEAR_FUNNEL_MESSAGE: ::flatbuffers::VOffsetT = 86;

    #[inline]
    pub fn mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_MODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn mode_type(&self) -> ScenarioModeTypes {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioModeTypes>(
                    ScenarioModeExcel::VT_MODE_TYPE,
                    Some(ScenarioModeTypes::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn sub_type(&self) -> ScenarioModeSubTypes {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioModeSubTypes>(
                    ScenarioModeExcel::VT_SUB_TYPE,
                    Some(ScenarioModeSubTypes::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn volume_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_VOLUME_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn chapter_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_CHAPTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn episode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_EPISODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn exposed_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_EXPOSED_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn hide(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeExcel::VT_HIDE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn open(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeExcel::VT_OPEN, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_open_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_SCENARIO_OPEN_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn scenario_close_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_SCENARIO_CLOSE_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn is_continue(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeExcel::VT_IS_CONTINUE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn episode_continue_mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_EPISODE_CONTINUE_MODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn front_scenario_group_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    ScenarioModeExcel::VT_FRONT_SCENARIO_GROUP_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn strategy_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_STRATEGY_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ground_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_GROUND_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_defeat_battle(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeExcel::VT_IS_DEFEAT_BATTLE, Some(false))
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
                .get::<i64>(ScenarioModeExcel::VT_BATTLE_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn back_scenario_group_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    ScenarioModeExcel::VT_BACK_SCENARIO_GROUP_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn cleared_mode_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    ScenarioModeExcel::VT_CLEARED_MODE_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn scenario_mode_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_SCENARIO_MODE_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_scenario_special_reward(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    ScenarioModeExcel::VT_IS_SCENARIO_SPECIAL_REWARD,
                    Some(false),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn account_level_limit(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_ACCOUNT_LEVEL_LIMIT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn cleared_stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_CLEARED_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn need_club(&self) -> Club {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Club>(ScenarioModeExcel::VT_NEED_CLUB, Some(Club::None))
                .unwrap()
        }
    }
    #[inline]
    pub fn need_club_student_count(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioModeExcel::VT_NEED_CLUB_STUDENT_COUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_type(&self) -> EventContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EventContentType>(
                    ScenarioModeExcel::VT_EVENT_CONTENT_TYPE,
                    Some(EventContentType::Stage),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_condition(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_EVENT_CONTENT_CONDITION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_condition_group(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_EVENT_CONTENT_CONDITION_GROUP, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn map_difficulty(&self) -> StageDifficulty {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StageDifficulty>(
                    ScenarioModeExcel::VT_MAP_DIFFICULTY,
                    Some(StageDifficulty::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn step_index(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioModeExcel::VT_STEP_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn recommend_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioModeExcel::VT_RECOMMEND_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_icon_parcel_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_EVENT_ICON_PARCEL_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn event_banner_title(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioModeExcel::VT_EVENT_BANNER_TITLE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn lof(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioModeExcel::VT_LOF, Some(false))
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
                    ScenarioModeExcel::VT_STAGE_TOPOGRAPHY,
                    Some(StageTopography::Street),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn fixed_echelon_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_FIXED_ECHELON_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn complete_report_event_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_COMPLETE_REPORT_EVENT_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn echelon_extension_type(&self) -> EchelonExtensionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EchelonExtensionType>(
                    ScenarioModeExcel::VT_ECHELON_EXTENSION_TYPE,
                    Some(EchelonExtensionType::Base),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn collection_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioModeExcel::VT_COLLECTION_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn first_clear_funnel_message(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioModeExcel::VT_FIRST_CLEAR_FUNNEL_MESSAGE,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioModeExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("mode_id", Self::VT_MODE_ID, false)?
            .visit_field::<ScenarioModeTypes>("mode_type", Self::VT_MODE_TYPE, false)?
            .visit_field::<ScenarioModeSubTypes>("sub_type", Self::VT_SUB_TYPE, false)?
            .visit_field::<i64>("volume_id", Self::VT_VOLUME_ID, false)?
            .visit_field::<i64>("chapter_id", Self::VT_CHAPTER_ID, false)?
            .visit_field::<i64>("episode_id", Self::VT_EPISODE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "exposed_time",
                Self::VT_EXPOSED_TIME,
                false,
            )?
            .visit_field::<bool>("hide", Self::VT_HIDE, false)?
            .visit_field::<bool>("open", Self::VT_OPEN, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "scenario_open_date",
                Self::VT_SCENARIO_OPEN_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "scenario_close_date",
                Self::VT_SCENARIO_CLOSE_DATE,
                false,
            )?
            .visit_field::<bool>("is_continue", Self::VT_IS_CONTINUE, false)?
            .visit_field::<i64>(
                "episode_continue_mode_id",
                Self::VT_EPISODE_CONTINUE_MODE_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "front_scenario_group_id",
                Self::VT_FRONT_SCENARIO_GROUP_ID,
                false,
            )?
            .visit_field::<i64>("strategy_id", Self::VT_STRATEGY_ID, false)?
            .visit_field::<i64>("ground_id", Self::VT_GROUND_ID, false)?
            .visit_field::<bool>("is_defeat_battle", Self::VT_IS_DEFEAT_BATTLE, false)?
            .visit_field::<i64>("battle_duration", Self::VT_BATTLE_DURATION, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "back_scenario_group_id",
                Self::VT_BACK_SCENARIO_GROUP_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "cleared_mode_id",
                Self::VT_CLEARED_MODE_ID,
                false,
            )?
            .visit_field::<i64>(
                "scenario_mode_reward_id",
                Self::VT_SCENARIO_MODE_REWARD_ID,
                false,
            )?
            .visit_field::<bool>(
                "is_scenario_special_reward",
                Self::VT_IS_SCENARIO_SPECIAL_REWARD,
                false,
            )?
            .visit_field::<i64>("account_level_limit", Self::VT_ACCOUNT_LEVEL_LIMIT, false)?
            .visit_field::<i64>("cleared_stage_id", Self::VT_CLEARED_STAGE_ID, false)?
            .visit_field::<Club>("need_club", Self::VT_NEED_CLUB, false)?
            .visit_field::<i32>(
                "need_club_student_count",
                Self::VT_NEED_CLUB_STUDENT_COUNT,
                false,
            )?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<EventContentType>(
                "event_content_type",
                Self::VT_EVENT_CONTENT_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "event_content_condition",
                Self::VT_EVENT_CONTENT_CONDITION,
                false,
            )?
            .visit_field::<i64>(
                "event_content_condition_group",
                Self::VT_EVENT_CONTENT_CONDITION_GROUP,
                false,
            )?
            .visit_field::<StageDifficulty>("map_difficulty", Self::VT_MAP_DIFFICULTY, false)?
            .visit_field::<i32>("step_index", Self::VT_STEP_INDEX, false)?
            .visit_field::<i32>("recommend_level", Self::VT_RECOMMEND_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "event_icon_parcel_path",
                Self::VT_EVENT_ICON_PARCEL_PATH,
                false,
            )?
            .visit_field::<u32>("event_banner_title", Self::VT_EVENT_BANNER_TITLE, false)?
            .visit_field::<bool>("lof", Self::VT_LOF, false)?
            .visit_field::<StageTopography>("stage_topography", Self::VT_STAGE_TOPOGRAPHY, false)?
            .visit_field::<i64>("fixed_echelon_id", Self::VT_FIXED_ECHELON_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "complete_report_event_name",
                Self::VT_COMPLETE_REPORT_EVENT_NAME,
                false,
            )?
            .visit_field::<EchelonExtensionType>(
                "echelon_extension_type",
                Self::VT_ECHELON_EXTENSION_TYPE,
                false,
            )?
            .visit_field::<i64>("collection_group_id", Self::VT_COLLECTION_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "first_clear_funnel_message",
                Self::VT_FIRST_CLEAR_FUNNEL_MESSAGE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioModeExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioModeExcel", 42)?;
        s.serialize_field("mode_id", &self.mode_id())?;
        s.serialize_field("mode_type", &self.mode_type())?;
        s.serialize_field("sub_type", &self.sub_type())?;
        s.serialize_field("volume_id", &self.volume_id())?;
        s.serialize_field("chapter_id", &self.chapter_id())?;
        s.serialize_field("episode_id", &self.episode_id())?;
        if let Some(f) = self.exposed_time() {
            s.serialize_field("exposed_time", &f)?;
        } else {
            s.skip_field("exposed_time")?;
        }
        s.serialize_field("hide", &self.hide())?;
        s.serialize_field("open", &self.open())?;
        if let Some(f) = self.scenario_open_date() {
            s.serialize_field("scenario_open_date", &f)?;
        } else {
            s.skip_field("scenario_open_date")?;
        }
        if let Some(f) = self.scenario_close_date() {
            s.serialize_field("scenario_close_date", &f)?;
        } else {
            s.skip_field("scenario_close_date")?;
        }
        s.serialize_field("is_continue", &self.is_continue())?;
        s.serialize_field("episode_continue_mode_id", &self.episode_continue_mode_id())?;
        if let Some(f) = self.front_scenario_group_id() {
            s.serialize_field("front_scenario_group_id", &f)?;
        } else {
            s.skip_field("front_scenario_group_id")?;
        }
        s.serialize_field("strategy_id", &self.strategy_id())?;
        s.serialize_field("ground_id", &self.ground_id())?;
        s.serialize_field("is_defeat_battle", &self.is_defeat_battle())?;
        s.serialize_field("battle_duration", &self.battle_duration())?;
        if let Some(f) = self.back_scenario_group_id() {
            s.serialize_field("back_scenario_group_id", &f)?;
        } else {
            s.skip_field("back_scenario_group_id")?;
        }
        if let Some(f) = self.cleared_mode_id() {
            s.serialize_field("cleared_mode_id", &f)?;
        } else {
            s.skip_field("cleared_mode_id")?;
        }
        s.serialize_field("scenario_mode_reward_id", &self.scenario_mode_reward_id())?;
        s.serialize_field(
            "is_scenario_special_reward",
            &self.is_scenario_special_reward(),
        )?;
        s.serialize_field("account_level_limit", &self.account_level_limit())?;
        s.serialize_field("cleared_stage_id", &self.cleared_stage_id())?;
        s.serialize_field("need_club", &self.need_club())?;
        s.serialize_field("need_club_student_count", &self.need_club_student_count())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("event_content_type", &self.event_content_type())?;
        s.serialize_field("event_content_condition", &self.event_content_condition())?;
        s.serialize_field(
            "event_content_condition_group",
            &self.event_content_condition_group(),
        )?;
        s.serialize_field("map_difficulty", &self.map_difficulty())?;
        s.serialize_field("step_index", &self.step_index())?;
        s.serialize_field("recommend_level", &self.recommend_level())?;
        if let Some(f) = self.event_icon_parcel_path() {
            s.serialize_field("event_icon_parcel_path", &f)?;
        } else {
            s.skip_field("event_icon_parcel_path")?;
        }
        s.serialize_field("event_banner_title", &self.event_banner_title())?;
        s.serialize_field("lof", &self.lof())?;
        s.serialize_field("stage_topography", &self.stage_topography())?;
        s.serialize_field("fixed_echelon_id", &self.fixed_echelon_id())?;
        if let Some(f) = self.complete_report_event_name() {
            s.serialize_field("complete_report_event_name", &f)?;
        } else {
            s.skip_field("complete_report_event_name")?;
        }
        s.serialize_field("echelon_extension_type", &self.echelon_extension_type())?;
        s.serialize_field("collection_group_id", &self.collection_group_id())?;
        if let Some(f) = self.first_clear_funnel_message() {
            s.serialize_field("first_clear_funnel_message", &f)?;
        } else {
            s.skip_field("first_clear_funnel_message")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioModeExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioModeExcel");
        ds.field("mode_id", &self.mode_id());
        ds.field("mode_type", &self.mode_type());
        ds.field("sub_type", &self.sub_type());
        ds.field("volume_id", &self.volume_id());
        ds.field("chapter_id", &self.chapter_id());
        ds.field("episode_id", &self.episode_id());
        ds.field("exposed_time", &self.exposed_time());
        ds.field("hide", &self.hide());
        ds.field("open", &self.open());
        ds.field("scenario_open_date", &self.scenario_open_date());
        ds.field("scenario_close_date", &self.scenario_close_date());
        ds.field("is_continue", &self.is_continue());
        ds.field("episode_continue_mode_id", &self.episode_continue_mode_id());
        ds.field("front_scenario_group_id", &self.front_scenario_group_id());
        ds.field("strategy_id", &self.strategy_id());
        ds.field("ground_id", &self.ground_id());
        ds.field("is_defeat_battle", &self.is_defeat_battle());
        ds.field("battle_duration", &self.battle_duration());
        ds.field("back_scenario_group_id", &self.back_scenario_group_id());
        ds.field("cleared_mode_id", &self.cleared_mode_id());
        ds.field("scenario_mode_reward_id", &self.scenario_mode_reward_id());
        ds.field(
            "is_scenario_special_reward",
            &self.is_scenario_special_reward(),
        );
        ds.field("account_level_limit", &self.account_level_limit());
        ds.field("cleared_stage_id", &self.cleared_stage_id());
        ds.field("need_club", &self.need_club());
        ds.field("need_club_student_count", &self.need_club_student_count());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("event_content_type", &self.event_content_type());
        ds.field("event_content_condition", &self.event_content_condition());
        ds.field(
            "event_content_condition_group",
            &self.event_content_condition_group(),
        );
        ds.field("map_difficulty", &self.map_difficulty());
        ds.field("step_index", &self.step_index());
        ds.field("recommend_level", &self.recommend_level());
        ds.field("event_icon_parcel_path", &self.event_icon_parcel_path());
        ds.field("event_banner_title", &self.event_banner_title());
        ds.field("lof", &self.lof());
        ds.field("stage_topography", &self.stage_topography());
        ds.field("fixed_echelon_id", &self.fixed_echelon_id());
        ds.field(
            "complete_report_event_name",
            &self.complete_report_event_name(),
        );
        ds.field("echelon_extension_type", &self.echelon_extension_type());
        ds.field("collection_group_id", &self.collection_group_id());
        ds.field(
            "first_clear_funnel_message",
            &self.first_clear_funnel_message(),
        );
        ds.finish()
    }
}
