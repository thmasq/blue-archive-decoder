extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct LocalizeEtcExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for LocalizeEtcExcel<'a> {
    type Inner = LocalizeEtcExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> LocalizeEtcExcel<'a> {
    pub const VT_KEY: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME_KR: ::flatbuffers::VOffsetT = 6;
    pub const VT_DESCRIPTION_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_NAME_JP: ::flatbuffers::VOffsetT = 10;
    pub const VT_DESCRIPTION_JP: ::flatbuffers::VOffsetT = 12;
    pub const VT_NAME_TH: ::flatbuffers::VOffsetT = 14;
    pub const VT_DESCRIPTION_TH: ::flatbuffers::VOffsetT = 16;
    pub const VT_NAME_TW: ::flatbuffers::VOffsetT = 18;
    pub const VT_DESCRIPTION_TW: ::flatbuffers::VOffsetT = 20;
    pub const VT_NAME_EN: ::flatbuffers::VOffsetT = 22;
    pub const VT_DESCRIPTION_EN: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn key(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(LocalizeEtcExcel::VT_KEY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeEtcExcel::VT_NAME_KR, None)
        }
    }
    #[inline]
    pub fn description_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                LocalizeEtcExcel::VT_DESCRIPTION_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn name_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeEtcExcel::VT_NAME_JP, None)
        }
    }
    #[inline]
    pub fn description_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                LocalizeEtcExcel::VT_DESCRIPTION_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn name_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeEtcExcel::VT_NAME_TH, None)
        }
    }
    #[inline]
    pub fn description_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                LocalizeEtcExcel::VT_DESCRIPTION_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn name_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeEtcExcel::VT_NAME_TW, None)
        }
    }
    #[inline]
    pub fn description_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                LocalizeEtcExcel::VT_DESCRIPTION_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn name_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(LocalizeEtcExcel::VT_NAME_EN, None)
        }
    }
    #[inline]
    pub fn description_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                LocalizeEtcExcel::VT_DESCRIPTION_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for LocalizeEtcExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("key", Self::VT_KEY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_kr",
                Self::VT_NAME_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "description_kr",
                Self::VT_DESCRIPTION_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_jp",
                Self::VT_NAME_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "description_jp",
                Self::VT_DESCRIPTION_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_th",
                Self::VT_NAME_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "description_th",
                Self::VT_DESCRIPTION_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_tw",
                Self::VT_NAME_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "description_tw",
                Self::VT_DESCRIPTION_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_en",
                Self::VT_NAME_EN,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "description_en",
                Self::VT_DESCRIPTION_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for LocalizeEtcExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LocalizeEtcExcel", 11)?;
        s.serialize_field("key", &self.key())?;
        if let Some(f) = self.name_kr() {
            s.serialize_field("name_kr", &f)?;
        } else {
            s.skip_field("name_kr")?;
        }
        if let Some(f) = self.description_kr() {
            s.serialize_field("description_kr", &f)?;
        } else {
            s.skip_field("description_kr")?;
        }
        if let Some(f) = self.name_jp() {
            s.serialize_field("name_jp", &f)?;
        } else {
            s.skip_field("name_jp")?;
        }
        if let Some(f) = self.description_jp() {
            s.serialize_field("description_jp", &f)?;
        } else {
            s.skip_field("description_jp")?;
        }
        if let Some(f) = self.name_th() {
            s.serialize_field("name_th", &f)?;
        } else {
            s.skip_field("name_th")?;
        }
        if let Some(f) = self.description_th() {
            s.serialize_field("description_th", &f)?;
        } else {
            s.skip_field("description_th")?;
        }
        if let Some(f) = self.name_tw() {
            s.serialize_field("name_tw", &f)?;
        } else {
            s.skip_field("name_tw")?;
        }
        if let Some(f) = self.description_tw() {
            s.serialize_field("description_tw", &f)?;
        } else {
            s.skip_field("description_tw")?;
        }
        if let Some(f) = self.name_en() {
            s.serialize_field("name_en", &f)?;
        } else {
            s.skip_field("name_en")?;
        }
        if let Some(f) = self.description_en() {
            s.serialize_field("description_en", &f)?;
        } else {
            s.skip_field("description_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for LocalizeEtcExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("LocalizeEtcExcel");
        ds.field("key", &self.key());
        ds.field("name_kr", &self.name_kr());
        ds.field("description_kr", &self.description_kr());
        ds.field("name_jp", &self.name_jp());
        ds.field("description_jp", &self.description_jp());
        ds.field("name_th", &self.name_th());
        ds.field("description_th", &self.description_th());
        ds.field("name_tw", &self.name_tw());
        ds.field("description_tw", &self.description_tw());
        ds.field("name_en", &self.name_en());
        ds.field("description_en", &self.description_en());
        ds.finish()
    }
}
