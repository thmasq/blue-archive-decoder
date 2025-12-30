extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct TutorialExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for TutorialExcel<'a> {
    type Inner = TutorialExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> TutorialExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_COMPLETION_REPORT_EVENT_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_COMPULSORY_TUTORIAL: ::flatbuffers::VOffsetT = 8;
    pub const VT_DESCRIPTION_TUTORIAL: ::flatbuffers::VOffsetT = 10;
    pub const VT_TUTORIAL_STAGE_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_UI_NAME: ::flatbuffers::VOffsetT = 14;
    pub const VT_TUTORIAL_PARENT_NAME: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(TutorialExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn completion_report_event_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialExcel::VT_COMPLETION_REPORT_EVENT_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn compulsory_tutorial(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(TutorialExcel::VT_COMPULSORY_TUTORIAL, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn description_tutorial(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(TutorialExcel::VT_DESCRIPTION_TUTORIAL, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn tutorial_stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(TutorialExcel::VT_TUTORIAL_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ui_name(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(TutorialExcel::VT_UI_NAME, None)
        }
    }
    #[inline]
    pub fn tutorial_parent_name(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(TutorialExcel::VT_TUTORIAL_PARENT_NAME, None)
        }
    }
}

impl ::flatbuffers::Verifiable for TutorialExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "completion_report_event_name",
                Self::VT_COMPLETION_REPORT_EVENT_NAME,
                false,
            )?
            .visit_field::<bool>("compulsory_tutorial", Self::VT_COMPULSORY_TUTORIAL, false)?
            .visit_field::<bool>("description_tutorial", Self::VT_DESCRIPTION_TUTORIAL, false)?
            .visit_field::<i64>("tutorial_stage_id", Self::VT_TUTORIAL_STAGE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("ui_name", Self::VT_UI_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("tutorial_parent_name", Self::VT_TUTORIAL_PARENT_NAME, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for TutorialExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TutorialExcel", 7)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.completion_report_event_name() {
            s.serialize_field("completion_report_event_name", &f)?;
        } else {
            s.skip_field("completion_report_event_name")?;
        }
        s.serialize_field("compulsory_tutorial", &self.compulsory_tutorial())?;
        s.serialize_field("description_tutorial", &self.description_tutorial())?;
        s.serialize_field("tutorial_stage_id", &self.tutorial_stage_id())?;
        if let Some(f) = self.ui_name() {
            s.serialize_field("ui_name", &f)?;
        } else {
            s.skip_field("ui_name")?;
        }
        if let Some(f) = self.tutorial_parent_name() {
            s.serialize_field("tutorial_parent_name", &f)?;
        } else {
            s.skip_field("tutorial_parent_name")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for TutorialExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("TutorialExcel");
        ds.field("id", &self.id());
        ds.field(
            "completion_report_event_name",
            &self.completion_report_event_name(),
        );
        ds.field("compulsory_tutorial", &self.compulsory_tutorial());
        ds.field("description_tutorial", &self.description_tutorial());
        ds.field("tutorial_stage_id", &self.tutorial_stage_id());
        ds.field("ui_name", &self.ui_name());
        ds.field("tutorial_parent_name", &self.tutorial_parent_name());
        ds.finish()
    }
}
