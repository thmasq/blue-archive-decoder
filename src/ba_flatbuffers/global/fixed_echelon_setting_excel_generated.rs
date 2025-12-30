extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct FixedEchelonSettingExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for FixedEchelonSettingExcel<'a> {
    type Inner = FixedEchelonSettingExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> FixedEchelonSettingExcel<'a> {
    pub const VT_FIXED_ECHELON_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ECHELON_SCENE_SKIP: ::flatbuffers::VOffsetT = 6;
    pub const VT_MAIN_LEADER_SLOT: ::flatbuffers::VOffsetT = 8;
    pub const VT_MAIN_CHARACTER_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_MAIN_LEVEL: ::flatbuffers::VOffsetT = 12;
    pub const VT_MAIN_GRADE: ::flatbuffers::VOffsetT = 14;
    pub const VT_MAIN_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 16;
    pub const VT_MAIN_NONE_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 18;
    pub const VT_MAIN_EQUIPMENT1_TIER: ::flatbuffers::VOffsetT = 20;
    pub const VT_MAIN_EQUIPMENT1_LEVEL: ::flatbuffers::VOffsetT = 22;
    pub const VT_MAIN_EQUIPMENT2_TIER: ::flatbuffers::VOffsetT = 24;
    pub const VT_MAIN_EQUIPMENT2_LEVEL: ::flatbuffers::VOffsetT = 26;
    pub const VT_MAIN_EQUIPMENT3_TIER: ::flatbuffers::VOffsetT = 28;
    pub const VT_MAIN_EQUIPMENT3_LEVEL: ::flatbuffers::VOffsetT = 30;
    pub const VT_MAIN_CHARACTER_WEAPON_GRADE: ::flatbuffers::VOffsetT = 32;
    pub const VT_MAIN_CHARACTER_WEAPON_LEVEL: ::flatbuffers::VOffsetT = 34;
    pub const VT_MAIN_CHARACTER_GEAR_TIER: ::flatbuffers::VOffsetT = 36;
    pub const VT_MAIN_CHARACTER_GEAR_LEVEL: ::flatbuffers::VOffsetT = 38;
    pub const VT_SUPPORT_CHARACTER_ID: ::flatbuffers::VOffsetT = 40;
    pub const VT_SUPPORT_LEVEL: ::flatbuffers::VOffsetT = 42;
    pub const VT_SUPPORT_GRADE: ::flatbuffers::VOffsetT = 44;
    pub const VT_SUPPORT_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 46;
    pub const VT_SUPPORT_NONE_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 48;
    pub const VT_SUPPORT_EQUIPMENT1_TIER: ::flatbuffers::VOffsetT = 50;
    pub const VT_SUPPORT_EQUIPMENT1_LEVEL: ::flatbuffers::VOffsetT = 52;
    pub const VT_SUPPORT_EQUIPMENT2_TIER: ::flatbuffers::VOffsetT = 54;
    pub const VT_SUPPORT_EQUIPMENT2_LEVEL: ::flatbuffers::VOffsetT = 56;
    pub const VT_SUPPORT_EQUIPMENT3_TIER: ::flatbuffers::VOffsetT = 58;
    pub const VT_SUPPORT_EQUIPMENT3_LEVEL: ::flatbuffers::VOffsetT = 60;
    pub const VT_SUPPORT_CHARACTER_WEAPON_GRADE: ::flatbuffers::VOffsetT = 62;
    pub const VT_SUPPORT_CHARACTER_WEAPON_LEVEL: ::flatbuffers::VOffsetT = 64;
    pub const VT_SUPPORT_CHARACTER_GEAR_TIER: ::flatbuffers::VOffsetT = 66;
    pub const VT_SUPPORT_CHARACTER_GEAR_LEVEL: ::flatbuffers::VOffsetT = 68;
    pub const VT_INTERACTION_TS_CHARACTER_ID: ::flatbuffers::VOffsetT = 70;

    #[inline]
    pub fn fixed_echelon_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FixedEchelonSettingExcel::VT_FIXED_ECHELON_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn echelon_scene_skip(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(FixedEchelonSettingExcel::VT_ECHELON_SCENE_SKIP, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn main_leader_slot(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(FixedEchelonSettingExcel::VT_MAIN_LEADER_SLOT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn main_character_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FixedEchelonSettingExcel::VT_MAIN_CHARACTER_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_grade(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_GRADE,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_ex_skill_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EX_SKILL_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_none_ex_skill_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_NONE_EX_SKILL_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment1_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT1_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment1_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT1_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment2_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT2_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment2_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT2_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment3_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT3_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_equipment3_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_EQUIPMENT3_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_character_weapon_grade(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_CHARACTER_WEAPON_GRADE,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_character_weapon_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_CHARACTER_WEAPON_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_character_gear_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_CHARACTER_GEAR_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn main_character_gear_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_MAIN_CHARACTER_GEAR_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_character_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_CHARACTER_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_grade(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_GRADE,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_ex_skill_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EX_SKILL_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_none_ex_skill_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_NONE_EX_SKILL_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment1_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT1_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment1_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT1_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment2_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT2_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment2_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT2_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment3_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT3_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_equipment3_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_EQUIPMENT3_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_character_weapon_grade(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_CHARACTER_WEAPON_GRADE,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_character_weapon_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_CHARACTER_WEAPON_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_character_gear_tier(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_CHARACTER_GEAR_TIER,
                    None,
                )
        }
    }
    #[inline]
    pub fn support_character_gear_level(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    FixedEchelonSettingExcel::VT_SUPPORT_CHARACTER_GEAR_LEVEL,
                    None,
                )
        }
    }
    #[inline]
    pub fn interaction_ts_character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    FixedEchelonSettingExcel::VT_INTERACTION_TS_CHARACTER_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for FixedEchelonSettingExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("fixed_echelon_id", Self::VT_FIXED_ECHELON_ID, false)?
            .visit_field::<bool>("echelon_scene_skip", Self::VT_ECHELON_SCENE_SKIP, false)?
            .visit_field::<i32>("main_leader_slot", Self::VT_MAIN_LEADER_SLOT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "main_character_id",
                Self::VT_MAIN_CHARACTER_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_level",
                Self::VT_MAIN_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_grade",
                Self::VT_MAIN_GRADE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_ex_skill_level",
                Self::VT_MAIN_EX_SKILL_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_none_ex_skill_level",
                Self::VT_MAIN_NONE_EX_SKILL_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment1_tier",
                Self::VT_MAIN_EQUIPMENT1_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment1_level",
                Self::VT_MAIN_EQUIPMENT1_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment2_tier",
                Self::VT_MAIN_EQUIPMENT2_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment2_level",
                Self::VT_MAIN_EQUIPMENT2_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment3_tier",
                Self::VT_MAIN_EQUIPMENT3_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_equipment3_level",
                Self::VT_MAIN_EQUIPMENT3_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_character_weapon_grade",
                Self::VT_MAIN_CHARACTER_WEAPON_GRADE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_character_weapon_level",
                Self::VT_MAIN_CHARACTER_WEAPON_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_character_gear_tier",
                Self::VT_MAIN_CHARACTER_GEAR_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "main_character_gear_level",
                Self::VT_MAIN_CHARACTER_GEAR_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "support_character_id",
                Self::VT_SUPPORT_CHARACTER_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_level",
                Self::VT_SUPPORT_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_grade",
                Self::VT_SUPPORT_GRADE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_ex_skill_level",
                Self::VT_SUPPORT_EX_SKILL_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_none_ex_skill_level",
                Self::VT_SUPPORT_NONE_EX_SKILL_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment1_tier",
                Self::VT_SUPPORT_EQUIPMENT1_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment1_level",
                Self::VT_SUPPORT_EQUIPMENT1_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment2_tier",
                Self::VT_SUPPORT_EQUIPMENT2_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment2_level",
                Self::VT_SUPPORT_EQUIPMENT2_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment3_tier",
                Self::VT_SUPPORT_EQUIPMENT3_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_equipment3_level",
                Self::VT_SUPPORT_EQUIPMENT3_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_character_weapon_grade",
                Self::VT_SUPPORT_CHARACTER_WEAPON_GRADE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_character_weapon_level",
                Self::VT_SUPPORT_CHARACTER_WEAPON_LEVEL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_character_gear_tier",
                Self::VT_SUPPORT_CHARACTER_GEAR_TIER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "support_character_gear_level",
                Self::VT_SUPPORT_CHARACTER_GEAR_LEVEL,
                false,
            )?
            .visit_field::<i64>(
                "interaction_ts_character_id",
                Self::VT_INTERACTION_TS_CHARACTER_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for FixedEchelonSettingExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FixedEchelonSettingExcel", 34)?;
        s.serialize_field("fixed_echelon_id", &self.fixed_echelon_id())?;
        s.serialize_field("echelon_scene_skip", &self.echelon_scene_skip())?;
        s.serialize_field("main_leader_slot", &self.main_leader_slot())?;
        if let Some(f) = self.main_character_id() {
            s.serialize_field("main_character_id", &f)?;
        } else {
            s.skip_field("main_character_id")?;
        }
        if let Some(f) = self.main_level() {
            s.serialize_field("main_level", &f)?;
        } else {
            s.skip_field("main_level")?;
        }
        if let Some(f) = self.main_grade() {
            s.serialize_field("main_grade", &f)?;
        } else {
            s.skip_field("main_grade")?;
        }
        if let Some(f) = self.main_ex_skill_level() {
            s.serialize_field("main_ex_skill_level", &f)?;
        } else {
            s.skip_field("main_ex_skill_level")?;
        }
        if let Some(f) = self.main_none_ex_skill_level() {
            s.serialize_field("main_none_ex_skill_level", &f)?;
        } else {
            s.skip_field("main_none_ex_skill_level")?;
        }
        if let Some(f) = self.main_equipment1_tier() {
            s.serialize_field("main_equipment1_tier", &f)?;
        } else {
            s.skip_field("main_equipment1_tier")?;
        }
        if let Some(f) = self.main_equipment1_level() {
            s.serialize_field("main_equipment1_level", &f)?;
        } else {
            s.skip_field("main_equipment1_level")?;
        }
        if let Some(f) = self.main_equipment2_tier() {
            s.serialize_field("main_equipment2_tier", &f)?;
        } else {
            s.skip_field("main_equipment2_tier")?;
        }
        if let Some(f) = self.main_equipment2_level() {
            s.serialize_field("main_equipment2_level", &f)?;
        } else {
            s.skip_field("main_equipment2_level")?;
        }
        if let Some(f) = self.main_equipment3_tier() {
            s.serialize_field("main_equipment3_tier", &f)?;
        } else {
            s.skip_field("main_equipment3_tier")?;
        }
        if let Some(f) = self.main_equipment3_level() {
            s.serialize_field("main_equipment3_level", &f)?;
        } else {
            s.skip_field("main_equipment3_level")?;
        }
        if let Some(f) = self.main_character_weapon_grade() {
            s.serialize_field("main_character_weapon_grade", &f)?;
        } else {
            s.skip_field("main_character_weapon_grade")?;
        }
        if let Some(f) = self.main_character_weapon_level() {
            s.serialize_field("main_character_weapon_level", &f)?;
        } else {
            s.skip_field("main_character_weapon_level")?;
        }
        if let Some(f) = self.main_character_gear_tier() {
            s.serialize_field("main_character_gear_tier", &f)?;
        } else {
            s.skip_field("main_character_gear_tier")?;
        }
        if let Some(f) = self.main_character_gear_level() {
            s.serialize_field("main_character_gear_level", &f)?;
        } else {
            s.skip_field("main_character_gear_level")?;
        }
        if let Some(f) = self.support_character_id() {
            s.serialize_field("support_character_id", &f)?;
        } else {
            s.skip_field("support_character_id")?;
        }
        if let Some(f) = self.support_level() {
            s.serialize_field("support_level", &f)?;
        } else {
            s.skip_field("support_level")?;
        }
        if let Some(f) = self.support_grade() {
            s.serialize_field("support_grade", &f)?;
        } else {
            s.skip_field("support_grade")?;
        }
        if let Some(f) = self.support_ex_skill_level() {
            s.serialize_field("support_ex_skill_level", &f)?;
        } else {
            s.skip_field("support_ex_skill_level")?;
        }
        if let Some(f) = self.support_none_ex_skill_level() {
            s.serialize_field("support_none_ex_skill_level", &f)?;
        } else {
            s.skip_field("support_none_ex_skill_level")?;
        }
        if let Some(f) = self.support_equipment1_tier() {
            s.serialize_field("support_equipment1_tier", &f)?;
        } else {
            s.skip_field("support_equipment1_tier")?;
        }
        if let Some(f) = self.support_equipment1_level() {
            s.serialize_field("support_equipment1_level", &f)?;
        } else {
            s.skip_field("support_equipment1_level")?;
        }
        if let Some(f) = self.support_equipment2_tier() {
            s.serialize_field("support_equipment2_tier", &f)?;
        } else {
            s.skip_field("support_equipment2_tier")?;
        }
        if let Some(f) = self.support_equipment2_level() {
            s.serialize_field("support_equipment2_level", &f)?;
        } else {
            s.skip_field("support_equipment2_level")?;
        }
        if let Some(f) = self.support_equipment3_tier() {
            s.serialize_field("support_equipment3_tier", &f)?;
        } else {
            s.skip_field("support_equipment3_tier")?;
        }
        if let Some(f) = self.support_equipment3_level() {
            s.serialize_field("support_equipment3_level", &f)?;
        } else {
            s.skip_field("support_equipment3_level")?;
        }
        if let Some(f) = self.support_character_weapon_grade() {
            s.serialize_field("support_character_weapon_grade", &f)?;
        } else {
            s.skip_field("support_character_weapon_grade")?;
        }
        if let Some(f) = self.support_character_weapon_level() {
            s.serialize_field("support_character_weapon_level", &f)?;
        } else {
            s.skip_field("support_character_weapon_level")?;
        }
        if let Some(f) = self.support_character_gear_tier() {
            s.serialize_field("support_character_gear_tier", &f)?;
        } else {
            s.skip_field("support_character_gear_tier")?;
        }
        if let Some(f) = self.support_character_gear_level() {
            s.serialize_field("support_character_gear_level", &f)?;
        } else {
            s.skip_field("support_character_gear_level")?;
        }
        s.serialize_field(
            "interaction_ts_character_id",
            &self.interaction_ts_character_id(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for FixedEchelonSettingExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("FixedEchelonSettingExcel");
        ds.field("fixed_echelon_id", &self.fixed_echelon_id());
        ds.field("echelon_scene_skip", &self.echelon_scene_skip());
        ds.field("main_leader_slot", &self.main_leader_slot());
        ds.field("main_character_id", &self.main_character_id());
        ds.field("main_level", &self.main_level());
        ds.field("main_grade", &self.main_grade());
        ds.field("main_ex_skill_level", &self.main_ex_skill_level());
        ds.field("main_none_ex_skill_level", &self.main_none_ex_skill_level());
        ds.field("main_equipment1_tier", &self.main_equipment1_tier());
        ds.field("main_equipment1_level", &self.main_equipment1_level());
        ds.field("main_equipment2_tier", &self.main_equipment2_tier());
        ds.field("main_equipment2_level", &self.main_equipment2_level());
        ds.field("main_equipment3_tier", &self.main_equipment3_tier());
        ds.field("main_equipment3_level", &self.main_equipment3_level());
        ds.field(
            "main_character_weapon_grade",
            &self.main_character_weapon_grade(),
        );
        ds.field(
            "main_character_weapon_level",
            &self.main_character_weapon_level(),
        );
        ds.field("main_character_gear_tier", &self.main_character_gear_tier());
        ds.field(
            "main_character_gear_level",
            &self.main_character_gear_level(),
        );
        ds.field("support_character_id", &self.support_character_id());
        ds.field("support_level", &self.support_level());
        ds.field("support_grade", &self.support_grade());
        ds.field("support_ex_skill_level", &self.support_ex_skill_level());
        ds.field(
            "support_none_ex_skill_level",
            &self.support_none_ex_skill_level(),
        );
        ds.field("support_equipment1_tier", &self.support_equipment1_tier());
        ds.field("support_equipment1_level", &self.support_equipment1_level());
        ds.field("support_equipment2_tier", &self.support_equipment2_tier());
        ds.field("support_equipment2_level", &self.support_equipment2_level());
        ds.field("support_equipment3_tier", &self.support_equipment3_tier());
        ds.field("support_equipment3_level", &self.support_equipment3_level());
        ds.field(
            "support_character_weapon_grade",
            &self.support_character_weapon_grade(),
        );
        ds.field(
            "support_character_weapon_level",
            &self.support_character_weapon_level(),
        );
        ds.field(
            "support_character_gear_tier",
            &self.support_character_gear_tier(),
        );
        ds.field(
            "support_character_gear_level",
            &self.support_character_gear_level(),
        );
        ds.field(
            "interaction_ts_character_id",
            &self.interaction_ts_character_id(),
        );
        ds.finish()
    }
}
