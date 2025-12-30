extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::DreamMakerParameterType;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamCollectionScenarioExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamCollectionScenarioExcel<'a> {
    type Inner = MiniGameDreamCollectionScenarioExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamCollectionScenarioExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_IS_SKIP: ::flatbuffers::VOffsetT = 6;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_PARAMETER: ::flatbuffers::VOffsetT = 10;
    pub const VT_PARAMETER_AMOUNT: ::flatbuffers::VOffsetT = 12;
    pub const VT_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamCollectionScenarioExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_skip(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MiniGameDreamCollectionScenarioExcel::VT_IS_SKIP,
                    Some(false),
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
                .get::<i64>(
                    MiniGameDreamCollectionScenarioExcel::VT_EVENT_CONTENT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn parameter(&self) -> Option<::flatbuffers::Vector<'a, DreamMakerParameterType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, DreamMakerParameterType>,
            >>(MiniGameDreamCollectionScenarioExcel::VT_PARAMETER, None)
        }
    }
    #[inline]
    pub fn parameter_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDreamCollectionScenarioExcel::VT_PARAMETER_AMOUNT,
                    None,
                )
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
                    MiniGameDreamCollectionScenarioExcel::VT_SCENARIO_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamCollectionScenarioExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<bool>("is_skip", Self::VT_IS_SKIP, false)?
     .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, DreamMakerParameterType>>>("parameter", Self::VT_PARAMETER, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("parameter_amount", Self::VT_PARAMETER_AMOUNT, false)?
     .visit_field::<i64>("scenario_group_id", Self::VT_SCENARIO_GROUP_ID, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamCollectionScenarioExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamCollectionScenarioExcel", 6)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("is_skip", &self.is_skip())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        if let Some(f) = self.parameter() {
            s.serialize_field("parameter", &f)?;
        } else {
            s.skip_field("parameter")?;
        }
        if let Some(f) = self.parameter_amount() {
            s.serialize_field("parameter_amount", &f)?;
        } else {
            s.skip_field("parameter_amount")?;
        }
        s.serialize_field("scenario_group_id", &self.scenario_group_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamCollectionScenarioExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamCollectionScenarioExcel");
        ds.field("id", &self.id());
        ds.field("is_skip", &self.is_skip());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("parameter", &self.parameter());
        ds.field("parameter_amount", &self.parameter_amount());
        ds.field("scenario_group_id", &self.scenario_group_id());
        ds.finish()
    }
}
