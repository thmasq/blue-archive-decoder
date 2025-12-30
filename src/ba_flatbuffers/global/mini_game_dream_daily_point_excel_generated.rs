extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamDailyPointExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamDailyPointExcel<'a> {
    type Inner = MiniGameDreamDailyPointExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamDailyPointExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_TOTAL_PARAMETER_MIN: ::flatbuffers::VOffsetT = 8;
    pub const VT_TOTAL_PARAMETER_MAX: ::flatbuffers::VOffsetT = 10;
    pub const VT_DAILY_POINT_COEFFICIENT: ::flatbuffers::VOffsetT = 12;
    pub const VT_DAILY_POINT_CORRECTION_VALUE: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamDailyPointExcel::VT_UNIQUE_ID, Some(0))
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
                .get::<i64>(MiniGameDreamDailyPointExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn total_parameter_min(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamDailyPointExcel::VT_TOTAL_PARAMETER_MIN,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn total_parameter_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamDailyPointExcel::VT_TOTAL_PARAMETER_MAX,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn daily_point_coefficient(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamDailyPointExcel::VT_DAILY_POINT_COEFFICIENT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn daily_point_correction_value(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamDailyPointExcel::VT_DAILY_POINT_CORRECTION_VALUE,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamDailyPointExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("total_parameter_min", Self::VT_TOTAL_PARAMETER_MIN, false)?
            .visit_field::<i64>("total_parameter_max", Self::VT_TOTAL_PARAMETER_MAX, false)?
            .visit_field::<i64>(
                "daily_point_coefficient",
                Self::VT_DAILY_POINT_COEFFICIENT,
                false,
            )?
            .visit_field::<i64>(
                "daily_point_correction_value",
                Self::VT_DAILY_POINT_CORRECTION_VALUE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamDailyPointExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamDailyPointExcel", 6)?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("total_parameter_min", &self.total_parameter_min())?;
        s.serialize_field("total_parameter_max", &self.total_parameter_max())?;
        s.serialize_field("daily_point_coefficient", &self.daily_point_coefficient())?;
        s.serialize_field(
            "daily_point_correction_value",
            &self.daily_point_correction_value(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamDailyPointExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamDailyPointExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("total_parameter_min", &self.total_parameter_min());
        ds.field("total_parameter_max", &self.total_parameter_max());
        ds.field("daily_point_coefficient", &self.daily_point_coefficient());
        ds.field(
            "daily_point_correction_value",
            &self.daily_point_correction_value(),
        );
        ds.finish()
    }
}
