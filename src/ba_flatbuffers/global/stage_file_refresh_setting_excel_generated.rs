extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct StageFileRefreshSettingExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for StageFileRefreshSettingExcel<'a> {
    type Inner = StageFileRefreshSettingExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> StageFileRefreshSettingExcel<'a> {
    pub const VT_GROUND_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_FORCE_SAVE: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn ground_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StageFileRefreshSettingExcel::VT_GROUND_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn force_save(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(StageFileRefreshSettingExcel::VT_FORCE_SAVE, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for StageFileRefreshSettingExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("ground_id", Self::VT_GROUND_ID, false)?
            .visit_field::<bool>("force_save", Self::VT_FORCE_SAVE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for StageFileRefreshSettingExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StageFileRefreshSettingExcel", 2)?;
        s.serialize_field("ground_id", &self.ground_id())?;
        s.serialize_field("force_save", &self.force_save())?;
        s.end()
    }
}

impl ::core::fmt::Debug for StageFileRefreshSettingExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("StageFileRefreshSettingExcel");
        ds.field("ground_id", &self.ground_id());
        ds.field("force_save", &self.force_save());
        ds.finish()
    }
}
