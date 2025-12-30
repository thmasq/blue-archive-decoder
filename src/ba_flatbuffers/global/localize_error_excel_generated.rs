extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::WebAPIErrorLevel;

#[derive(Copy, Clone, PartialEq)]
pub struct LocalizeErrorExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for LocalizeErrorExcel<'a> {
    type Inner = LocalizeErrorExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> LocalizeErrorExcel<'a> {
    pub const VT_KEY: ::flatbuffers::VOffsetT = 4;
    pub const VT_ERROR_LEVEL: ::flatbuffers::VOffsetT = 6;
    pub const VT_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_JP: ::flatbuffers::VOffsetT = 10;
    pub const VT_TH: ::flatbuffers::VOffsetT = 12;
    pub const VT_TW: ::flatbuffers::VOffsetT = 14;
    pub const VT_EN: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn key(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(LocalizeErrorExcel::VT_KEY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn error_level(&self) -> WebAPIErrorLevel {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<WebAPIErrorLevel>(
                    LocalizeErrorExcel::VT_ERROR_LEVEL,
                    Some(WebAPIErrorLevel::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeErrorExcel::VT_KR, None)
        }
    }
    #[inline]
    pub fn jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeErrorExcel::VT_JP, None)
        }
    }
    #[inline]
    pub fn th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeErrorExcel::VT_TH, None)
        }
    }
    #[inline]
    pub fn tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeErrorExcel::VT_TW, None)
        }
    }
    #[inline]
    pub fn en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeErrorExcel::VT_EN, None)
        }
    }
}

impl ::flatbuffers::Verifiable for LocalizeErrorExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("key", Self::VT_KEY, false)?
            .visit_field::<WebAPIErrorLevel>("error_level", Self::VT_ERROR_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("kr", Self::VT_KR, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("jp", Self::VT_JP, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("th", Self::VT_TH, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("tw", Self::VT_TW, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("en", Self::VT_EN, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for LocalizeErrorExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LocalizeErrorExcel", 7)?;
        s.serialize_field("key", &self.key())?;
        s.serialize_field("error_level", &self.error_level())?;
        if let Some(f) = self.kr() {
            s.serialize_field("kr", &f)?;
        } else {
            s.skip_field("kr")?;
        }
        if let Some(f) = self.jp() {
            s.serialize_field("jp", &f)?;
        } else {
            s.skip_field("jp")?;
        }
        if let Some(f) = self.th() {
            s.serialize_field("th", &f)?;
        } else {
            s.skip_field("th")?;
        }
        if let Some(f) = self.tw() {
            s.serialize_field("tw", &f)?;
        } else {
            s.skip_field("tw")?;
        }
        if let Some(f) = self.en() {
            s.serialize_field("en", &f)?;
        } else {
            s.skip_field("en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for LocalizeErrorExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("LocalizeErrorExcel");
        ds.field("key", &self.key());
        ds.field("error_level", &self.error_level());
        ds.field("kr", &self.kr());
        ds.field("jp", &self.jp());
        ds.field("th", &self.th());
        ds.field("tw", &self.tw());
        ds.field("en", &self.en());
        ds.finish()
    }
}
