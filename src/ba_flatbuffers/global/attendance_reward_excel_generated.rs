extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct AttendanceRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for AttendanceRewardExcel<'a> {
    type Inner = AttendanceRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> AttendanceRewardExcel<'a> {
    pub const VT_ATTENDANCE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DAY: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_ICON: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn attendance_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceRewardExcel::VT_ATTENDANCE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn day(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceRewardExcel::VT_DAY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_icon(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceRewardExcel::VT_REWARD_ICON,
                None,
            )
        }
    }
    #[inline]
    pub fn reward_parcel_type(&self) -> Option<::flatbuffers::Vector<'a, ParcelType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, ParcelType>>>(
                    AttendanceRewardExcel::VT_REWARD_PARCEL_TYPE,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    AttendanceRewardExcel::VT_REWARD_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    AttendanceRewardExcel::VT_REWARD_AMOUNT,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for AttendanceRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("attendance_id", Self::VT_ATTENDANCE_ID, false)?
            .visit_field::<i64>("day", Self::VT_DAY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "reward_icon",
                Self::VT_REWARD_ICON,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ParcelType>>>(
                "reward_parcel_type",
                Self::VT_REWARD_PARCEL_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "reward_id",
                Self::VT_REWARD_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "reward_amount",
                Self::VT_REWARD_AMOUNT,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for AttendanceRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AttendanceRewardExcel", 6)?;
        s.serialize_field("attendance_id", &self.attendance_id())?;
        s.serialize_field("day", &self.day())?;
        if let Some(f) = self.reward_icon() {
            s.serialize_field("reward_icon", &f)?;
        } else {
            s.skip_field("reward_icon")?;
        }
        if let Some(f) = self.reward_parcel_type() {
            s.serialize_field("reward_parcel_type", &f)?;
        } else {
            s.skip_field("reward_parcel_type")?;
        }
        if let Some(f) = self.reward_id() {
            s.serialize_field("reward_id", &f)?;
        } else {
            s.skip_field("reward_id")?;
        }
        if let Some(f) = self.reward_amount() {
            s.serialize_field("reward_amount", &f)?;
        } else {
            s.skip_field("reward_amount")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for AttendanceRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("AttendanceRewardExcel");
        ds.field("attendance_id", &self.attendance_id());
        ds.field("day", &self.day());
        ds.field("reward_icon", &self.reward_icon());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_id", &self.reward_id());
        ds.field("reward_amount", &self.reward_amount());
        ds.finish()
    }
}
