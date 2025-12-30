extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::OperatorCondition;

#[derive(Copy, Clone, PartialEq)]
pub struct OperatorExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for OperatorExcel<'a> {
    type Inner = OperatorExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> OperatorExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_OPERATOR_CONDITION: ::flatbuffers::VOffsetT = 8;
    pub const VT_OUTPUT_SEQUENCE: ::flatbuffers::VOffsetT = 10;
    pub const VT_RANDOM_WEIGHT: ::flatbuffers::VOffsetT = 12;
    pub const VT_OUTPUT_DELAY: ::flatbuffers::VOffsetT = 14;
    pub const VT_DURATION: ::flatbuffers::VOffsetT = 16;
    pub const VT_OPERATOR_OUTPUT_PRIORITY: ::flatbuffers::VOffsetT = 18;
    pub const VT_PORTRAIT_PATH: ::flatbuffers::VOffsetT = 20;
    pub const VT_TEXT_LOCALIZE_KEY: ::flatbuffers::VOffsetT = 22;
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 24;
    pub const VT_OPERATOR_WAIT_QUEUE: ::flatbuffers::VOffsetT = 26;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(OperatorExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn group_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(OperatorExcel::VT_GROUP_ID, None)
        }
    }
    #[inline]
    pub fn operator_condition(&self) -> OperatorCondition {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<OperatorCondition>(
                    OperatorExcel::VT_OPERATOR_CONDITION,
                    Some(OperatorCondition::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn output_sequence(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OperatorExcel::VT_OUTPUT_SEQUENCE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn random_weight(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OperatorExcel::VT_RANDOM_WEIGHT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn output_delay(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OperatorExcel::VT_OUTPUT_DELAY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn duration(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OperatorExcel::VT_DURATION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn operator_output_priority(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(OperatorExcel::VT_OPERATOR_OUTPUT_PRIORITY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn portrait_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(OperatorExcel::VT_PORTRAIT_PATH, None)
        }
    }
    #[inline]
    pub fn text_localize_key(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                OperatorExcel::VT_TEXT_LOCALIZE_KEY,
                None,
            )
        }
    }
    #[inline]
    pub fn voice_id(&self) -> Option<::flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, u32>>>(
                    OperatorExcel::VT_VOICE_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn operator_wait_queue(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(OperatorExcel::VT_OPERATOR_WAIT_QUEUE, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for OperatorExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "group_id",
                Self::VT_GROUP_ID,
                false,
            )?
            .visit_field::<OperatorCondition>(
                "operator_condition",
                Self::VT_OPERATOR_CONDITION,
                false,
            )?
            .visit_field::<i32>("output_sequence", Self::VT_OUTPUT_SEQUENCE, false)?
            .visit_field::<i32>("random_weight", Self::VT_RANDOM_WEIGHT, false)?
            .visit_field::<i32>("output_delay", Self::VT_OUTPUT_DELAY, false)?
            .visit_field::<i32>("duration", Self::VT_DURATION, false)?
            .visit_field::<i32>(
                "operator_output_priority",
                Self::VT_OPERATOR_OUTPUT_PRIORITY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "portrait_path",
                Self::VT_PORTRAIT_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "text_localize_key",
                Self::VT_TEXT_LOCALIZE_KEY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>(
                "voice_id",
                Self::VT_VOICE_ID,
                false,
            )?
            .visit_field::<bool>("operator_wait_queue", Self::VT_OPERATOR_WAIT_QUEUE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for OperatorExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OperatorExcel", 12)?;
        s.serialize_field("unique_id", &self.unique_id())?;
        if let Some(f) = self.group_id() {
            s.serialize_field("group_id", &f)?;
        } else {
            s.skip_field("group_id")?;
        }
        s.serialize_field("operator_condition", &self.operator_condition())?;
        s.serialize_field("output_sequence", &self.output_sequence())?;
        s.serialize_field("random_weight", &self.random_weight())?;
        s.serialize_field("output_delay", &self.output_delay())?;
        s.serialize_field("duration", &self.duration())?;
        s.serialize_field("operator_output_priority", &self.operator_output_priority())?;
        if let Some(f) = self.portrait_path() {
            s.serialize_field("portrait_path", &f)?;
        } else {
            s.skip_field("portrait_path")?;
        }
        if let Some(f) = self.text_localize_key() {
            s.serialize_field("text_localize_key", &f)?;
        } else {
            s.skip_field("text_localize_key")?;
        }
        if let Some(f) = self.voice_id() {
            s.serialize_field("voice_id", &f)?;
        } else {
            s.skip_field("voice_id")?;
        }
        s.serialize_field("operator_wait_queue", &self.operator_wait_queue())?;
        s.end()
    }
}

impl ::core::fmt::Debug for OperatorExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("OperatorExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("group_id", &self.group_id());
        ds.field("operator_condition", &self.operator_condition());
        ds.field("output_sequence", &self.output_sequence());
        ds.field("random_weight", &self.random_weight());
        ds.field("output_delay", &self.output_delay());
        ds.field("duration", &self.duration());
        ds.field("operator_output_priority", &self.operator_output_priority());
        ds.field("portrait_path", &self.portrait_path());
        ds.field("text_localize_key", &self.text_localize_key());
        ds.field("voice_id", &self.voice_id());
        ds.field("operator_wait_queue", &self.operator_wait_queue());
        ds.finish()
    }
}
