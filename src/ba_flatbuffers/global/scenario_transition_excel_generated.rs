extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioTransitionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioTransitionExcel<'a> {
    type Inner = ScenarioTransitionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioTransitionExcel<'a> {
    pub const VT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_TRANSITION_OUT: ::flatbuffers::VOffsetT = 6;
    pub const VT_TRANSITION_OUT_DURATION: ::flatbuffers::VOffsetT = 8;
    pub const VT_TRANSITION_OUT_RESOURCE: ::flatbuffers::VOffsetT = 10;
    pub const VT_TRANSITION_IN: ::flatbuffers::VOffsetT = 12;
    pub const VT_TRANSITION_IN_DURATION: ::flatbuffers::VOffsetT = 14;
    pub const VT_TRANSITION_IN_RESOURCE: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioTransitionExcel::VT_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn transition_out(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioTransitionExcel::VT_TRANSITION_OUT,
                None,
            )
        }
    }
    #[inline]
    pub fn transition_out_duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioTransitionExcel::VT_TRANSITION_OUT_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn transition_out_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioTransitionExcel::VT_TRANSITION_OUT_RESOURCE,
                None,
            )
        }
    }
    #[inline]
    pub fn transition_in(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioTransitionExcel::VT_TRANSITION_IN,
                None,
            )
        }
    }
    #[inline]
    pub fn transition_in_duration(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioTransitionExcel::VT_TRANSITION_IN_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn transition_in_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioTransitionExcel::VT_TRANSITION_IN_RESOURCE,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioTransitionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("name", Self::VT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "transition_out",
                Self::VT_TRANSITION_OUT,
                false,
            )?
            .visit_field::<i64>(
                "transition_out_duration",
                Self::VT_TRANSITION_OUT_DURATION,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "transition_out_resource",
                Self::VT_TRANSITION_OUT_RESOURCE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "transition_in",
                Self::VT_TRANSITION_IN,
                false,
            )?
            .visit_field::<i64>(
                "transition_in_duration",
                Self::VT_TRANSITION_IN_DURATION,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "transition_in_resource",
                Self::VT_TRANSITION_IN_RESOURCE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioTransitionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioTransitionExcel", 7)?;
        s.serialize_field("name", &self.name())?;
        if let Some(f) = self.transition_out() {
            s.serialize_field("transition_out", &f)?;
        } else {
            s.skip_field("transition_out")?;
        }
        s.serialize_field("transition_out_duration", &self.transition_out_duration())?;
        if let Some(f) = self.transition_out_resource() {
            s.serialize_field("transition_out_resource", &f)?;
        } else {
            s.skip_field("transition_out_resource")?;
        }
        if let Some(f) = self.transition_in() {
            s.serialize_field("transition_in", &f)?;
        } else {
            s.skip_field("transition_in")?;
        }
        s.serialize_field("transition_in_duration", &self.transition_in_duration())?;
        if let Some(f) = self.transition_in_resource() {
            s.serialize_field("transition_in_resource", &f)?;
        } else {
            s.skip_field("transition_in_resource")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioTransitionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioTransitionExcel");
        ds.field("name", &self.name());
        ds.field("transition_out", &self.transition_out());
        ds.field("transition_out_duration", &self.transition_out_duration());
        ds.field("transition_out_resource", &self.transition_out_resource());
        ds.field("transition_in", &self.transition_in());
        ds.field("transition_in_duration", &self.transition_in_duration());
        ds.field("transition_in_resource", &self.transition_in_resource());
        ds.finish()
    }
}
