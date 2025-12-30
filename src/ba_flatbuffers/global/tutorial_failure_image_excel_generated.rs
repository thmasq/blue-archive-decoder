extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::TutorialFailureContentType;

#[derive(Copy, Clone, PartialEq)]
pub struct TutorialFailureImageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for TutorialFailureImageExcel<'a> {
    type Inner = TutorialFailureImageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> TutorialFailureImageExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CONTENTS: ::flatbuffers::VOffsetT = 6;
    pub const VT_TYPE_: ::flatbuffers::VOffsetT = 8;
    pub const VT_IMAGE_PATH_KR: ::flatbuffers::VOffsetT = 10;
    pub const VT_IMAGE_PATH_JP: ::flatbuffers::VOffsetT = 12;
    pub const VT_IMAGE_PATH_TH: ::flatbuffers::VOffsetT = 14;
    pub const VT_IMAGE_PATH_TW: ::flatbuffers::VOffsetT = 16;
    pub const VT_IMAGE_PATH_EN: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(TutorialFailureImageExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn contents(&self) -> TutorialFailureContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<TutorialFailureContentType>(
                    TutorialFailureImageExcel::VT_CONTENTS,
                    Some(TutorialFailureContentType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn type_(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_TYPE_,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_IMAGE_PATH_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_IMAGE_PATH_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_IMAGE_PATH_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_IMAGE_PATH_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn image_path_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                TutorialFailureImageExcel::VT_IMAGE_PATH_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for TutorialFailureImageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<TutorialFailureContentType>("contents", Self::VT_CONTENTS, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("type_", Self::VT_TYPE_, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_kr",
                Self::VT_IMAGE_PATH_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_jp",
                Self::VT_IMAGE_PATH_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_th",
                Self::VT_IMAGE_PATH_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_tw",
                Self::VT_IMAGE_PATH_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "image_path_en",
                Self::VT_IMAGE_PATH_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for TutorialFailureImageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TutorialFailureImageExcel", 8)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("contents", &self.contents())?;
        if let Some(f) = self.type_() {
            s.serialize_field("type_", &f)?;
        } else {
            s.skip_field("type_")?;
        }
        if let Some(f) = self.image_path_kr() {
            s.serialize_field("image_path_kr", &f)?;
        } else {
            s.skip_field("image_path_kr")?;
        }
        if let Some(f) = self.image_path_jp() {
            s.serialize_field("image_path_jp", &f)?;
        } else {
            s.skip_field("image_path_jp")?;
        }
        if let Some(f) = self.image_path_th() {
            s.serialize_field("image_path_th", &f)?;
        } else {
            s.skip_field("image_path_th")?;
        }
        if let Some(f) = self.image_path_tw() {
            s.serialize_field("image_path_tw", &f)?;
        } else {
            s.skip_field("image_path_tw")?;
        }
        if let Some(f) = self.image_path_en() {
            s.serialize_field("image_path_en", &f)?;
        } else {
            s.skip_field("image_path_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for TutorialFailureImageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("TutorialFailureImageExcel");
        ds.field("id", &self.id());
        ds.field("contents", &self.contents());
        ds.field("type_", &self.type_());
        ds.field("image_path_kr", &self.image_path_kr());
        ds.field("image_path_jp", &self.image_path_jp());
        ds.field("image_path_th", &self.image_path_th());
        ds.field("image_path_tw", &self.image_path_tw());
        ds.field("image_path_en", &self.image_path_en());
        ds.finish()
    }
}
