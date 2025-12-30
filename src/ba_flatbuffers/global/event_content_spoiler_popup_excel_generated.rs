extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentSpoilerPopupExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentSpoilerPopupExcel<'a> {
    type Inner = EventContentSpoilerPopupExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentSpoilerPopupExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SPOILER_POPUP_TITLE: ::flatbuffers::VOffsetT = 6;
    pub const VT_SPOILER_POPUP_DESCRIPTION: ::flatbuffers::VOffsetT = 8;
    pub const VT_IS_WARNING_POP_UP: ::flatbuffers::VOffsetT = 10;
    pub const VT_CONDITION_SCENARIO_MODE_ID: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentSpoilerPopupExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn spoiler_popup_title(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentSpoilerPopupExcel::VT_SPOILER_POPUP_TITLE,
                None,
            )
        }
    }
    #[inline]
    pub fn spoiler_popup_description(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentSpoilerPopupExcel::VT_SPOILER_POPUP_DESCRIPTION,
                None,
            )
        }
    }
    #[inline]
    pub fn is_warning_pop_up(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    EventContentSpoilerPopupExcel::VT_IS_WARNING_POP_UP,
                    Some(false),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn condition_scenario_mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    EventContentSpoilerPopupExcel::VT_CONDITION_SCENARIO_MODE_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentSpoilerPopupExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "spoiler_popup_title",
                Self::VT_SPOILER_POPUP_TITLE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "spoiler_popup_description",
                Self::VT_SPOILER_POPUP_DESCRIPTION,
                false,
            )?
            .visit_field::<bool>("is_warning_pop_up", Self::VT_IS_WARNING_POP_UP, false)?
            .visit_field::<i64>(
                "condition_scenario_mode_id",
                Self::VT_CONDITION_SCENARIO_MODE_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for EventContentSpoilerPopupExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentSpoilerPopupExcel", 5)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        if let Some(f) = self.spoiler_popup_title() {
            s.serialize_field("spoiler_popup_title", &f)?;
        } else {
            s.skip_field("spoiler_popup_title")?;
        }
        if let Some(f) = self.spoiler_popup_description() {
            s.serialize_field("spoiler_popup_description", &f)?;
        } else {
            s.skip_field("spoiler_popup_description")?;
        }
        s.serialize_field("is_warning_pop_up", &self.is_warning_pop_up())?;
        s.serialize_field(
            "condition_scenario_mode_id",
            &self.condition_scenario_mode_id(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentSpoilerPopupExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentSpoilerPopupExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("spoiler_popup_title", &self.spoiler_popup_title());
        ds.field(
            "spoiler_popup_description",
            &self.spoiler_popup_description(),
        );
        ds.field("is_warning_pop_up", &self.is_warning_pop_up());
        ds.field(
            "condition_scenario_mode_id",
            &self.condition_scenario_mode_id(),
        );
        ds.finish()
    }
}
