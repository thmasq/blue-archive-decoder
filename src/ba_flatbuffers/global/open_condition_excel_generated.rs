extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{MultipleConditionCheckType, OpenConditionContent, WeekDay};

#[derive(Copy, Clone, PartialEq)]

pub struct OpenConditionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for OpenConditionExcel<'a> {
    type Inner = OpenConditionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> OpenConditionExcel<'a> {
    pub const VT_OPEN_CONDITION_CONTENT_TYPE: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCK_UI: ::flatbuffers::VOffsetT = 6;
    pub const VT_SHORTCUT_POPUP_PRIORITY: ::flatbuffers::VOffsetT = 8;
    pub const VT_SHORTCUT_UI_NAME: ::flatbuffers::VOffsetT = 10;
    pub const VT_SHORTCUT_PARAM: ::flatbuffers::VOffsetT = 12;
    pub const VT_SCENE: ::flatbuffers::VOffsetT = 14;
    pub const VT_HIDE_WHEN_LOCKED: ::flatbuffers::VOffsetT = 16;
    pub const VT_ACCOUNT_LEVEL: ::flatbuffers::VOffsetT = 18;
    pub const VT_SCENARIO_MODE_ID: ::flatbuffers::VOffsetT = 20;
    pub const VT_CAMPAIGN_STAGE_ID: ::flatbuffers::VOffsetT = 22;
    pub const VT_MULTIPLE_CONDITION_CHECK_TYPE: ::flatbuffers::VOffsetT = 24;
    pub const VT_OPEN_DAY_OF_WEEK: ::flatbuffers::VOffsetT = 26;
    pub const VT_OPEN_HOUR: ::flatbuffers::VOffsetT = 28;
    pub const VT_CLOSE_DAY_OF_WEEK: ::flatbuffers::VOffsetT = 30;
    pub const VT_CLOSE_HOUR: ::flatbuffers::VOffsetT = 32;
    pub const VT_OPENED_CAFE_ID: ::flatbuffers::VOffsetT = 34;
    pub const VT_CAFE_IDFOR_CAFE_RANK: ::flatbuffers::VOffsetT = 36;
    pub const VT_CAFE_RANK: ::flatbuffers::VOffsetT = 38;
    pub const VT_CONTENTS_OPEN_SHOW: ::flatbuffers::VOffsetT = 40;
    pub const VT_CONTENTS_OPEN_SHORTCUT_UI: ::flatbuffers::VOffsetT = 42;

    #[inline]
    pub fn open_condition_content_type(&self) -> OpenConditionContent {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<OpenConditionContent>(
                    OpenConditionExcel::VT_OPEN_CONDITION_CONTENT_TYPE,
                    Some(OpenConditionContent::Shop),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn lock_ui(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(OpenConditionExcel::VT_LOCK_UI, None)
        }
    }
    #[inline]
    pub fn shortcut_popup_priority(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_SHORTCUT_POPUP_PRIORITY, Some(0))
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
            >>(OpenConditionExcel::VT_SHORTCUT_UI_NAME, None)
        }
    }
    #[inline]
    pub fn shortcut_param(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OpenConditionExcel::VT_SHORTCUT_PARAM, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scene(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(OpenConditionExcel::VT_SCENE, None)
        }
    }
    #[inline]
    pub fn hide_when_locked(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(OpenConditionExcel::VT_HIDE_WHEN_LOCKED, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn account_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_ACCOUNT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_SCENARIO_MODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn campaign_stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_CAMPAIGN_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn multiple_condition_check_type(&self) -> MultipleConditionCheckType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MultipleConditionCheckType>(
                    OpenConditionExcel::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                    Some(MultipleConditionCheckType::And),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn open_day_of_week(&self) -> WeekDay {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<WeekDay>(
                    OpenConditionExcel::VT_OPEN_DAY_OF_WEEK,
                    Some(WeekDay::Sunday),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn open_hour(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_OPEN_HOUR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn close_day_of_week(&self) -> WeekDay {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<WeekDay>(
                    OpenConditionExcel::VT_CLOSE_DAY_OF_WEEK,
                    Some(WeekDay::Sunday),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn close_hour(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_CLOSE_HOUR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn opened_cafe_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_OPENED_CAFE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn cafe_idfor_cafe_rank(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_CAFE_IDFOR_CAFE_RANK, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn cafe_rank(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OpenConditionExcel::VT_CAFE_RANK, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn contents_open_show(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(OpenConditionExcel::VT_CONTENTS_OPEN_SHOW, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn contents_open_shortcut_ui(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                OpenConditionExcel::VT_CONTENTS_OPEN_SHORTCUT_UI,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for OpenConditionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<OpenConditionContent>(
                "open_condition_content_type",
                Self::VT_OPEN_CONDITION_CONTENT_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("lock_ui", Self::VT_LOCK_UI, false)?
            .visit_field::<i64>(
                "shortcut_popup_priority",
                Self::VT_SHORTCUT_POPUP_PRIORITY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("shortcut_ui_name", Self::VT_SHORTCUT_UI_NAME, false)?
            .visit_field::<i32>("shortcut_param", Self::VT_SHORTCUT_PARAM, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("scene", Self::VT_SCENE, false)?
            .visit_field::<bool>("hide_when_locked", Self::VT_HIDE_WHEN_LOCKED, false)?
            .visit_field::<i64>("account_level", Self::VT_ACCOUNT_LEVEL, false)?
            .visit_field::<i64>("scenario_mode_id", Self::VT_SCENARIO_MODE_ID, false)?
            .visit_field::<i64>("campaign_stage_id", Self::VT_CAMPAIGN_STAGE_ID, false)?
            .visit_field::<MultipleConditionCheckType>(
                "multiple_condition_check_type",
                Self::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                false,
            )?
            .visit_field::<WeekDay>("open_day_of_week", Self::VT_OPEN_DAY_OF_WEEK, false)?
            .visit_field::<i64>("open_hour", Self::VT_OPEN_HOUR, false)?
            .visit_field::<WeekDay>("close_day_of_week", Self::VT_CLOSE_DAY_OF_WEEK, false)?
            .visit_field::<i64>("close_hour", Self::VT_CLOSE_HOUR, false)?
            .visit_field::<i64>("opened_cafe_id", Self::VT_OPENED_CAFE_ID, false)?
            .visit_field::<i64>("cafe_idfor_cafe_rank", Self::VT_CAFE_IDFOR_CAFE_RANK, false)?
            .visit_field::<i64>("cafe_rank", Self::VT_CAFE_RANK, false)?
            .visit_field::<bool>("contents_open_show", Self::VT_CONTENTS_OPEN_SHOW, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "contents_open_shortcut_ui",
                Self::VT_CONTENTS_OPEN_SHORTCUT_UI,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for OpenConditionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OpenConditionExcel", 20)?;
        s.serialize_field(
            "open_condition_content_type",
            &self.open_condition_content_type(),
        )?;
        if let Some(f) = self.lock_ui() {
            s.serialize_field("lock_ui", &f)?;
        } else {
            s.skip_field("lock_ui")?;
        }
        s.serialize_field("shortcut_popup_priority", &self.shortcut_popup_priority())?;
        if let Some(f) = self.shortcut_ui_name() {
            s.serialize_field("shortcut_ui_name", &f)?;
        } else {
            s.skip_field("shortcut_ui_name")?;
        }
        s.serialize_field("shortcut_param", &self.shortcut_param())?;
        if let Some(f) = self.scene() {
            s.serialize_field("scene", &f)?;
        } else {
            s.skip_field("scene")?;
        }
        s.serialize_field("hide_when_locked", &self.hide_when_locked())?;
        s.serialize_field("account_level", &self.account_level())?;
        s.serialize_field("scenario_mode_id", &self.scenario_mode_id())?;
        s.serialize_field("campaign_stage_id", &self.campaign_stage_id())?;
        s.serialize_field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        )?;
        s.serialize_field("open_day_of_week", &self.open_day_of_week())?;
        s.serialize_field("open_hour", &self.open_hour())?;
        s.serialize_field("close_day_of_week", &self.close_day_of_week())?;
        s.serialize_field("close_hour", &self.close_hour())?;
        s.serialize_field("opened_cafe_id", &self.opened_cafe_id())?;
        s.serialize_field("cafe_idfor_cafe_rank", &self.cafe_idfor_cafe_rank())?;
        s.serialize_field("cafe_rank", &self.cafe_rank())?;
        s.serialize_field("contents_open_show", &self.contents_open_show())?;
        if let Some(f) = self.contents_open_shortcut_ui() {
            s.serialize_field("contents_open_shortcut_ui", &f)?;
        } else {
            s.skip_field("contents_open_shortcut_ui")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for OpenConditionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("OpenConditionExcel");
        ds.field(
            "open_condition_content_type",
            &self.open_condition_content_type(),
        );
        ds.field("lock_ui", &self.lock_ui());
        ds.field("shortcut_popup_priority", &self.shortcut_popup_priority());
        ds.field("shortcut_ui_name", &self.shortcut_ui_name());
        ds.field("shortcut_param", &self.shortcut_param());
        ds.field("scene", &self.scene());
        ds.field("hide_when_locked", &self.hide_when_locked());
        ds.field("account_level", &self.account_level());
        ds.field("scenario_mode_id", &self.scenario_mode_id());
        ds.field("campaign_stage_id", &self.campaign_stage_id());
        ds.field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        );
        ds.field("open_day_of_week", &self.open_day_of_week());
        ds.field("open_hour", &self.open_hour());
        ds.field("close_day_of_week", &self.close_day_of_week());
        ds.field("close_hour", &self.close_hour());
        ds.field("opened_cafe_id", &self.opened_cafe_id());
        ds.field("cafe_idfor_cafe_rank", &self.cafe_idfor_cafe_rank());
        ds.field("cafe_rank", &self.cafe_rank());
        ds.field("contents_open_show", &self.contents_open_show());
        ds.field(
            "contents_open_shortcut_ui",
            &self.contents_open_shortcut_ui(),
        );
        ds.finish()
    }
}
