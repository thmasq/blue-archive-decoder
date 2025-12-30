extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct BGM_GlobalExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for BGM_GlobalExcel<'a> {
    type Inner = BGM_GlobalExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> BGM_GlobalExcel<'a> {
    pub const VT_GROUP_BGM_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_BGM_ID_KR: ::flatbuffers::VOffsetT = 6;
    pub const VT_BGM_ID_JP: ::flatbuffers::VOffsetT = 8;
    pub const VT_BGM_ID_TH: ::flatbuffers::VOffsetT = 10;
    pub const VT_BGM_ID_TW: ::flatbuffers::VOffsetT = 12;
    pub const VT_BGM_ID_EN: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn group_bgm_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_GROUP_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id_kr(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_BGM_ID_KR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id_jp(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_BGM_ID_JP, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id_th(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_BGM_ID_TH, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id_tw(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_BGM_ID_TW, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bgm_id_en(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGM_GlobalExcel::VT_BGM_ID_EN, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for BGM_GlobalExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_bgm_id", Self::VT_GROUP_BGM_ID, false)?
            .visit_field::<i64>("bgm_id_kr", Self::VT_BGM_ID_KR, false)?
            .visit_field::<i64>("bgm_id_jp", Self::VT_BGM_ID_JP, false)?
            .visit_field::<i64>("bgm_id_th", Self::VT_BGM_ID_TH, false)?
            .visit_field::<i64>("bgm_id_tw", Self::VT_BGM_ID_TW, false)?
            .visit_field::<i64>("bgm_id_en", Self::VT_BGM_ID_EN, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for BGM_GlobalExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BGM_GlobalExcel", 6)?;
        s.serialize_field("group_bgm_id", &self.group_bgm_id())?;
        s.serialize_field("bgm_id_kr", &self.bgm_id_kr())?;
        s.serialize_field("bgm_id_jp", &self.bgm_id_jp())?;
        s.serialize_field("bgm_id_th", &self.bgm_id_th())?;
        s.serialize_field("bgm_id_tw", &self.bgm_id_tw())?;
        s.serialize_field("bgm_id_en", &self.bgm_id_en())?;
        s.end()
    }
}

impl ::core::fmt::Debug for BGM_GlobalExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("BGM_GlobalExcel");
        ds.field("group_bgm_id", &self.group_bgm_id());
        ds.field("bgm_id_kr", &self.bgm_id_kr());
        ds.field("bgm_id_jp", &self.bgm_id_jp());
        ds.field("bgm_id_th", &self.bgm_id_th());
        ds.field("bgm_id_tw", &self.bgm_id_tw());
        ds.field("bgm_id_en", &self.bgm_id_en());
        ds.finish()
    }
}
