extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{
    ContentType, EchelonExtensionType, ParcelType, StageDifficulty, StageTopography, StarGoalType,
};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDefenseStageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDefenseStageExcel<'a> {
    type Inner = MiniGameDefenseStageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDefenseStageExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_STAGE_DIFFICULTY: ::flatbuffers::VOffsetT = 10;
    pub const VT_STAGE_DIFFICULTY_LOCALIZE: ::flatbuffers::VOffsetT = 12;
    pub const VT_STAGE_NUMBER: ::flatbuffers::VOffsetT = 14;
    pub const VT_STAGE_DISPLAY: ::flatbuffers::VOffsetT = 16;
    pub const VT_PREV_STAGE_ID: ::flatbuffers::VOffsetT = 18;
    pub const VT_ECHELON_EXTENSION_TYPE: ::flatbuffers::VOffsetT = 20;
    pub const VT_BATTLE_DURATION: ::flatbuffers::VOffsetT = 22;
    pub const VT_STAGE_ENTER_COST_TYPE: ::flatbuffers::VOffsetT = 24;
    pub const VT_STAGE_ENTER_COST_ID: ::flatbuffers::VOffsetT = 26;
    pub const VT_STAGE_ENTER_COST_AMOUNT: ::flatbuffers::VOffsetT = 28;
    pub const VT_EVENT_CONTENT_STAGE_REWARD_ID: ::flatbuffers::VOffsetT = 30;
    pub const VT_ENTER_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 32;
    pub const VT_CLEAR_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 34;
    pub const VT_STAGE_TOPOGRAPHY: ::flatbuffers::VOffsetT = 36;
    pub const VT_RECOMMAND_LEVEL: ::flatbuffers::VOffsetT = 38;
    pub const VT_GROUND_ID: ::flatbuffers::VOffsetT = 40;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 42;
    pub const VT_STAR_GOAL: ::flatbuffers::VOffsetT = 44;
    pub const VT_STAR_GOAL_AMOUNT: ::flatbuffers::VOffsetT = 46;
    pub const VT_DEFENSE_FORMATION_BG_PREFAB: ::flatbuffers::VOffsetT = 48;
    pub const VT_DEFENSE_FORMATION_BG_PREFAB_SCALE: ::flatbuffers::VOffsetT = 50;
    pub const VT_FIXED_ECHELON: ::flatbuffers::VOffsetT = 52;
    pub const VT_MININAGE_DEFENSE_FIXED_STAT_ID: ::flatbuffers::VOffsetT = 54;
    pub const VT_STAGE_HINT: ::flatbuffers::VOffsetT = 56;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseStageExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDefenseStageExcel::VT_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseStageExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_difficulty(&self) -> StageDifficulty {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StageDifficulty>(
                    MiniGameDefenseStageExcel::VT_STAGE_DIFFICULTY,
                    Some(StageDifficulty::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_difficulty_localize(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MiniGameDefenseStageExcel::VT_STAGE_DIFFICULTY_LOCALIZE,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_number(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseStageExcel::VT_STAGE_NUMBER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_display(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseStageExcel::VT_STAGE_DISPLAY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn prev_stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseStageExcel::VT_PREV_STAGE_ID, Some(0))
                .unwrap()
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
                    MiniGameDefenseStageExcel::VT_ECHELON_EXTENSION_TYPE,
                    Some(EchelonExtensionType::Base),
                )
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
                .get::<i64>(MiniGameDefenseStageExcel::VT_BATTLE_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_enter_cost_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameDefenseStageExcel::VT_STAGE_ENTER_COST_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_enter_cost_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseStageExcel::VT_STAGE_ENTER_COST_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_enter_cost_amount(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseStageExcel::VT_STAGE_ENTER_COST_AMOUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_stage_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseStageExcel::VT_EVENT_CONTENT_STAGE_REWARD_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn enter_scenario_group_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDefenseStageExcel::VT_ENTER_SCENARIO_GROUP_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn clear_scenario_group_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDefenseStageExcel::VT_CLEAR_SCENARIO_GROUP_ID,
                    None,
                )
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
                    MiniGameDefenseStageExcel::VT_STAGE_TOPOGRAPHY,
                    Some(StageTopography::Street),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn recommand_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseStageExcel::VT_RECOMMAND_LEVEL, Some(0))
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
                .get::<i64>(MiniGameDefenseStageExcel::VT_GROUND_ID, Some(0))
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
                .get::<ContentType>(
                    MiniGameDefenseStageExcel::VT_CONTENT_TYPE,
                    Some(ContentType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn star_goal(&self) -> Option<::flatbuffers::Vector<'a, StarGoalType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, StarGoalType>>>(
                    MiniGameDefenseStageExcel::VT_STAR_GOAL,
                    None,
                )
        }
    }
    #[inline]
    pub fn star_goal_amount(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    MiniGameDefenseStageExcel::VT_STAR_GOAL_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn defense_formation_bg_prefab(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDefenseStageExcel::VT_DEFENSE_FORMATION_BG_PREFAB,
                None,
            )
        }
    }
    #[inline]
    pub fn defense_formation_bg_prefab_scale(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(
                    MiniGameDefenseStageExcel::VT_DEFENSE_FORMATION_BG_PREFAB_SCALE,
                    Some(0.0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn fixed_echelon(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseStageExcel::VT_FIXED_ECHELON, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn mininage_defense_fixed_stat_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseStageExcel::VT_MININAGE_DEFENSE_FIXED_STAT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_hint(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MiniGameDefenseStageExcel::VT_STAGE_HINT, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDefenseStageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<StageDifficulty>("stage_difficulty", Self::VT_STAGE_DIFFICULTY, false)?
            .visit_field::<u32>(
                "stage_difficulty_localize",
                Self::VT_STAGE_DIFFICULTY_LOCALIZE,
                false,
            )?
            .visit_field::<i32>("stage_number", Self::VT_STAGE_NUMBER, false)?
            .visit_field::<i32>("stage_display", Self::VT_STAGE_DISPLAY, false)?
            .visit_field::<i64>("prev_stage_id", Self::VT_PREV_STAGE_ID, false)?
            .visit_field::<EchelonExtensionType>(
                "echelon_extension_type",
                Self::VT_ECHELON_EXTENSION_TYPE,
                false,
            )?
            .visit_field::<i64>("battle_duration", Self::VT_BATTLE_DURATION, false)?
            .visit_field::<ParcelType>(
                "stage_enter_cost_type",
                Self::VT_STAGE_ENTER_COST_TYPE,
                false,
            )?
            .visit_field::<i64>("stage_enter_cost_id", Self::VT_STAGE_ENTER_COST_ID, false)?
            .visit_field::<i32>(
                "stage_enter_cost_amount",
                Self::VT_STAGE_ENTER_COST_AMOUNT,
                false,
            )?
            .visit_field::<i64>(
                "event_content_stage_reward_id",
                Self::VT_EVENT_CONTENT_STAGE_REWARD_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "enter_scenario_group_id",
                Self::VT_ENTER_SCENARIO_GROUP_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "clear_scenario_group_id",
                Self::VT_CLEAR_SCENARIO_GROUP_ID,
                false,
            )?
            .visit_field::<StageTopography>("stage_topography", Self::VT_STAGE_TOPOGRAPHY, false)?
            .visit_field::<i32>("recommand_level", Self::VT_RECOMMAND_LEVEL, false)?
            .visit_field::<i64>("ground_id", Self::VT_GROUND_ID, false)?
            .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, StarGoalType>>>(
                "star_goal",
                Self::VT_STAR_GOAL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "star_goal_amount",
                Self::VT_STAR_GOAL_AMOUNT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "defense_formation_bg_prefab",
                Self::VT_DEFENSE_FORMATION_BG_PREFAB,
                false,
            )?
            .visit_field::<f32>(
                "defense_formation_bg_prefab_scale",
                Self::VT_DEFENSE_FORMATION_BG_PREFAB_SCALE,
                false,
            )?
            .visit_field::<i64>("fixed_echelon", Self::VT_FIXED_ECHELON, false)?
            .visit_field::<i64>(
                "mininage_defense_fixed_stat_id",
                Self::VT_MININAGE_DEFENSE_FIXED_STAT_ID,
                false,
            )?
            .visit_field::<u32>("stage_hint", Self::VT_STAGE_HINT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDefenseStageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDefenseStageExcel", 27)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.name() {
            s.serialize_field("name", &f)?;
        } else {
            s.skip_field("name")?;
        }
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("stage_difficulty", &self.stage_difficulty())?;
        s.serialize_field(
            "stage_difficulty_localize",
            &self.stage_difficulty_localize(),
        )?;
        s.serialize_field("stage_number", &self.stage_number())?;
        s.serialize_field("stage_display", &self.stage_display())?;
        s.serialize_field("prev_stage_id", &self.prev_stage_id())?;
        s.serialize_field("echelon_extension_type", &self.echelon_extension_type())?;
        s.serialize_field("battle_duration", &self.battle_duration())?;
        s.serialize_field("stage_enter_cost_type", &self.stage_enter_cost_type())?;
        s.serialize_field("stage_enter_cost_id", &self.stage_enter_cost_id())?;
        s.serialize_field("stage_enter_cost_amount", &self.stage_enter_cost_amount())?;
        s.serialize_field(
            "event_content_stage_reward_id",
            &self.event_content_stage_reward_id(),
        )?;
        if let Some(f) = self.enter_scenario_group_id() {
            s.serialize_field("enter_scenario_group_id", &f)?;
        } else {
            s.skip_field("enter_scenario_group_id")?;
        }
        if let Some(f) = self.clear_scenario_group_id() {
            s.serialize_field("clear_scenario_group_id", &f)?;
        } else {
            s.skip_field("clear_scenario_group_id")?;
        }
        s.serialize_field("stage_topography", &self.stage_topography())?;
        s.serialize_field("recommand_level", &self.recommand_level())?;
        s.serialize_field("ground_id", &self.ground_id())?;
        s.serialize_field("content_type", &self.content_type())?;
        if let Some(f) = self.star_goal() {
            s.serialize_field("star_goal", &f)?;
        } else {
            s.skip_field("star_goal")?;
        }
        if let Some(f) = self.star_goal_amount() {
            s.serialize_field("star_goal_amount", &f)?;
        } else {
            s.skip_field("star_goal_amount")?;
        }
        if let Some(f) = self.defense_formation_bg_prefab() {
            s.serialize_field("defense_formation_bg_prefab", &f)?;
        } else {
            s.skip_field("defense_formation_bg_prefab")?;
        }
        s.serialize_field(
            "defense_formation_bg_prefab_scale",
            &self.defense_formation_bg_prefab_scale(),
        )?;
        s.serialize_field("fixed_echelon", &self.fixed_echelon())?;
        s.serialize_field(
            "mininage_defense_fixed_stat_id",
            &self.mininage_defense_fixed_stat_id(),
        )?;
        s.serialize_field("stage_hint", &self.stage_hint())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDefenseStageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDefenseStageExcel");
        ds.field("id", &self.id());
        ds.field("name", &self.name());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("stage_difficulty", &self.stage_difficulty());
        ds.field(
            "stage_difficulty_localize",
            &self.stage_difficulty_localize(),
        );
        ds.field("stage_number", &self.stage_number());
        ds.field("stage_display", &self.stage_display());
        ds.field("prev_stage_id", &self.prev_stage_id());
        ds.field("echelon_extension_type", &self.echelon_extension_type());
        ds.field("battle_duration", &self.battle_duration());
        ds.field("stage_enter_cost_type", &self.stage_enter_cost_type());
        ds.field("stage_enter_cost_id", &self.stage_enter_cost_id());
        ds.field("stage_enter_cost_amount", &self.stage_enter_cost_amount());
        ds.field(
            "event_content_stage_reward_id",
            &self.event_content_stage_reward_id(),
        );
        ds.field("enter_scenario_group_id", &self.enter_scenario_group_id());
        ds.field("clear_scenario_group_id", &self.clear_scenario_group_id());
        ds.field("stage_topography", &self.stage_topography());
        ds.field("recommand_level", &self.recommand_level());
        ds.field("ground_id", &self.ground_id());
        ds.field("content_type", &self.content_type());
        ds.field("star_goal", &self.star_goal());
        ds.field("star_goal_amount", &self.star_goal_amount());
        ds.field(
            "defense_formation_bg_prefab",
            &self.defense_formation_bg_prefab(),
        );
        ds.field(
            "defense_formation_bg_prefab_scale",
            &self.defense_formation_bg_prefab_scale(),
        );
        ds.field("fixed_echelon", &self.fixed_echelon());
        ds.field(
            "mininage_defense_fixed_stat_id",
            &self.mininage_defense_fixed_stat_id(),
        );
        ds.field("stage_hint", &self.stage_hint());
        ds.finish()
    }
}
