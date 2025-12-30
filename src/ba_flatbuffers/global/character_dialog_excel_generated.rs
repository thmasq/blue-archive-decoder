extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{
    Anniversary, CVCollectionType, DialogCategory, DialogCondition, DialogType, ProductionStep,
};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterDialogExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterDialogExcel<'a> {
    type Inner = CharacterDialogExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterDialogExcel<'a> {
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_COSTUME_UNIQUE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_PRODUCTION_STEP: ::flatbuffers::VOffsetT = 10;
    pub const VT_DIALOG_CATEGORY: ::flatbuffers::VOffsetT = 12;
    pub const VT_DIALOG_CONDITION: ::flatbuffers::VOffsetT = 14;
    pub const VT_ANNIVERSARY: ::flatbuffers::VOffsetT = 16;
    pub const VT_START_DATE: ::flatbuffers::VOffsetT = 18;
    pub const VT_END_DATE: ::flatbuffers::VOffsetT = 20;
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 22;
    pub const VT_DIALOG_TYPE: ::flatbuffers::VOffsetT = 24;
    pub const VT_ACTION_NAME: ::flatbuffers::VOffsetT = 26;
    pub const VT_DURATION: ::flatbuffers::VOffsetT = 28;
    pub const VT_DURATION_KR: ::flatbuffers::VOffsetT = 30;
    pub const VT_ANIMATION_NAME: ::flatbuffers::VOffsetT = 32;
    pub const VT_LOCALIZE_KR: ::flatbuffers::VOffsetT = 34;
    pub const VT_LOCALIZE_JP: ::flatbuffers::VOffsetT = 36;
    pub const VT_LOCALIZE_TH: ::flatbuffers::VOffsetT = 38;
    pub const VT_LOCALIZE_TW: ::flatbuffers::VOffsetT = 40;
    pub const VT_LOCALIZE_EN: ::flatbuffers::VOffsetT = 42;
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 44;
    pub const VT_APPLY_POSITION: ::flatbuffers::VOffsetT = 46;
    pub const VT_POS_X: ::flatbuffers::VOffsetT = 48;
    pub const VT_POS_Y: ::flatbuffers::VOffsetT = 50;
    pub const VT_COLLECTION_VISIBLE: ::flatbuffers::VOffsetT = 52;
    pub const VT_CV_COLLECTION_TYPE: ::flatbuffers::VOffsetT = 54;
    pub const VT_UNLOCK_FAVOR_RANK: ::flatbuffers::VOffsetT = 56;
    pub const VT_UNLOCK_EQUIP_WEAPON: ::flatbuffers::VOffsetT = 58;
    pub const VT_LOCALIZE_CV_GROUP: ::flatbuffers::VOffsetT = 60;
    pub const VT_TEEN_MODE: ::flatbuffers::VOffsetT = 62;

    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn costume_unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_COSTUME_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn display_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_DISPLAY_ORDER, Some(0))
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
                    CharacterDialogExcel::VT_PRODUCTION_STEP,
                    Some(ProductionStep::ToDo),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dialog_category(&self) -> DialogCategory {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DialogCategory>(
                    CharacterDialogExcel::VT_DIALOG_CATEGORY,
                    Some(DialogCategory::Cafe),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dialog_condition(&self) -> DialogCondition {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DialogCondition>(
                    CharacterDialogExcel::VT_DIALOG_CONDITION,
                    Some(DialogCondition::Idle),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn anniversary(&self) -> Anniversary {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Anniversary>(
                    CharacterDialogExcel::VT_ANNIVERSARY,
                    Some(Anniversary::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn start_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_START_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn end_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_END_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dialog_type(&self) -> DialogType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DialogType>(CharacterDialogExcel::VT_DIALOG_TYPE, Some(DialogType::Talk))
                .unwrap()
        }
    }
    #[inline]
    pub fn action_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_ACTION_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn duration_kr(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_DURATION_KR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn animation_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_ANIMATION_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_EN,
                None,
            )
        }
    }
    #[inline]
    pub fn voice_id(&self) -> Option<::flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, u32>>>(
                    CharacterDialogExcel::VT_VOICE_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn apply_position(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogExcel::VT_APPLY_POSITION, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn pos_x(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CharacterDialogExcel::VT_POS_X, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn pos_y(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CharacterDialogExcel::VT_POS_Y, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn collection_visible(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogExcel::VT_COLLECTION_VISIBLE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn cv_collection_type(&self) -> CVCollectionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CVCollectionType>(
                    CharacterDialogExcel::VT_CV_COLLECTION_TYPE,
                    Some(CVCollectionType::CVNormal),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_favor_rank(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterDialogExcel::VT_UNLOCK_FAVOR_RANK, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_equip_weapon(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogExcel::VT_UNLOCK_EQUIP_WEAPON, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_cv_group(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterDialogExcel::VT_LOCALIZE_CV_GROUP,
                None,
            )
        }
    }
    #[inline]
    pub fn teen_mode(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterDialogExcel::VT_TEEN_MODE, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterDialogExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .visit_field::<i64>("costume_unique_id", Self::VT_COSTUME_UNIQUE_ID, false)?
            .visit_field::<i64>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<ProductionStep>("production_step", Self::VT_PRODUCTION_STEP, false)?
            .visit_field::<DialogCategory>("dialog_category", Self::VT_DIALOG_CATEGORY, false)?
            .visit_field::<DialogCondition>("dialog_condition", Self::VT_DIALOG_CONDITION, false)?
            .visit_field::<Anniversary>("anniversary", Self::VT_ANNIVERSARY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "start_date",
                Self::VT_START_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "end_date",
                Self::VT_END_DATE,
                false,
            )?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<DialogType>("dialog_type", Self::VT_DIALOG_TYPE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "action_name",
                Self::VT_ACTION_NAME,
                false,
            )?
            .visit_field::<i64>("duration", Self::VT_DURATION, false)?
            .visit_field::<i64>("duration_kr", Self::VT_DURATION_KR, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "animation_name",
                Self::VT_ANIMATION_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_kr",
                Self::VT_LOCALIZE_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_jp",
                Self::VT_LOCALIZE_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_th",
                Self::VT_LOCALIZE_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_tw",
                Self::VT_LOCALIZE_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_en",
                Self::VT_LOCALIZE_EN,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>(
                "voice_id",
                Self::VT_VOICE_ID,
                false,
            )?
            .visit_field::<bool>("apply_position", Self::VT_APPLY_POSITION, false)?
            .visit_field::<f32>("pos_x", Self::VT_POS_X, false)?
            .visit_field::<f32>("pos_y", Self::VT_POS_Y, false)?
            .visit_field::<bool>("collection_visible", Self::VT_COLLECTION_VISIBLE, false)?
            .visit_field::<CVCollectionType>(
                "cv_collection_type",
                Self::VT_CV_COLLECTION_TYPE,
                false,
            )?
            .visit_field::<i64>("unlock_favor_rank", Self::VT_UNLOCK_FAVOR_RANK, false)?
            .visit_field::<bool>("unlock_equip_weapon", Self::VT_UNLOCK_EQUIP_WEAPON, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_cv_group",
                Self::VT_LOCALIZE_CV_GROUP,
                false,
            )?
            .visit_field::<bool>("teen_mode", Self::VT_TEEN_MODE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterDialogExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterDialogExcel", 30)?;
        s.serialize_field("character_id", &self.character_id())?;
        s.serialize_field("costume_unique_id", &self.costume_unique_id())?;
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("production_step", &self.production_step())?;
        s.serialize_field("dialog_category", &self.dialog_category())?;
        s.serialize_field("dialog_condition", &self.dialog_condition())?;
        s.serialize_field("anniversary", &self.anniversary())?;
        if let Some(f) = self.start_date() {
            s.serialize_field("start_date", &f)?;
        } else {
            s.skip_field("start_date")?;
        }
        if let Some(f) = self.end_date() {
            s.serialize_field("end_date", &f)?;
        } else {
            s.skip_field("end_date")?;
        }
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("dialog_type", &self.dialog_type())?;
        if let Some(f) = self.action_name() {
            s.serialize_field("action_name", &f)?;
        } else {
            s.skip_field("action_name")?;
        }
        s.serialize_field("duration", &self.duration())?;
        s.serialize_field("duration_kr", &self.duration_kr())?;
        if let Some(f) = self.animation_name() {
            s.serialize_field("animation_name", &f)?;
        } else {
            s.skip_field("animation_name")?;
        }
        if let Some(f) = self.localize_kr() {
            s.serialize_field("localize_kr", &f)?;
        } else {
            s.skip_field("localize_kr")?;
        }
        if let Some(f) = self.localize_jp() {
            s.serialize_field("localize_jp", &f)?;
        } else {
            s.skip_field("localize_jp")?;
        }
        if let Some(f) = self.localize_th() {
            s.serialize_field("localize_th", &f)?;
        } else {
            s.skip_field("localize_th")?;
        }
        if let Some(f) = self.localize_tw() {
            s.serialize_field("localize_tw", &f)?;
        } else {
            s.skip_field("localize_tw")?;
        }
        if let Some(f) = self.localize_en() {
            s.serialize_field("localize_en", &f)?;
        } else {
            s.skip_field("localize_en")?;
        }
        if let Some(f) = self.voice_id() {
            s.serialize_field("voice_id", &f)?;
        } else {
            s.skip_field("voice_id")?;
        }
        s.serialize_field("apply_position", &self.apply_position())?;
        s.serialize_field("pos_x", &self.pos_x())?;
        s.serialize_field("pos_y", &self.pos_y())?;
        s.serialize_field("collection_visible", &self.collection_visible())?;
        s.serialize_field("cv_collection_type", &self.cv_collection_type())?;
        s.serialize_field("unlock_favor_rank", &self.unlock_favor_rank())?;
        s.serialize_field("unlock_equip_weapon", &self.unlock_equip_weapon())?;
        if let Some(f) = self.localize_cv_group() {
            s.serialize_field("localize_cv_group", &f)?;
        } else {
            s.skip_field("localize_cv_group")?;
        }
        s.serialize_field("teen_mode", &self.teen_mode())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterDialogExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterDialogExcel");
        ds.field("character_id", &self.character_id());
        ds.field("costume_unique_id", &self.costume_unique_id());
        ds.field("display_order", &self.display_order());
        ds.field("production_step", &self.production_step());
        ds.field("dialog_category", &self.dialog_category());
        ds.field("dialog_condition", &self.dialog_condition());
        ds.field("anniversary", &self.anniversary());
        ds.field("start_date", &self.start_date());
        ds.field("end_date", &self.end_date());
        ds.field("group_id", &self.group_id());
        ds.field("dialog_type", &self.dialog_type());
        ds.field("action_name", &self.action_name());
        ds.field("duration", &self.duration());
        ds.field("duration_kr", &self.duration_kr());
        ds.field("animation_name", &self.animation_name());
        ds.field("localize_kr", &self.localize_kr());
        ds.field("localize_jp", &self.localize_jp());
        ds.field("localize_th", &self.localize_th());
        ds.field("localize_tw", &self.localize_tw());
        ds.field("localize_en", &self.localize_en());
        ds.field("voice_id", &self.voice_id());
        ds.field("apply_position", &self.apply_position());
        ds.field("pos_x", &self.pos_x());
        ds.field("pos_y", &self.pos_y());
        ds.field("collection_visible", &self.collection_visible());
        ds.field("cv_collection_type", &self.cv_collection_type());
        ds.field("unlock_favor_rank", &self.unlock_favor_rank());
        ds.field("unlock_equip_weapon", &self.unlock_equip_weapon());
        ds.field("localize_cv_group", &self.localize_cv_group());
        ds.field("teen_mode", &self.teen_mode());
        ds.finish()
    }
}
