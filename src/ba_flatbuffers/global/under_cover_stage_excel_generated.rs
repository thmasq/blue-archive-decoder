extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct UnderCoverStageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for UnderCoverStageExcel<'a> {
    type Inner = UnderCoverStageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> UnderCoverStageExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_STAGE_NAME_FILE: ::flatbuffers::VOffsetT = 6;
    pub const VT_STAGE_TRY_COUNT: ::flatbuffers::VOffsetT = 8;
    pub const VT_APPLY_SKIP: ::flatbuffers::VOffsetT = 10;
    pub const VT_SKIP_COUNT: ::flatbuffers::VOffsetT = 12;
    pub const VT_SHOW_CLEAR_SCENE: ::flatbuffers::VOffsetT = 14;
    pub const VT_STAGE_TIPS: ::flatbuffers::VOffsetT = 16;
    pub const VT_STAGE_NAME: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(UnderCoverStageExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_name_file(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                UnderCoverStageExcel::VT_STAGE_NAME_FILE,
                None,
            )
        }
    }
    #[inline]
    pub fn stage_try_count(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(UnderCoverStageExcel::VT_STAGE_TRY_COUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn apply_skip(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(UnderCoverStageExcel::VT_APPLY_SKIP, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn skip_count(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(UnderCoverStageExcel::VT_SKIP_COUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn show_clear_scene(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(UnderCoverStageExcel::VT_SHOW_CLEAR_SCENE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_tips(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(UnderCoverStageExcel::VT_STAGE_TIPS, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(UnderCoverStageExcel::VT_STAGE_NAME, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for UnderCoverStageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "stage_name_file",
                Self::VT_STAGE_NAME_FILE,
                false,
            )?
            .visit_field::<i32>("stage_try_count", Self::VT_STAGE_TRY_COUNT, false)?
            .visit_field::<bool>("apply_skip", Self::VT_APPLY_SKIP, false)?
            .visit_field::<i32>("skip_count", Self::VT_SKIP_COUNT, false)?
            .visit_field::<bool>("show_clear_scene", Self::VT_SHOW_CLEAR_SCENE, false)?
            .visit_field::<u32>("stage_tips", Self::VT_STAGE_TIPS, false)?
            .visit_field::<u32>("stage_name", Self::VT_STAGE_NAME, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for UnderCoverStageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("UnderCoverStageExcel", 8)?;
        s.serialize_field("group_id", &self.group_id())?;
        if let Some(f) = self.stage_name_file() {
            s.serialize_field("stage_name_file", &f)?;
        } else {
            s.skip_field("stage_name_file")?;
        }
        s.serialize_field("stage_try_count", &self.stage_try_count())?;
        s.serialize_field("apply_skip", &self.apply_skip())?;
        s.serialize_field("skip_count", &self.skip_count())?;
        s.serialize_field("show_clear_scene", &self.show_clear_scene())?;
        s.serialize_field("stage_tips", &self.stage_tips())?;
        s.serialize_field("stage_name", &self.stage_name())?;
        s.end()
    }
}

impl ::core::fmt::Debug for UnderCoverStageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("UnderCoverStageExcel");
        ds.field("group_id", &self.group_id());
        ds.field("stage_name_file", &self.stage_name_file());
        ds.field("stage_try_count", &self.stage_try_count());
        ds.field("apply_skip", &self.apply_skip());
        ds.field("skip_count", &self.skip_count());
        ds.field("show_clear_scene", &self.show_clear_scene());
        ds.field("stage_tips", &self.stage_tips());
        ds.field("stage_name", &self.stage_name());
        ds.finish()
    }
}
