extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ToastType;

#[derive(Copy, Clone, PartialEq)]
pub struct ToastExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ToastExcel<'a> {
    type Inner = ToastExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ToastExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TOAST_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_MISSION_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_TEXT_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_LIFE_TIME: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u32>(ToastExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn toast_type(&self) -> ToastType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ToastType>(ToastExcel::VT_TOAST_TYPE, Some(ToastType::None))
                .unwrap()
        }
    }
    #[inline]
    pub fn mission_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ToastExcel::VT_MISSION_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn text_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ToastExcel::VT_TEXT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn life_time(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ToastExcel::VT_LIFE_TIME, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ToastExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("id", Self::VT_ID, false)?
            .visit_field::<ToastType>("toast_type", Self::VT_TOAST_TYPE, false)?
            .visit_field::<u32>("mission_id", Self::VT_MISSION_ID, false)?
            .visit_field::<u32>("text_id", Self::VT_TEXT_ID, false)?
            .visit_field::<i64>("life_time", Self::VT_LIFE_TIME, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ToastExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ToastExcel", 5)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("toast_type", &self.toast_type())?;
        s.serialize_field("mission_id", &self.mission_id())?;
        s.serialize_field("text_id", &self.text_id())?;
        s.serialize_field("life_time", &self.life_time())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ToastExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ToastExcel");
        ds.field("id", &self.id());
        ds.field("toast_type", &self.toast_type());
        ds.field("mission_id", &self.mission_id());
        ds.field("text_id", &self.text_id());
        ds.field("life_time", &self.life_time());
        ds.finish()
    }
}
