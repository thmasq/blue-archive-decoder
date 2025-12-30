extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MissionEmergencyCompleteExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MissionEmergencyCompleteExcel<'a> {
    type Inner = MissionEmergencyCompleteExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MissionEmergencyCompleteExcel<'a> {
    pub const VT_MISSION_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EMERGENCY_COMPLETE: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn mission_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MissionEmergencyCompleteExcel::VT_MISSION_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn emergency_complete(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MissionEmergencyCompleteExcel::VT_EMERGENCY_COMPLETE,
                    Some(false),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MissionEmergencyCompleteExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("mission_id", Self::VT_MISSION_ID, false)?
            .visit_field::<bool>("emergency_complete", Self::VT_EMERGENCY_COMPLETE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MissionEmergencyCompleteExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MissionEmergencyCompleteExcel", 2)?;
        s.serialize_field("mission_id", &self.mission_id())?;
        s.serialize_field("emergency_complete", &self.emergency_complete())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MissionEmergencyCompleteExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MissionEmergencyCompleteExcel");
        ds.field("mission_id", &self.mission_id());
        ds.field("emergency_complete", &self.emergency_complete());
        ds.finish()
    }
}
