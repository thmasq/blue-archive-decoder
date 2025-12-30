extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{DreamMakerEndingCondition, DreamMakerEndingType};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamEndingExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamEndingExcel<'a> {
    type Inner = MiniGameDreamEndingExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamEndingExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ENDING_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DREAM_MAKER_ENDING_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_ORDER: ::flatbuffers::VOffsetT = 10;
    pub const VT_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_ENDING_CONDITION: ::flatbuffers::VOffsetT = 14;
    pub const VT_ENDING_CONDITION_VALUE: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamEndingExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ending_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamEndingExcel::VT_ENDING_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_ending_type(&self) -> DreamMakerEndingType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerEndingType>(
                    MiniGameDreamEndingExcel::VT_DREAM_MAKER_ENDING_TYPE,
                    Some(DreamMakerEndingType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn order(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDreamEndingExcel::VT_ORDER, Some(0))
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
                .get::<i64>(MiniGameDreamEndingExcel::VT_SCENARIO_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ending_condition(&self) -> Option<::flatbuffers::Vector<'a, DreamMakerEndingCondition>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, DreamMakerEndingCondition>,
            >>(MiniGameDreamEndingExcel::VT_ENDING_CONDITION, None)
        }
    }
    #[inline]
    pub fn ending_condition_value(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDreamEndingExcel::VT_ENDING_CONDITION_VALUE,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamEndingExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
     .visit_field::<i64>("ending_id", Self::VT_ENDING_ID, false)?
     .visit_field::<DreamMakerEndingType>("dream_maker_ending_type", Self::VT_DREAM_MAKER_ENDING_TYPE, false)?
     .visit_field::<i32>("order", Self::VT_ORDER, false)?
     .visit_field::<i64>("scenario_group_id", Self::VT_SCENARIO_GROUP_ID, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, DreamMakerEndingCondition>>>("ending_condition", Self::VT_ENDING_CONDITION, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("ending_condition_value", Self::VT_ENDING_CONDITION_VALUE, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamEndingExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamEndingExcel", 7)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("ending_id", &self.ending_id())?;
        s.serialize_field("dream_maker_ending_type", &self.dream_maker_ending_type())?;
        s.serialize_field("order", &self.order())?;
        s.serialize_field("scenario_group_id", &self.scenario_group_id())?;
        if let Some(f) = self.ending_condition() {
            s.serialize_field("ending_condition", &f)?;
        } else {
            s.skip_field("ending_condition")?;
        }
        if let Some(f) = self.ending_condition_value() {
            s.serialize_field("ending_condition_value", &f)?;
        } else {
            s.skip_field("ending_condition_value")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamEndingExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamEndingExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("ending_id", &self.ending_id());
        ds.field("dream_maker_ending_type", &self.dream_maker_ending_type());
        ds.field("order", &self.order());
        ds.field("scenario_group_id", &self.scenario_group_id());
        ds.field("ending_condition", &self.ending_condition());
        ds.field("ending_condition_value", &self.ending_condition_value());
        ds.finish()
    }
}
