extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{DreamMakerMultiplierCondition, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamInfoExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamInfoExcel<'a> {
    type Inner = MiniGameDreamInfoExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamInfoExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DREAM_MAKER_MULTIPLIER_CONDITION: ::flatbuffers::VOffsetT = 6;
    pub const VT_DREAM_MAKER_MULTIPLIER_CONDITION_VALUE: ::flatbuffers::VOffsetT = 8;
    pub const VT_DREAM_MAKER_MULTIPLIER_MAX: ::flatbuffers::VOffsetT = 10;
    pub const VT_DREAM_MAKER_DAYS: ::flatbuffers::VOffsetT = 12;
    pub const VT_DREAM_MAKER_ACTION_POINT: ::flatbuffers::VOffsetT = 14;
    pub const VT_DREAM_MAKER_PARCEL_TYPE: ::flatbuffers::VOffsetT = 16;
    pub const VT_DREAM_MAKER_PARCEL_ID: ::flatbuffers::VOffsetT = 18;
    pub const VT_DREAM_MAKER_DAILY_POINT_PARCEL_TYPE: ::flatbuffers::VOffsetT = 20;
    pub const VT_DREAM_MAKER_DAILY_POINT_ID: ::flatbuffers::VOffsetT = 22;
    pub const VT_DREAM_MAKER_PARAMETER_TRANSFER: ::flatbuffers::VOffsetT = 24;
    pub const VT_SCHEDULE_COST_GOODS_ID: ::flatbuffers::VOffsetT = 26;
    pub const VT_LOBBY_BGM_CHANGE_SCENARIO_ID: ::flatbuffers::VOffsetT = 28;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamInfoExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_multiplier_condition(&self) -> DreamMakerMultiplierCondition {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerMultiplierCondition>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_MULTIPLIER_CONDITION,
                    Some(DreamMakerMultiplierCondition::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_multiplier_condition_value(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_MULTIPLIER_CONDITION_VALUE,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_multiplier_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_MULTIPLIER_MAX,
                    Some(0),
                )
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
                .get::<i64>(MiniGameDreamInfoExcel::VT_DREAM_MAKER_DAYS, Some(0))
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
                .get::<i64>(MiniGameDreamInfoExcel::VT_DREAM_MAKER_ACTION_POINT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamInfoExcel::VT_DREAM_MAKER_PARCEL_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_daily_point_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_DAILY_POINT_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_daily_point_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_DAILY_POINT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_parameter_transfer(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamInfoExcel::VT_DREAM_MAKER_PARAMETER_TRANSFER,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn schedule_cost_goods_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamInfoExcel::VT_SCHEDULE_COST_GOODS_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn lobby_bgm_change_scenario_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamInfoExcel::VT_LOBBY_BGM_CHANGE_SCENARIO_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamInfoExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<DreamMakerMultiplierCondition>(
                "dream_maker_multiplier_condition",
                Self::VT_DREAM_MAKER_MULTIPLIER_CONDITION,
                false,
            )?
            .visit_field::<i64>(
                "dream_maker_multiplier_condition_value",
                Self::VT_DREAM_MAKER_MULTIPLIER_CONDITION_VALUE,
                false,
            )?
            .visit_field::<i64>(
                "dream_maker_multiplier_max",
                Self::VT_DREAM_MAKER_MULTIPLIER_MAX,
                false,
            )?
            .visit_field::<i64>("dream_maker_days", Self::VT_DREAM_MAKER_DAYS, false)?
            .visit_field::<i64>(
                "dream_maker_action_point",
                Self::VT_DREAM_MAKER_ACTION_POINT,
                false,
            )?
            .visit_field::<ParcelType>(
                "dream_maker_parcel_type",
                Self::VT_DREAM_MAKER_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "dream_maker_parcel_id",
                Self::VT_DREAM_MAKER_PARCEL_ID,
                false,
            )?
            .visit_field::<ParcelType>(
                "dream_maker_daily_point_parcel_type",
                Self::VT_DREAM_MAKER_DAILY_POINT_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "dream_maker_daily_point_id",
                Self::VT_DREAM_MAKER_DAILY_POINT_ID,
                false,
            )?
            .visit_field::<i64>(
                "dream_maker_parameter_transfer",
                Self::VT_DREAM_MAKER_PARAMETER_TRANSFER,
                false,
            )?
            .visit_field::<i64>(
                "schedule_cost_goods_id",
                Self::VT_SCHEDULE_COST_GOODS_ID,
                false,
            )?
            .visit_field::<i64>(
                "lobby_bgm_change_scenario_id",
                Self::VT_LOBBY_BGM_CHANGE_SCENARIO_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamInfoExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamInfoExcel", 13)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field(
            "dream_maker_multiplier_condition",
            &self.dream_maker_multiplier_condition(),
        )?;
        s.serialize_field(
            "dream_maker_multiplier_condition_value",
            &self.dream_maker_multiplier_condition_value(),
        )?;
        s.serialize_field(
            "dream_maker_multiplier_max",
            &self.dream_maker_multiplier_max(),
        )?;
        s.serialize_field("dream_maker_days", &self.dream_maker_days())?;
        s.serialize_field("dream_maker_action_point", &self.dream_maker_action_point())?;
        s.serialize_field("dream_maker_parcel_type", &self.dream_maker_parcel_type())?;
        s.serialize_field("dream_maker_parcel_id", &self.dream_maker_parcel_id())?;
        s.serialize_field(
            "dream_maker_daily_point_parcel_type",
            &self.dream_maker_daily_point_parcel_type(),
        )?;
        s.serialize_field(
            "dream_maker_daily_point_id",
            &self.dream_maker_daily_point_id(),
        )?;
        s.serialize_field(
            "dream_maker_parameter_transfer",
            &self.dream_maker_parameter_transfer(),
        )?;
        s.serialize_field("schedule_cost_goods_id", &self.schedule_cost_goods_id())?;
        s.serialize_field(
            "lobby_bgm_change_scenario_id",
            &self.lobby_bgm_change_scenario_id(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamInfoExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamInfoExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field(
            "dream_maker_multiplier_condition",
            &self.dream_maker_multiplier_condition(),
        );
        ds.field(
            "dream_maker_multiplier_condition_value",
            &self.dream_maker_multiplier_condition_value(),
        );
        ds.field(
            "dream_maker_multiplier_max",
            &self.dream_maker_multiplier_max(),
        );
        ds.field("dream_maker_days", &self.dream_maker_days());
        ds.field("dream_maker_action_point", &self.dream_maker_action_point());
        ds.field("dream_maker_parcel_type", &self.dream_maker_parcel_type());
        ds.field("dream_maker_parcel_id", &self.dream_maker_parcel_id());
        ds.field(
            "dream_maker_daily_point_parcel_type",
            &self.dream_maker_daily_point_parcel_type(),
        );
        ds.field(
            "dream_maker_daily_point_id",
            &self.dream_maker_daily_point_id(),
        );
        ds.field(
            "dream_maker_parameter_transfer",
            &self.dream_maker_parameter_transfer(),
        );
        ds.field("schedule_cost_goods_id", &self.schedule_cost_goods_id());
        ds.field(
            "lobby_bgm_change_scenario_id",
            &self.lobby_bgm_change_scenario_id(),
        );
        ds.finish()
    }
}
