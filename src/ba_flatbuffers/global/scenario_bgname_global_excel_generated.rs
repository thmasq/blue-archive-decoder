extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioBGName_GlobalExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioBGName_GlobalExcel<'a> {
    type Inner = ScenarioBGName_GlobalExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioBGName_GlobalExcel<'a> {
    pub const VT_GROUP_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME_KR: ::flatbuffers::VOffsetT = 6;
    pub const VT_NAME_TW: ::flatbuffers::VOffsetT = 8;
    pub const VT_NAME_ASIA: ::flatbuffers::VOffsetT = 10;
    pub const VT_NAME_NA: ::flatbuffers::VOffsetT = 12;
    pub const VT_NAME_GLOBAL: ::flatbuffers::VOffsetT = 14;
    pub const VT_NAME_TEEN: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn group_name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_GROUP_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_kr(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_KR, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_tw(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_TW, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_asia(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_ASIA, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_na(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_NA, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_global(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_GLOBAL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_teen(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGName_GlobalExcel::VT_NAME_TEEN, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioBGName_GlobalExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("group_name", Self::VT_GROUP_NAME, false)?
            .visit_field::<u32>("name_kr", Self::VT_NAME_KR, false)?
            .visit_field::<u32>("name_tw", Self::VT_NAME_TW, false)?
            .visit_field::<u32>("name_asia", Self::VT_NAME_ASIA, false)?
            .visit_field::<u32>("name_na", Self::VT_NAME_NA, false)?
            .visit_field::<u32>("name_global", Self::VT_NAME_GLOBAL, false)?
            .visit_field::<u32>("name_teen", Self::VT_NAME_TEEN, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioBGName_GlobalExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioBGName_GlobalExcel", 7)?;
        s.serialize_field("group_name", &self.group_name())?;
        s.serialize_field("name_kr", &self.name_kr())?;
        s.serialize_field("name_tw", &self.name_tw())?;
        s.serialize_field("name_asia", &self.name_asia())?;
        s.serialize_field("name_na", &self.name_na())?;
        s.serialize_field("name_global", &self.name_global())?;
        s.serialize_field("name_teen", &self.name_teen())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioBGName_GlobalExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioBGName_GlobalExcel");
        ds.field("group_name", &self.group_name());
        ds.field("name_kr", &self.name_kr());
        ds.field("name_tw", &self.name_tw());
        ds.field("name_asia", &self.name_asia());
        ds.field("name_na", &self.name_na());
        ds.field("name_global", &self.name_global());
        ds.field("name_teen", &self.name_teen());
        ds.finish()
    }
}
