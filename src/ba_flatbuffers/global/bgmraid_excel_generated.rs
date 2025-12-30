extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct BGMRaidExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for BGMRaidExcel<'a> {
    type Inner = BGMRaidExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> BGMRaidExcel<'a> {
    pub const VT_STAGE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_PHASE_INDEX: ::flatbuffers::VOffsetT = 6;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGMRaidExcel::VT_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn phase_index(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BGMRaidExcel::VT_PHASE_INDEX, Some(0))
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
                .get::<i64>(BGMRaidExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for BGMRaidExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("stage_id", Self::VT_STAGE_ID, false)?
            .visit_field::<i64>("phase_index", Self::VT_PHASE_INDEX, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for BGMRaidExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BGMRaidExcel", 3)?;
        s.serialize_field("stage_id", &self.stage_id())?;
        s.serialize_field("phase_index", &self.phase_index())?;
        s.serialize_field("bgm_id", &self.bgm_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for BGMRaidExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("BGMRaidExcel");
        ds.field("stage_id", &self.stage_id());
        ds.field("phase_index", &self.phase_index());
        ds.field("bgm_id", &self.bgm_id());
        ds.finish()
    }
}
