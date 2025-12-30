extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ScenarioContentType;

#[derive(Copy, Clone, PartialEq)]
pub struct ContentsScenarioExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ContentsScenarioExcel<'a> {
    type Inner = ContentsScenarioExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ContentsScenarioExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCALIZE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_SCENARIO_CONTENT_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_SCENARIO_GROUP_ID: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ContentsScenarioExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ContentsScenarioExcel::VT_LOCALIZE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn display_order(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ContentsScenarioExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_content_type(&self) -> ScenarioContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioContentType>(
                    ContentsScenarioExcel::VT_SCENARIO_CONTENT_TYPE,
                    Some(ScenarioContentType::Prologue),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_group_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    ContentsScenarioExcel::VT_SCENARIO_GROUP_ID,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for ContentsScenarioExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("id", Self::VT_ID, false)?
            .visit_field::<u32>("localize_id", Self::VT_LOCALIZE_ID, false)?
            .visit_field::<i32>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<ScenarioContentType>(
                "scenario_content_type",
                Self::VT_SCENARIO_CONTENT_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "scenario_group_id",
                Self::VT_SCENARIO_GROUP_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ContentsScenarioExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContentsScenarioExcel", 5)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("localize_id", &self.localize_id())?;
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("scenario_content_type", &self.scenario_content_type())?;
        if let Some(f) = self.scenario_group_id() {
            s.serialize_field("scenario_group_id", &f)?;
        } else {
            s.skip_field("scenario_group_id")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ContentsScenarioExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ContentsScenarioExcel");
        ds.field("id", &self.id());
        ds.field("localize_id", &self.localize_id());
        ds.field("display_order", &self.display_order());
        ds.field("scenario_content_type", &self.scenario_content_type());
        ds.field("scenario_group_id", &self.scenario_group_id());
        ds.finish()
    }
}
