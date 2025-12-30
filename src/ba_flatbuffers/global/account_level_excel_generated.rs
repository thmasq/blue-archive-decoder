extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct AccountLevelExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for AccountLevelExcel<'a> {
    type Inner = AccountLevelExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> AccountLevelExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LEVEL: ::flatbuffers::VOffsetT = 6;
    pub const VT_EXP: ::flatbuffers::VOffsetT = 8;
    pub const VT_NEWBIE_EXP_RATIO: ::flatbuffers::VOffsetT = 10;
    pub const VT_CLOSE_INTERVAL: ::flatbuffers::VOffsetT = 12;
    pub const VT_AP_AUTO_CHARGE_MAX: ::flatbuffers::VOffsetT = 14;
    pub const VT_NEED_REPORT_EVENT: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AccountLevelExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AccountLevelExcel::VT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn exp(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AccountLevelExcel::VT_EXP, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn newbie_exp_ratio(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(AccountLevelExcel::VT_NEWBIE_EXP_RATIO, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn close_interval(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(AccountLevelExcel::VT_CLOSE_INTERVAL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ap_auto_charge_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AccountLevelExcel::VT_AP_AUTO_CHARGE_MAX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn need_report_event(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(AccountLevelExcel::VT_NEED_REPORT_EVENT, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for AccountLevelExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("level", Self::VT_LEVEL, false)?
            .visit_field::<i64>("exp", Self::VT_EXP, false)?
            .visit_field::<i32>("newbie_exp_ratio", Self::VT_NEWBIE_EXP_RATIO, false)?
            .visit_field::<i32>("close_interval", Self::VT_CLOSE_INTERVAL, false)?
            .visit_field::<i64>("ap_auto_charge_max", Self::VT_AP_AUTO_CHARGE_MAX, false)?
            .visit_field::<bool>("need_report_event", Self::VT_NEED_REPORT_EVENT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for AccountLevelExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AccountLevelExcel", 7)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("level", &self.level())?;
        s.serialize_field("exp", &self.exp())?;
        s.serialize_field("newbie_exp_ratio", &self.newbie_exp_ratio())?;
        s.serialize_field("close_interval", &self.close_interval())?;
        s.serialize_field("ap_auto_charge_max", &self.ap_auto_charge_max())?;
        s.serialize_field("need_report_event", &self.need_report_event())?;
        s.end()
    }
}

impl ::core::fmt::Debug for AccountLevelExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("AccountLevelExcel");
        ds.field("id", &self.id());
        ds.field("level", &self.level());
        ds.field("exp", &self.exp());
        ds.field("newbie_exp_ratio", &self.newbie_exp_ratio());
        ds.field("close_interval", &self.close_interval());
        ds.field("ap_auto_charge_max", &self.ap_auto_charge_max());
        ds.field("need_report_event", &self.need_report_event());
        ds.finish()
    }
}
