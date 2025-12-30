extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{EquipmentOptionType, StatLevelUpType, Tag};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterGearExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterGearExcel<'a> {
    type Inner = CharacterGearExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterGearExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_STAT_LEVEL_UP_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_TIER: ::flatbuffers::VOffsetT = 10;
    pub const VT_NEXT_TIER_EQUIPMENT: ::flatbuffers::VOffsetT = 12;
    pub const VT_RECIPE_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_OPEN_FAVOR_LEVEL: ::flatbuffers::VOffsetT = 16;
    pub const VT_MAX_LEVEL: ::flatbuffers::VOffsetT = 18;
    pub const VT_LEARN_SKILL_SLOT: ::flatbuffers::VOffsetT = 20;
    pub const VT_STAT_TYPE: ::flatbuffers::VOffsetT = 22;
    pub const VT_MIN_STAT_VALUE: ::flatbuffers::VOffsetT = 24;
    pub const VT_MAX_STAT_VALUE: ::flatbuffers::VOffsetT = 26;
    pub const VT_ICON: ::flatbuffers::VOffsetT = 28;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 30;
    pub const VT_TAGS: ::flatbuffers::VOffsetT = 32;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_ID, Some(0))
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
                .get::<i64>(CharacterGearExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stat_level_up_type(&self) -> StatLevelUpType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StatLevelUpType>(
                    CharacterGearExcel::VT_STAT_LEVEL_UP_TYPE,
                    Some(StatLevelUpType::Standard),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn tier(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_TIER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn next_tier_equipment(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_NEXT_TIER_EQUIPMENT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn recipe_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_RECIPE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn open_favor_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_OPEN_FAVOR_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn max_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterGearExcel::VT_MAX_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn learn_skill_slot(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterGearExcel::VT_LEARN_SKILL_SLOT,
                None,
            )
        }
    }
    #[inline]
    pub fn stat_type(&self) -> Option<::flatbuffers::Vector<'a, EquipmentOptionType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, EquipmentOptionType>>>(CharacterGearExcel::VT_STAT_TYPE, None)
        }
    }
    #[inline]
    pub fn min_stat_value(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CharacterGearExcel::VT_MIN_STAT_VALUE,
                    None,
                )
        }
    }
    #[inline]
    pub fn max_stat_value(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CharacterGearExcel::VT_MAX_STAT_VALUE,
                    None,
                )
        }
    }
    #[inline]
    pub fn icon(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(CharacterGearExcel::VT_ICON, None)
        }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(CharacterGearExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn tags(&self) -> Option<::flatbuffers::Vector<'a, Tag>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Tag>>>(
                    CharacterGearExcel::VT_TAGS,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterGearExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
     .visit_field::<StatLevelUpType>("stat_level_up_type", Self::VT_STAT_LEVEL_UP_TYPE, false)?
     .visit_field::<i64>("tier", Self::VT_TIER, false)?
     .visit_field::<i64>("next_tier_equipment", Self::VT_NEXT_TIER_EQUIPMENT, false)?
     .visit_field::<i64>("recipe_id", Self::VT_RECIPE_ID, false)?
     .visit_field::<i64>("open_favor_level", Self::VT_OPEN_FAVOR_LEVEL, false)?
     .visit_field::<i64>("max_level", Self::VT_MAX_LEVEL, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("learn_skill_slot", Self::VT_LEARN_SKILL_SLOT, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, EquipmentOptionType>>>("stat_type", Self::VT_STAT_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("min_stat_value", Self::VT_MIN_STAT_VALUE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("max_stat_value", Self::VT_MAX_STAT_VALUE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("icon", Self::VT_ICON, false)?
     .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, Tag>>>("tags", Self::VT_TAGS, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for CharacterGearExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterGearExcel", 15)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("character_id", &self.character_id())?;
        s.serialize_field("stat_level_up_type", &self.stat_level_up_type())?;
        s.serialize_field("tier", &self.tier())?;
        s.serialize_field("next_tier_equipment", &self.next_tier_equipment())?;
        s.serialize_field("recipe_id", &self.recipe_id())?;
        s.serialize_field("open_favor_level", &self.open_favor_level())?;
        s.serialize_field("max_level", &self.max_level())?;
        if let Some(f) = self.learn_skill_slot() {
            s.serialize_field("learn_skill_slot", &f)?;
        } else {
            s.skip_field("learn_skill_slot")?;
        }
        if let Some(f) = self.stat_type() {
            s.serialize_field("stat_type", &f)?;
        } else {
            s.skip_field("stat_type")?;
        }
        if let Some(f) = self.min_stat_value() {
            s.serialize_field("min_stat_value", &f)?;
        } else {
            s.skip_field("min_stat_value")?;
        }
        if let Some(f) = self.max_stat_value() {
            s.serialize_field("max_stat_value", &f)?;
        } else {
            s.skip_field("max_stat_value")?;
        }
        if let Some(f) = self.icon() {
            s.serialize_field("icon", &f)?;
        } else {
            s.skip_field("icon")?;
        }
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.tags() {
            s.serialize_field("tags", &f)?;
        } else {
            s.skip_field("tags")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterGearExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterGearExcel");
        ds.field("id", &self.id());
        ds.field("character_id", &self.character_id());
        ds.field("stat_level_up_type", &self.stat_level_up_type());
        ds.field("tier", &self.tier());
        ds.field("next_tier_equipment", &self.next_tier_equipment());
        ds.field("recipe_id", &self.recipe_id());
        ds.field("open_favor_level", &self.open_favor_level());
        ds.field("max_level", &self.max_level());
        ds.field("learn_skill_slot", &self.learn_skill_slot());
        ds.field("stat_type", &self.stat_type());
        ds.field("min_stat_value", &self.min_stat_value());
        ds.field("max_stat_value", &self.max_stat_value());
        ds.field("icon", &self.icon());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("tags", &self.tags());
        ds.finish()
    }
}
