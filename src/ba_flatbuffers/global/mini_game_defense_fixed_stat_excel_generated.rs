extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDefenseFixedStatExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDefenseFixedStatExcel<'a> {
    type Inner = MiniGameDefenseFixedStatExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDefenseFixedStatExcel<'a> {
    pub const VT_MINIGAME_DEFENSE_FIXED_STAT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LEVEL: ::flatbuffers::VOffsetT = 6;
    pub const VT_GRADE: ::flatbuffers::VOffsetT = 8;
    pub const VT_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 10;
    pub const VT_NONE_EX_SKILL_LEVEL: ::flatbuffers::VOffsetT = 12;
    pub const VT_EQUIPMENT1_TIER: ::flatbuffers::VOffsetT = 14;
    pub const VT_EQUIPMENT1_LEVEL: ::flatbuffers::VOffsetT = 16;
    pub const VT_EQUIPMENT2_TIER: ::flatbuffers::VOffsetT = 18;
    pub const VT_EQUIPMENT2_LEVEL: ::flatbuffers::VOffsetT = 20;
    pub const VT_EQUIPMENT3_TIER: ::flatbuffers::VOffsetT = 22;
    pub const VT_EQUIPMENT3_LEVEL: ::flatbuffers::VOffsetT = 24;
    pub const VT_CHARACTER_WEAPON_GRADE: ::flatbuffers::VOffsetT = 26;
    pub const VT_CHARACTER_WEAPON_LEVEL: ::flatbuffers::VOffsetT = 28;
    pub const VT_CHARACTER_GEAR_TIER: ::flatbuffers::VOffsetT = 30;
    pub const VT_CHARACTER_GEAR_LEVEL: ::flatbuffers::VOffsetT = 32;

    #[inline]
    pub fn minigame_defense_fixed_stat_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseFixedStatExcel::VT_MINIGAME_DEFENSE_FIXED_STAT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn grade(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_GRADE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ex_skill_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EX_SKILL_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn none_ex_skill_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseFixedStatExcel::VT_NONE_EX_SKILL_LEVEL,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment1_tier(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT1_TIER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment1_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT1_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment2_tier(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT2_TIER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment2_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT2_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment3_tier(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT3_TIER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn equipment3_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDefenseFixedStatExcel::VT_EQUIPMENT3_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn character_weapon_grade(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseFixedStatExcel::VT_CHARACTER_WEAPON_GRADE,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn character_weapon_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseFixedStatExcel::VT_CHARACTER_WEAPON_LEVEL,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn character_gear_tier(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseFixedStatExcel::VT_CHARACTER_GEAR_TIER,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn character_gear_level(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(
                    MiniGameDefenseFixedStatExcel::VT_CHARACTER_GEAR_LEVEL,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDefenseFixedStatExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "minigame_defense_fixed_stat_id",
                Self::VT_MINIGAME_DEFENSE_FIXED_STAT_ID,
                false,
            )?
            .visit_field::<i32>("level", Self::VT_LEVEL, false)?
            .visit_field::<i32>("grade", Self::VT_GRADE, false)?
            .visit_field::<i32>("ex_skill_level", Self::VT_EX_SKILL_LEVEL, false)?
            .visit_field::<i32>("none_ex_skill_level", Self::VT_NONE_EX_SKILL_LEVEL, false)?
            .visit_field::<i32>("equipment1_tier", Self::VT_EQUIPMENT1_TIER, false)?
            .visit_field::<i32>("equipment1_level", Self::VT_EQUIPMENT1_LEVEL, false)?
            .visit_field::<i32>("equipment2_tier", Self::VT_EQUIPMENT2_TIER, false)?
            .visit_field::<i32>("equipment2_level", Self::VT_EQUIPMENT2_LEVEL, false)?
            .visit_field::<i32>("equipment3_tier", Self::VT_EQUIPMENT3_TIER, false)?
            .visit_field::<i32>("equipment3_level", Self::VT_EQUIPMENT3_LEVEL, false)?
            .visit_field::<i32>(
                "character_weapon_grade",
                Self::VT_CHARACTER_WEAPON_GRADE,
                false,
            )?
            .visit_field::<i32>(
                "character_weapon_level",
                Self::VT_CHARACTER_WEAPON_LEVEL,
                false,
            )?
            .visit_field::<i32>("character_gear_tier", Self::VT_CHARACTER_GEAR_TIER, false)?
            .visit_field::<i32>("character_gear_level", Self::VT_CHARACTER_GEAR_LEVEL, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDefenseFixedStatExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDefenseFixedStatExcel", 15)?;
        s.serialize_field(
            "minigame_defense_fixed_stat_id",
            &self.minigame_defense_fixed_stat_id(),
        )?;
        s.serialize_field("level", &self.level())?;
        s.serialize_field("grade", &self.grade())?;
        s.serialize_field("ex_skill_level", &self.ex_skill_level())?;
        s.serialize_field("none_ex_skill_level", &self.none_ex_skill_level())?;
        s.serialize_field("equipment1_tier", &self.equipment1_tier())?;
        s.serialize_field("equipment1_level", &self.equipment1_level())?;
        s.serialize_field("equipment2_tier", &self.equipment2_tier())?;
        s.serialize_field("equipment2_level", &self.equipment2_level())?;
        s.serialize_field("equipment3_tier", &self.equipment3_tier())?;
        s.serialize_field("equipment3_level", &self.equipment3_level())?;
        s.serialize_field("character_weapon_grade", &self.character_weapon_grade())?;
        s.serialize_field("character_weapon_level", &self.character_weapon_level())?;
        s.serialize_field("character_gear_tier", &self.character_gear_tier())?;
        s.serialize_field("character_gear_level", &self.character_gear_level())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDefenseFixedStatExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDefenseFixedStatExcel");
        ds.field(
            "minigame_defense_fixed_stat_id",
            &self.minigame_defense_fixed_stat_id(),
        );
        ds.field("level", &self.level());
        ds.field("grade", &self.grade());
        ds.field("ex_skill_level", &self.ex_skill_level());
        ds.field("none_ex_skill_level", &self.none_ex_skill_level());
        ds.field("equipment1_tier", &self.equipment1_tier());
        ds.field("equipment1_level", &self.equipment1_level());
        ds.field("equipment2_tier", &self.equipment2_tier());
        ds.field("equipment2_level", &self.equipment2_level());
        ds.field("equipment3_tier", &self.equipment3_tier());
        ds.field("equipment3_level", &self.equipment3_level());
        ds.field("character_weapon_grade", &self.character_weapon_grade());
        ds.field("character_weapon_level", &self.character_weapon_level());
        ds.field("character_gear_tier", &self.character_gear_tier());
        ds.field("character_gear_level", &self.character_gear_level());
        ds.finish()
    }
}
