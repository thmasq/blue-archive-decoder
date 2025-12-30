extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct InformationExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for InformationExcel<'a> {
    type Inner = InformationExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> InformationExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_PAGE_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_IS_PC_BUILD: ::flatbuffers::VOffsetT = 8;
    pub const VT_LOCALIZE_CODE_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_TUTORIAL_PARENT_NAME: ::flatbuffers::VOffsetT = 12;
    pub const VT_UI_NAME: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(InformationExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn page_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(InformationExcel::VT_PAGE_NAME, None)
        }
    }
    #[inline]
    pub fn is_pc_build(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(InformationExcel::VT_IS_PC_BUILD, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_code_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                InformationExcel::VT_LOCALIZE_CODE_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn tutorial_parent_name(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(InformationExcel::VT_TUTORIAL_PARENT_NAME, None)
        }
    }
    #[inline]
    pub fn ui_name(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(InformationExcel::VT_UI_NAME, None)
        }
    }
}

impl ::flatbuffers::Verifiable for InformationExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "page_name",
                Self::VT_PAGE_NAME,
                false,
            )?
            .visit_field::<bool>("is_pc_build", Self::VT_IS_PC_BUILD, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_code_id",
                Self::VT_LOCALIZE_CODE_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("tutorial_parent_name", Self::VT_TUTORIAL_PARENT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("ui_name", Self::VT_UI_NAME, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for InformationExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InformationExcel", 6)?;
        s.serialize_field("group_id", &self.group_id())?;
        if let Some(f) = self.page_name() {
            s.serialize_field("page_name", &f)?;
        } else {
            s.skip_field("page_name")?;
        }
        s.serialize_field("is_pc_build", &self.is_pc_build())?;
        if let Some(f) = self.localize_code_id() {
            s.serialize_field("localize_code_id", &f)?;
        } else {
            s.skip_field("localize_code_id")?;
        }
        if let Some(f) = self.tutorial_parent_name() {
            s.serialize_field("tutorial_parent_name", &f)?;
        } else {
            s.skip_field("tutorial_parent_name")?;
        }
        if let Some(f) = self.ui_name() {
            s.serialize_field("ui_name", &f)?;
        } else {
            s.skip_field("ui_name")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for InformationExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("InformationExcel");
        ds.field("group_id", &self.group_id());
        ds.field("page_name", &self.page_name());
        ds.field("is_pc_build", &self.is_pc_build());
        ds.field("localize_code_id", &self.localize_code_id());
        ds.field("tutorial_parent_name", &self.tutorial_parent_name());
        ds.field("ui_name", &self.ui_name());
        ds.finish()
    }
}
