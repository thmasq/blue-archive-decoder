extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamReplayScenarioExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamReplayScenarioExcel<'a> {
    type Inner = MiniGameDreamReplayScenarioExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamReplayScenarioExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_REPLAY_SUMMARY_TITLE_LOCALIZE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REPLAY_SUMMARY_LOCALIZE_SCENARIO_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REPLAY_SCENARIO_RESOURCE: ::flatbuffers::VOffsetT = 14;
    pub const VT_IS_REPLAY_SCENARIO_HORIZON: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamReplayScenarioExcel::VT_EVENT_CONTENT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamReplayScenarioExcel::VT_SCENARIO_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamReplayScenarioExcel::VT_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn replay_summary_title_localize(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MiniGameDreamReplayScenarioExcel::VT_REPLAY_SUMMARY_TITLE_LOCALIZE,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn replay_summary_localize_scenario_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MiniGameDreamReplayScenarioExcel::VT_REPLAY_SUMMARY_LOCALIZE_SCENARIO_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn replay_scenario_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamReplayScenarioExcel::VT_REPLAY_SCENARIO_RESOURCE,
                None,
            )
        }
    }
    #[inline]
    pub fn is_replay_scenario_horizon(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MiniGameDreamReplayScenarioExcel::VT_IS_REPLAY_SCENARIO_HORIZON,
                    Some(false),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamReplayScenarioExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("scenario_group_id", Self::VT_SCENARIO_GROUP_ID, false)?
            .visit_field::<i64>("order", Self::VT_ORDER, false)?
            .visit_field::<u32>(
                "replay_summary_title_localize",
                Self::VT_REPLAY_SUMMARY_TITLE_LOCALIZE,
                false,
            )?
            .visit_field::<u32>(
                "replay_summary_localize_scenario_id",
                Self::VT_REPLAY_SUMMARY_LOCALIZE_SCENARIO_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "replay_scenario_resource",
                Self::VT_REPLAY_SCENARIO_RESOURCE,
                false,
            )?
            .visit_field::<bool>(
                "is_replay_scenario_horizon",
                Self::VT_IS_REPLAY_SCENARIO_HORIZON,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamReplayScenarioExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamReplayScenarioExcel", 7)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("scenario_group_id", &self.scenario_group_id())?;
        s.serialize_field("order", &self.order())?;
        s.serialize_field(
            "replay_summary_title_localize",
            &self.replay_summary_title_localize(),
        )?;
        s.serialize_field(
            "replay_summary_localize_scenario_id",
            &self.replay_summary_localize_scenario_id(),
        )?;
        if let Some(f) = self.replay_scenario_resource() {
            s.serialize_field("replay_scenario_resource", &f)?;
        } else {
            s.skip_field("replay_scenario_resource")?;
        }
        s.serialize_field(
            "is_replay_scenario_horizon",
            &self.is_replay_scenario_horizon(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamReplayScenarioExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamReplayScenarioExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("scenario_group_id", &self.scenario_group_id());
        ds.field("order", &self.order());
        ds.field(
            "replay_summary_title_localize",
            &self.replay_summary_title_localize(),
        );
        ds.field(
            "replay_summary_localize_scenario_id",
            &self.replay_summary_localize_scenario_id(),
        );
        ds.field("replay_scenario_resource", &self.replay_scenario_resource());
        ds.field(
            "is_replay_scenario_horizon",
            &self.is_replay_scenario_horizon(),
        );
        ds.finish()
    }
}
