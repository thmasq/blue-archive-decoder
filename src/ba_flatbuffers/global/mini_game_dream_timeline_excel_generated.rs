extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamTimelineExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamTimelineExcel<'a> {
    type Inner = MiniGameDreamTimelineExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamTimelineExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_DREAM_MAKER_DAYS: ::flatbuffers::VOffsetT = 10;
    pub const VT_DREAM_MAKER_ACTION_POINT: ::flatbuffers::VOffsetT = 12;
    pub const VT_ENTER_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_BGM: ::flatbuffers::VOffsetT = 16;
    pub const VT_ART_LEVEL_PATH: ::flatbuffers::VOffsetT = 18;
    pub const VT_DESIGN_LEVEL_PATH: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamTimelineExcel::VT_ID, Some(0))
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
                .get::<i64>(MiniGameDreamTimelineExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamTimelineExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_days(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamTimelineExcel::VT_DREAM_MAKER_DAYS, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_action_point(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamTimelineExcel::VT_DREAM_MAKER_ACTION_POINT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn enter_scenario_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamTimelineExcel::VT_ENTER_SCENARIO_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamTimelineExcel::VT_BGM, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn art_level_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamTimelineExcel::VT_ART_LEVEL_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn design_level_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamTimelineExcel::VT_DESIGN_LEVEL_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamTimelineExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<i64>("dream_maker_days", Self::VT_DREAM_MAKER_DAYS, false)?
            .visit_field::<i64>(
                "dream_maker_action_point",
                Self::VT_DREAM_MAKER_ACTION_POINT,
                false,
            )?
            .visit_field::<i64>(
                "enter_scenario_group_id",
                Self::VT_ENTER_SCENARIO_GROUP_ID,
                false,
            )?
            .visit_field::<i64>("bgm", Self::VT_BGM, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "art_level_path",
                Self::VT_ART_LEVEL_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "design_level_path",
                Self::VT_DESIGN_LEVEL_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamTimelineExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamTimelineExcel", 9)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("dream_maker_days", &self.dream_maker_days())?;
        s.serialize_field("dream_maker_action_point", &self.dream_maker_action_point())?;
        s.serialize_field("enter_scenario_group_id", &self.enter_scenario_group_id())?;
        s.serialize_field("bgm", &self.bgm())?;
        if let Some(f) = self.art_level_path() {
            s.serialize_field("art_level_path", &f)?;
        } else {
            s.skip_field("art_level_path")?;
        }
        if let Some(f) = self.design_level_path() {
            s.serialize_field("design_level_path", &f)?;
        } else {
            s.skip_field("design_level_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamTimelineExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamTimelineExcel");
        ds.field("id", &self.id());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("group_id", &self.group_id());
        ds.field("dream_maker_days", &self.dream_maker_days());
        ds.field("dream_maker_action_point", &self.dream_maker_action_point());
        ds.field("enter_scenario_group_id", &self.enter_scenario_group_id());
        ds.field("bgm", &self.bgm());
        ds.field("art_level_path", &self.art_level_path());
        ds.field("design_level_path", &self.design_level_path());
        ds.finish()
    }
}
