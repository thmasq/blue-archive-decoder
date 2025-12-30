extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct BGMUIExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for BGMUIExcel<'a> {
    type Inner = BGMUIExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> BGMUIExcel<'a> {
    pub const VT_UI_PREFAB: ::flatbuffers::VOffsetT = 4;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_BGM_ID2ND: ::flatbuffers::VOffsetT = 8;
    pub const VT_BGM_ID3RD: ::flatbuffers::VOffsetT = 10;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn ui_prefab(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(BGMUIExcel::VT_UI_PREFAB, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGMUIExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id2nd(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGMUIExcel::VT_BGM_ID2ND, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id3rd(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGMUIExcel::VT_BGM_ID3RD, Some(0))
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
                .get::<i64>(BGMUIExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for BGMUIExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("ui_prefab", Self::VT_UI_PREFAB, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<i64>("bgm_id2nd", Self::VT_BGM_ID2ND, false)?
            .visit_field::<i64>("bgm_id3rd", Self::VT_BGM_ID3RD, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for BGMUIExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BGMUIExcel", 5)?;
        s.serialize_field("ui_prefab", &self.ui_prefab())?;
        s.serialize_field("bgm_id", &self.bgm_id())?;
        s.serialize_field("bgm_id2nd", &self.bgm_id2nd())?;
        s.serialize_field("bgm_id3rd", &self.bgm_id3rd())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for BGMUIExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("BGMUIExcel");
        ds.field("ui_prefab", &self.ui_prefab());
        ds.field("bgm_id", &self.bgm_id());
        ds.field("bgm_id2nd", &self.bgm_id2nd());
        ds.field("bgm_id3rd", &self.bgm_id3rd());
        ds.field("event_content_id", &self.event_content_id());
        ds.finish()
    }
}
