extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::EchelonExtensionType;

#[derive(Copy, Clone, PartialEq)]
pub struct MultiFloorRaidStageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MultiFloorRaidStageExcel<'a> {
    type Inner = MultiFloorRaidStageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MultiFloorRaidStageExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ECHELON_EXTENSION_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_BOSS_GROUP_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_ASSIST_SLOT: ::flatbuffers::VOffsetT = 10;
    pub const VT_STAGE_OPEN_CONDITION: ::flatbuffers::VOffsetT = 12;
    pub const VT_FLOOR_LIST_SECTION: ::flatbuffers::VOffsetT = 14;
    pub const VT_FLOOR_LIST_SECTION_OPEN_CONDITION: ::flatbuffers::VOffsetT = 16;
    pub const VT_FLOOR_LIST_SECTION_LABEL: ::flatbuffers::VOffsetT = 18;
    pub const VT_DIFFICULTY: ::flatbuffers::VOffsetT = 20;
    pub const VT_USE_BOSS_INDEX: ::flatbuffers::VOffsetT = 22;
    pub const VT_USE_BOSS_AI_PHASE_SYNC: ::flatbuffers::VOffsetT = 24;
    pub const VT_FLOOR_LIST_IMG_PATH: ::flatbuffers::VOffsetT = 26;
    pub const VT_FLOOR_IMG_PATH: ::flatbuffers::VOffsetT = 28;
    pub const VT_RAID_CHARACTER_ID: ::flatbuffers::VOffsetT = 30;
    pub const VT_BOSS_CHARACTER_ID: ::flatbuffers::VOffsetT = 32;
    pub const VT_STAT_CHANGE_ID: ::flatbuffers::VOffsetT = 34;
    pub const VT_BATTLE_DURATION: ::flatbuffers::VOffsetT = 36;
    pub const VT_GROUND_ID: ::flatbuffers::VOffsetT = 38;
    pub const VT_RECOMMEND_LEVEL: ::flatbuffers::VOffsetT = 40;
    pub const VT_REWARD_GROUP_ID: ::flatbuffers::VOffsetT = 42;
    pub const VT_BATTLE_READY_TIMELINE_PATH: ::flatbuffers::VOffsetT = 44;
    pub const VT_BATTLE_READY_TIMELINE_PHASE_START: ::flatbuffers::VOffsetT = 46;
    pub const VT_BATTLE_READY_TIMELINE_PHASE_END: ::flatbuffers::VOffsetT = 48;
    pub const VT_VICTORY_TIMELINE_PATH: ::flatbuffers::VOffsetT = 50;
    pub const VT_SHOW_SKILL_CARD: ::flatbuffers::VOffsetT = 52;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_ID, Some(0))
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
                    MultiFloorRaidStageExcel::VT_ECHELON_EXTENSION_TYPE,
                    Some(EchelonExtensionType::Base),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn boss_group_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidStageExcel::VT_BOSS_GROUP_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn assist_slot(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MultiFloorRaidStageExcel::VT_ASSIST_SLOT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_open_condition(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_STAGE_OPEN_CONDITION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn floor_list_section(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(MultiFloorRaidStageExcel::VT_FLOOR_LIST_SECTION, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn floor_list_section_open_condition(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MultiFloorRaidStageExcel::VT_FLOOR_LIST_SECTION_OPEN_CONDITION,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn floor_list_section_label(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MultiFloorRaidStageExcel::VT_FLOOR_LIST_SECTION_LABEL,
                    Some(0),
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
                .get::<i32>(MultiFloorRaidStageExcel::VT_DIFFICULTY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn use_boss_index(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(MultiFloorRaidStageExcel::VT_USE_BOSS_INDEX, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn use_boss_ai_phase_sync(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MultiFloorRaidStageExcel::VT_USE_BOSS_AI_PHASE_SYNC,
                    Some(false),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn floor_list_img_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidStageExcel::VT_FLOOR_LIST_IMG_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn floor_img_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidStageExcel::VT_FLOOR_IMG_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn raid_character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_RAID_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn boss_character_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MultiFloorRaidStageExcel::VT_BOSS_CHARACTER_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn stat_change_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MultiFloorRaidStageExcel::VT_STAT_CHANGE_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn battle_duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_BATTLE_DURATION, Some(0))
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
                .get::<i64>(MultiFloorRaidStageExcel::VT_GROUND_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn recommend_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_RECOMMEND_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStageExcel::VT_REWARD_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn battle_ready_timeline_path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(
                MultiFloorRaidStageExcel::VT_BATTLE_READY_TIMELINE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn battle_ready_timeline_phase_start(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    MultiFloorRaidStageExcel::VT_BATTLE_READY_TIMELINE_PHASE_START,
                    None,
                )
        }
    }
    #[inline]
    pub fn battle_ready_timeline_phase_end(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    MultiFloorRaidStageExcel::VT_BATTLE_READY_TIMELINE_PHASE_END,
                    None,
                )
        }
    }
    #[inline]
    pub fn victory_timeline_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidStageExcel::VT_VICTORY_TIMELINE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn show_skill_card(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(MultiFloorRaidStageExcel::VT_SHOW_SKILL_CARD, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MultiFloorRaidStageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<EchelonExtensionType>(
                "echelon_extension_type",
                Self::VT_ECHELON_EXTENSION_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "boss_group_id",
                Self::VT_BOSS_GROUP_ID,
                false,
            )?
            .visit_field::<i32>("assist_slot", Self::VT_ASSIST_SLOT, false)?
            .visit_field::<i64>("stage_open_condition", Self::VT_STAGE_OPEN_CONDITION, false)?
            .visit_field::<bool>("floor_list_section", Self::VT_FLOOR_LIST_SECTION, false)?
            .visit_field::<i64>(
                "floor_list_section_open_condition",
                Self::VT_FLOOR_LIST_SECTION_OPEN_CONDITION,
                false,
            )?
            .visit_field::<u32>(
                "floor_list_section_label",
                Self::VT_FLOOR_LIST_SECTION_LABEL,
                false,
            )?
            .visit_field::<i32>("difficulty", Self::VT_DIFFICULTY, false)?
            .visit_field::<bool>("use_boss_index", Self::VT_USE_BOSS_INDEX, false)?
            .visit_field::<bool>(
                "use_boss_ai_phase_sync",
                Self::VT_USE_BOSS_AI_PHASE_SYNC,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "floor_list_img_path",
                Self::VT_FLOOR_LIST_IMG_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "floor_img_path",
                Self::VT_FLOOR_IMG_PATH,
                false,
            )?
            .visit_field::<i64>("raid_character_id", Self::VT_RAID_CHARACTER_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "boss_character_id",
                Self::VT_BOSS_CHARACTER_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stat_change_id",
                Self::VT_STAT_CHANGE_ID,
                false,
            )?
            .visit_field::<i64>("battle_duration", Self::VT_BATTLE_DURATION, false)?
            .visit_field::<i64>("ground_id", Self::VT_GROUND_ID, false)?
            .visit_field::<i64>("recommend_level", Self::VT_RECOMMEND_LEVEL, false)?
            .visit_field::<i64>("reward_group_id", Self::VT_REWARD_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>(
                "battle_ready_timeline_path",
                Self::VT_BATTLE_READY_TIMELINE_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "battle_ready_timeline_phase_start",
                Self::VT_BATTLE_READY_TIMELINE_PHASE_START,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "battle_ready_timeline_phase_end",
                Self::VT_BATTLE_READY_TIMELINE_PHASE_END,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "victory_timeline_path",
                Self::VT_VICTORY_TIMELINE_PATH,
                false,
            )?
            .visit_field::<bool>("show_skill_card", Self::VT_SHOW_SKILL_CARD, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MultiFloorRaidStageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MultiFloorRaidStageExcel", 25)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("echelon_extension_type", &self.echelon_extension_type())?;
        if let Some(f) = self.boss_group_id() {
            s.serialize_field("boss_group_id", &f)?;
        } else {
            s.skip_field("boss_group_id")?;
        }
        s.serialize_field("assist_slot", &self.assist_slot())?;
        s.serialize_field("stage_open_condition", &self.stage_open_condition())?;
        s.serialize_field("floor_list_section", &self.floor_list_section())?;
        s.serialize_field(
            "floor_list_section_open_condition",
            &self.floor_list_section_open_condition(),
        )?;
        s.serialize_field("floor_list_section_label", &self.floor_list_section_label())?;
        s.serialize_field("difficulty", &self.difficulty())?;
        s.serialize_field("use_boss_index", &self.use_boss_index())?;
        s.serialize_field("use_boss_ai_phase_sync", &self.use_boss_ai_phase_sync())?;
        if let Some(f) = self.floor_list_img_path() {
            s.serialize_field("floor_list_img_path", &f)?;
        } else {
            s.skip_field("floor_list_img_path")?;
        }
        if let Some(f) = self.floor_img_path() {
            s.serialize_field("floor_img_path", &f)?;
        } else {
            s.skip_field("floor_img_path")?;
        }
        s.serialize_field("raid_character_id", &self.raid_character_id())?;
        if let Some(f) = self.boss_character_id() {
            s.serialize_field("boss_character_id", &f)?;
        } else {
            s.skip_field("boss_character_id")?;
        }
        if let Some(f) = self.stat_change_id() {
            s.serialize_field("stat_change_id", &f)?;
        } else {
            s.skip_field("stat_change_id")?;
        }
        s.serialize_field("battle_duration", &self.battle_duration())?;
        s.serialize_field("ground_id", &self.ground_id())?;
        s.serialize_field("recommend_level", &self.recommend_level())?;
        s.serialize_field("reward_group_id", &self.reward_group_id())?;
        if let Some(f) = self.battle_ready_timeline_path() {
            s.serialize_field("battle_ready_timeline_path", &f)?;
        } else {
            s.skip_field("battle_ready_timeline_path")?;
        }
        if let Some(f) = self.battle_ready_timeline_phase_start() {
            s.serialize_field("battle_ready_timeline_phase_start", &f)?;
        } else {
            s.skip_field("battle_ready_timeline_phase_start")?;
        }
        if let Some(f) = self.battle_ready_timeline_phase_end() {
            s.serialize_field("battle_ready_timeline_phase_end", &f)?;
        } else {
            s.skip_field("battle_ready_timeline_phase_end")?;
        }
        if let Some(f) = self.victory_timeline_path() {
            s.serialize_field("victory_timeline_path", &f)?;
        } else {
            s.skip_field("victory_timeline_path")?;
        }
        s.serialize_field("show_skill_card", &self.show_skill_card())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MultiFloorRaidStageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MultiFloorRaidStageExcel");
        ds.field("id", &self.id());
        ds.field("echelon_extension_type", &self.echelon_extension_type());
        ds.field("boss_group_id", &self.boss_group_id());
        ds.field("assist_slot", &self.assist_slot());
        ds.field("stage_open_condition", &self.stage_open_condition());
        ds.field("floor_list_section", &self.floor_list_section());
        ds.field(
            "floor_list_section_open_condition",
            &self.floor_list_section_open_condition(),
        );
        ds.field("floor_list_section_label", &self.floor_list_section_label());
        ds.field("difficulty", &self.difficulty());
        ds.field("use_boss_index", &self.use_boss_index());
        ds.field("use_boss_ai_phase_sync", &self.use_boss_ai_phase_sync());
        ds.field("floor_list_img_path", &self.floor_list_img_path());
        ds.field("floor_img_path", &self.floor_img_path());
        ds.field("raid_character_id", &self.raid_character_id());
        ds.field("boss_character_id", &self.boss_character_id());
        ds.field("stat_change_id", &self.stat_change_id());
        ds.field("battle_duration", &self.battle_duration());
        ds.field("ground_id", &self.ground_id());
        ds.field("recommend_level", &self.recommend_level());
        ds.field("reward_group_id", &self.reward_group_id());
        ds.field(
            "battle_ready_timeline_path",
            &self.battle_ready_timeline_path(),
        );
        ds.field(
            "battle_ready_timeline_phase_start",
            &self.battle_ready_timeline_phase_start(),
        );
        ds.field(
            "battle_ready_timeline_phase_end",
            &self.battle_ready_timeline_phase_end(),
        );
        ds.field("victory_timeline_path", &self.victory_timeline_path());
        ds.field("show_skill_card", &self.show_skill_card());
        ds.finish()
    }
}
