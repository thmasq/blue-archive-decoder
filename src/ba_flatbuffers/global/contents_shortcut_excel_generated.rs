extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ContentType, StageDifficulty};

#[derive(Copy, Clone, PartialEq)]
pub struct ContentsShortcutExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ContentsShortcutExcel<'a> {
    type Inner = ContentsShortcutExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ContentsShortcutExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_SCENARIO_MODE_VOLUME: ::flatbuffers::VOffsetT = 10;
    pub const VT_SCENARIO_MODE_CHAPTER: ::flatbuffers::VOffsetT = 12;
    pub const VT_SHORTCUT_OPEN_TIME: ::flatbuffers::VOffsetT = 14;
    pub const VT_SHORTCUT_CLOSE_TIME: ::flatbuffers::VOffsetT = 16;
    pub const VT_CONDITION_CONTENT_ID: ::flatbuffers::VOffsetT = 18;
    pub const VT_CONQUEST_MAP_DIFFICULTY: ::flatbuffers::VOffsetT = 20;
    pub const VT_CONQUEST_STEP_INDEX: ::flatbuffers::VOffsetT = 22;
    pub const VT_SHORTCUT_CONTENT_ID: ::flatbuffers::VOffsetT = 24;
    pub const VT_SHORTCUT_UI_NAME: ::flatbuffers::VOffsetT = 26;
    pub const VT_LOCALIZE: ::flatbuffers::VOffsetT = 28;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentsShortcutExcel::VT_UNIQUE_ID, Some(0))
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
                    ContentsShortcutExcel::VT_CONTENT_TYPE,
                    Some(ContentType::None),
                )
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
                .get::<i64>(ContentsShortcutExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_volume(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentsShortcutExcel::VT_SCENARIO_MODE_VOLUME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_chapter(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentsShortcutExcel::VT_SCENARIO_MODE_CHAPTER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn shortcut_open_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ContentsShortcutExcel::VT_SHORTCUT_OPEN_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn shortcut_close_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ContentsShortcutExcel::VT_SHORTCUT_CLOSE_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn condition_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentsShortcutExcel::VT_CONDITION_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn conquest_map_difficulty(&self) -> StageDifficulty {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StageDifficulty>(
                    ContentsShortcutExcel::VT_CONQUEST_MAP_DIFFICULTY,
                    Some(StageDifficulty::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn conquest_step_index(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ContentsShortcutExcel::VT_CONQUEST_STEP_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn shortcut_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentsShortcutExcel::VT_SHORTCUT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn shortcut_ui_name(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(ContentsShortcutExcel::VT_SHORTCUT_UI_NAME, None)
        }
    }
    #[inline]
    pub fn localize(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ContentsShortcutExcel::VT_LOCALIZE,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ContentsShortcutExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("scenario_mode_volume", Self::VT_SCENARIO_MODE_VOLUME, false)?
            .visit_field::<i64>(
                "scenario_mode_chapter",
                Self::VT_SCENARIO_MODE_CHAPTER,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "shortcut_open_time",
                Self::VT_SHORTCUT_OPEN_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "shortcut_close_time",
                Self::VT_SHORTCUT_CLOSE_TIME,
                false,
            )?
            .visit_field::<i64>("condition_content_id", Self::VT_CONDITION_CONTENT_ID, false)?
            .visit_field::<StageDifficulty>(
                "conquest_map_difficulty",
                Self::VT_CONQUEST_MAP_DIFFICULTY,
                false,
            )?
            .visit_field::<i32>("conquest_step_index", Self::VT_CONQUEST_STEP_INDEX, false)?
            .visit_field::<i64>("shortcut_content_id", Self::VT_SHORTCUT_CONTENT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("shortcut_ui_name", Self::VT_SHORTCUT_UI_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize",
                Self::VT_LOCALIZE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ContentsShortcutExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContentsShortcutExcel", 13)?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("content_type", &self.content_type())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("scenario_mode_volume", &self.scenario_mode_volume())?;
        s.serialize_field("scenario_mode_chapter", &self.scenario_mode_chapter())?;
        if let Some(f) = self.shortcut_open_time() {
            s.serialize_field("shortcut_open_time", &f)?;
        } else {
            s.skip_field("shortcut_open_time")?;
        }
        if let Some(f) = self.shortcut_close_time() {
            s.serialize_field("shortcut_close_time", &f)?;
        } else {
            s.skip_field("shortcut_close_time")?;
        }
        s.serialize_field("condition_content_id", &self.condition_content_id())?;
        s.serialize_field("conquest_map_difficulty", &self.conquest_map_difficulty())?;
        s.serialize_field("conquest_step_index", &self.conquest_step_index())?;
        s.serialize_field("shortcut_content_id", &self.shortcut_content_id())?;
        if let Some(f) = self.shortcut_ui_name() {
            s.serialize_field("shortcut_ui_name", &f)?;
        } else {
            s.skip_field("shortcut_ui_name")?;
        }
        if let Some(f) = self.localize() {
            s.serialize_field("localize", &f)?;
        } else {
            s.skip_field("localize")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ContentsShortcutExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ContentsShortcutExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("content_type", &self.content_type());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("scenario_mode_volume", &self.scenario_mode_volume());
        ds.field("scenario_mode_chapter", &self.scenario_mode_chapter());
        ds.field("shortcut_open_time", &self.shortcut_open_time());
        ds.field("shortcut_close_time", &self.shortcut_close_time());
        ds.field("condition_content_id", &self.condition_content_id());
        ds.field("conquest_map_difficulty", &self.conquest_map_difficulty());
        ds.field("conquest_step_index", &self.conquest_step_index());
        ds.field("shortcut_content_id", &self.shortcut_content_id());
        ds.field("shortcut_ui_name", &self.shortcut_ui_name());
        ds.field("localize", &self.localize());
        ds.finish()
    }
}
