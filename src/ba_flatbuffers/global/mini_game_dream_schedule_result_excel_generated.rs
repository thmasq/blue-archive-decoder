extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{DreamMakerParamOperationType, DreamMakerParameterType, DreamMakerResult, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamScheduleResultExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamScheduleResultExcel<'a> {
    type Inner = MiniGameDreamScheduleResultExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamScheduleResultExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DREAM_MAKER_RESULT: ::flatbuffers::VOffsetT = 8;
    pub const VT_DREAM_MAKER_SCHEDULE_GROUP: ::flatbuffers::VOffsetT = 10;
    pub const VT_PROB: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARAMETER: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_PARAMETER_OPERATION_TYPE: ::flatbuffers::VOffsetT = 16;
    pub const VT_REWARD_PARAMETER_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 20;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 22;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamScheduleResultExcel::VT_ID, Some(0))
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
                    MiniGameDreamScheduleResultExcel::VT_EVENT_CONTENT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_result(&self) -> DreamMakerResult {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerResult>(
                    MiniGameDreamScheduleResultExcel::VT_DREAM_MAKER_RESULT,
                    Some(DreamMakerResult::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_schedule_group(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamScheduleResultExcel::VT_DREAM_MAKER_SCHEDULE_GROUP,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn prob(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameDreamScheduleResultExcel::VT_PROB, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parameter(&self) -> Option<::flatbuffers::Vector<'a, DreamMakerParameterType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, DreamMakerParameterType>,
            >>(MiniGameDreamScheduleResultExcel::VT_REWARD_PARAMETER, None)
        }
    }
    #[inline]
    pub fn reward_parameter_operation_type(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, DreamMakerParamOperationType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, DreamMakerParamOperationType>,
            >>(
                MiniGameDreamScheduleResultExcel::VT_REWARD_PARAMETER_OPERATION_TYPE,
                None,
            )
        }
    }
    #[inline]
    pub fn reward_parameter_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameDreamScheduleResultExcel::VT_REWARD_PARAMETER_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameDreamScheduleResultExcel::VT_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamScheduleResultExcel::VT_REWARD_PARCEL_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamScheduleResultExcel::VT_REWARD_PARCEL_AMOUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamScheduleResultExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
     .visit_field::<DreamMakerResult>("dream_maker_result", Self::VT_DREAM_MAKER_RESULT, false)?
     .visit_field::<i64>("dream_maker_schedule_group", Self::VT_DREAM_MAKER_SCHEDULE_GROUP, false)?
     .visit_field::<i32>("prob", Self::VT_PROB, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, DreamMakerParameterType>>>("reward_parameter", Self::VT_REWARD_PARAMETER, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, DreamMakerParamOperationType>>>("reward_parameter_operation_type", Self::VT_REWARD_PARAMETER_OPERATION_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>("reward_parameter_amount", Self::VT_REWARD_PARAMETER_AMOUNT, false)?
     .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
     .visit_field::<i64>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
     .visit_field::<i64>("reward_parcel_amount", Self::VT_REWARD_PARCEL_AMOUNT, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamScheduleResultExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamScheduleResultExcel", 11)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("dream_maker_result", &self.dream_maker_result())?;
        s.serialize_field(
            "dream_maker_schedule_group",
            &self.dream_maker_schedule_group(),
        )?;
        s.serialize_field("prob", &self.prob())?;
        if let Some(f) = self.reward_parameter() {
            s.serialize_field("reward_parameter", &f)?;
        } else {
            s.skip_field("reward_parameter")?;
        }
        if let Some(f) = self.reward_parameter_operation_type() {
            s.serialize_field("reward_parameter_operation_type", &f)?;
        } else {
            s.skip_field("reward_parameter_operation_type")?;
        }
        if let Some(f) = self.reward_parameter_amount() {
            s.serialize_field("reward_parameter_amount", &f)?;
        } else {
            s.skip_field("reward_parameter_amount")?;
        }
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_parcel_id", &self.reward_parcel_id())?;
        s.serialize_field("reward_parcel_amount", &self.reward_parcel_amount())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamScheduleResultExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamScheduleResultExcel");
        ds.field("id", &self.id());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("dream_maker_result", &self.dream_maker_result());
        ds.field(
            "dream_maker_schedule_group",
            &self.dream_maker_schedule_group(),
        );
        ds.field("prob", &self.prob());
        ds.field("reward_parameter", &self.reward_parameter());
        ds.field(
            "reward_parameter_operation_type",
            &self.reward_parameter_operation_type(),
        );
        ds.field("reward_parameter_amount", &self.reward_parameter_amount());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.finish()
    }
}
