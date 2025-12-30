extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{EchelonExtensionType, ParcelType, SchoolDungeonType, StageTopography, StarGoalType};

#[derive(Copy, Clone, PartialEq)]
pub struct SchoolDungeonStageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for SchoolDungeonStageExcel<'a> {
    type Inner = SchoolDungeonStageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> SchoolDungeonStageExcel<'a> {
    pub const VT_STAGE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DUNGEON_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_DIFFICULTY: ::flatbuffers::VOffsetT = 8;
    pub const VT_BATTLE_DURATION: ::flatbuffers::VOffsetT = 10;
    pub const VT_PREV_STAGE_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_STAGE_ENTER_COST_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_STAGE_ENTER_COST_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_STAGE_ENTER_COST_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_STAGE_ENTER_COST_MINIMUM_AMOUNT: ::flatbuffers::VOffsetT = 20;
    pub const VT_GROUND_ID: ::flatbuffers::VOffsetT = 22;
    pub const VT_STAR_GOAL: ::flatbuffers::VOffsetT = 24;
    pub const VT_STAR_GOAL_AMOUNT: ::flatbuffers::VOffsetT = 26;
    pub const VT_STAGE_TOPOGRAPHY: ::flatbuffers::VOffsetT = 28;
    pub const VT_RECOMMAND_LEVEL: ::flatbuffers::VOffsetT = 30;
    pub const VT_STAGE_REWARD_ID: ::flatbuffers::VOffsetT = 32;
    pub const VT_PLAY_TIME_LIMIT_IN_SECONDS: ::flatbuffers::VOffsetT = 34;
    pub const VT_ECHELON_EXTENSION_TYPE: ::flatbuffers::VOffsetT = 36;

    #[inline]
    pub fn stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonStageExcel::VT_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dungeon_type(&self) -> SchoolDungeonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<SchoolDungeonType>(
                    SchoolDungeonStageExcel::VT_DUNGEON_TYPE,
                    Some(SchoolDungeonType::SchoolA),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn difficulty(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(SchoolDungeonStageExcel::VT_DIFFICULTY, Some(0))
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
                .get::<i64>(SchoolDungeonStageExcel::VT_BATTLE_DURATION, Some(0))
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
                .get::<i64>(SchoolDungeonStageExcel::VT_PREV_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_enter_cost_type(&self) -> Option<::flatbuffers::Vector<'a, ParcelType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, ParcelType>>>(
                    SchoolDungeonStageExcel::VT_STAGE_ENTER_COST_TYPE,
                    None,
                )
        }
    }
    #[inline]
    pub fn stage_enter_cost_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    SchoolDungeonStageExcel::VT_STAGE_ENTER_COST_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn stage_enter_cost_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    SchoolDungeonStageExcel::VT_STAGE_ENTER_COST_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn stage_enter_cost_minimum_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    SchoolDungeonStageExcel::VT_STAGE_ENTER_COST_MINIMUM_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn ground_id(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(SchoolDungeonStageExcel::VT_GROUND_ID, Some(0))
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
                    SchoolDungeonStageExcel::VT_STAR_GOAL,
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
                    SchoolDungeonStageExcel::VT_STAR_GOAL_AMOUNT,
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
                    SchoolDungeonStageExcel::VT_STAGE_TOPOGRAPHY,
                    Some(StageTopography::Street),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn recommand_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonStageExcel::VT_RECOMMAND_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonStageExcel::VT_STAGE_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn play_time_limit_in_seconds(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    SchoolDungeonStageExcel::VT_PLAY_TIME_LIMIT_IN_SECONDS,
                    Some(0),
                )
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
                    SchoolDungeonStageExcel::VT_ECHELON_EXTENSION_TYPE,
                    Some(EchelonExtensionType::Base),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for SchoolDungeonStageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("stage_id", Self::VT_STAGE_ID, false)?
            .visit_field::<SchoolDungeonType>("dungeon_type", Self::VT_DUNGEON_TYPE, false)?
            .visit_field::<i32>("difficulty", Self::VT_DIFFICULTY, false)?
            .visit_field::<i64>("battle_duration", Self::VT_BATTLE_DURATION, false)?
            .visit_field::<i64>("prev_stage_id", Self::VT_PREV_STAGE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ParcelType>>>(
                "stage_enter_cost_type",
                Self::VT_STAGE_ENTER_COST_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stage_enter_cost_id",
                Self::VT_STAGE_ENTER_COST_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stage_enter_cost_amount",
                Self::VT_STAGE_ENTER_COST_AMOUNT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stage_enter_cost_minimum_amount",
                Self::VT_STAGE_ENTER_COST_MINIMUM_AMOUNT,
                false,
            )?
            .visit_field::<i32>("ground_id", Self::VT_GROUND_ID, false)?
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
            .visit_field::<StageTopography>("stage_topography", Self::VT_STAGE_TOPOGRAPHY, false)?
            .visit_field::<i64>("recommand_level", Self::VT_RECOMMAND_LEVEL, false)?
            .visit_field::<i64>("stage_reward_id", Self::VT_STAGE_REWARD_ID, false)?
            .visit_field::<i64>(
                "play_time_limit_in_seconds",
                Self::VT_PLAY_TIME_LIMIT_IN_SECONDS,
                false,
            )?
            .visit_field::<EchelonExtensionType>(
                "echelon_extension_type",
                Self::VT_ECHELON_EXTENSION_TYPE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for SchoolDungeonStageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SchoolDungeonStageExcel", 17)?;
        s.serialize_field("stage_id", &self.stage_id())?;
        s.serialize_field("dungeon_type", &self.dungeon_type())?;
        s.serialize_field("difficulty", &self.difficulty())?;
        s.serialize_field("battle_duration", &self.battle_duration())?;
        s.serialize_field("prev_stage_id", &self.prev_stage_id())?;
        if let Some(f) = self.stage_enter_cost_type() {
            s.serialize_field("stage_enter_cost_type", &f)?;
        } else {
            s.skip_field("stage_enter_cost_type")?;
        }
        if let Some(f) = self.stage_enter_cost_id() {
            s.serialize_field("stage_enter_cost_id", &f)?;
        } else {
            s.skip_field("stage_enter_cost_id")?;
        }
        if let Some(f) = self.stage_enter_cost_amount() {
            s.serialize_field("stage_enter_cost_amount", &f)?;
        } else {
            s.skip_field("stage_enter_cost_amount")?;
        }
        if let Some(f) = self.stage_enter_cost_minimum_amount() {
            s.serialize_field("stage_enter_cost_minimum_amount", &f)?;
        } else {
            s.skip_field("stage_enter_cost_minimum_amount")?;
        }
        s.serialize_field("ground_id", &self.ground_id())?;
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
        s.serialize_field("stage_topography", &self.stage_topography())?;
        s.serialize_field("recommand_level", &self.recommand_level())?;
        s.serialize_field("stage_reward_id", &self.stage_reward_id())?;
        s.serialize_field(
            "play_time_limit_in_seconds",
            &self.play_time_limit_in_seconds(),
        )?;
        s.serialize_field("echelon_extension_type", &self.echelon_extension_type())?;
        s.end()
    }
}

impl ::core::fmt::Debug for SchoolDungeonStageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("SchoolDungeonStageExcel");
        ds.field("stage_id", &self.stage_id());
        ds.field("dungeon_type", &self.dungeon_type());
        ds.field("difficulty", &self.difficulty());
        ds.field("battle_duration", &self.battle_duration());
        ds.field("prev_stage_id", &self.prev_stage_id());
        ds.field("stage_enter_cost_type", &self.stage_enter_cost_type());
        ds.field("stage_enter_cost_id", &self.stage_enter_cost_id());
        ds.field("stage_enter_cost_amount", &self.stage_enter_cost_amount());
        ds.field(
            "stage_enter_cost_minimum_amount",
            &self.stage_enter_cost_minimum_amount(),
        );
        ds.field("ground_id", &self.ground_id());
        ds.field("star_goal", &self.star_goal());
        ds.field("star_goal_amount", &self.star_goal_amount());
        ds.field("stage_topography", &self.stage_topography());
        ds.field("recommand_level", &self.recommand_level());
        ds.field("stage_reward_id", &self.stage_reward_id());
        ds.field(
            "play_time_limit_in_seconds",
            &self.play_time_limit_in_seconds(),
        );
        ds.field("echelon_extension_type", &self.echelon_extension_type());
        ds.finish()
    }
}
