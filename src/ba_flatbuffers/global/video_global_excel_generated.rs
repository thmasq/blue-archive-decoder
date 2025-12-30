extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct Video_GlobalExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for Video_GlobalExcel<'a> {
    type Inner = Video_GlobalExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> Video_GlobalExcel<'a> {
    pub const VT_VIDEO_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_VIDEO_PATH_KR: ::flatbuffers::VOffsetT = 6;
    pub const VT_VIDEO_TEEN_PATH_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_VIDEO_PATH_TH: ::flatbuffers::VOffsetT = 10;
    pub const VT_VIDEO_TEEN_PATH_TH: ::flatbuffers::VOffsetT = 12;
    pub const VT_VIDEO_PATH_TW: ::flatbuffers::VOffsetT = 14;
    pub const VT_VIDEO_TEEN_PATH_TW: ::flatbuffers::VOffsetT = 16;
    pub const VT_VIDEO_PATH_EN: ::flatbuffers::VOffsetT = 18;
    pub const VT_VIDEO_TEEN_PATH_EN: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn video_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(Video_GlobalExcel::VT_VIDEO_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn video_path_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_PATH_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn video_teen_path_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_TEEN_PATH_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn video_path_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_PATH_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn video_teen_path_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_TEEN_PATH_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn video_path_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_PATH_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn video_teen_path_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_TEEN_PATH_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn video_path_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_PATH_EN,
                None,
            )
        }
    }
    #[inline]
    pub fn video_teen_path_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                Video_GlobalExcel::VT_VIDEO_TEEN_PATH_EN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for Video_GlobalExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("video_id", Self::VT_VIDEO_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_path_kr",
                Self::VT_VIDEO_PATH_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_teen_path_kr",
                Self::VT_VIDEO_TEEN_PATH_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_path_th",
                Self::VT_VIDEO_PATH_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_teen_path_th",
                Self::VT_VIDEO_TEEN_PATH_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_path_tw",
                Self::VT_VIDEO_PATH_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_teen_path_tw",
                Self::VT_VIDEO_TEEN_PATH_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_path_en",
                Self::VT_VIDEO_PATH_EN,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "video_teen_path_en",
                Self::VT_VIDEO_TEEN_PATH_EN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for Video_GlobalExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Video_GlobalExcel", 9)?;
        s.serialize_field("video_id", &self.video_id())?;
        if let Some(f) = self.video_path_kr() {
            s.serialize_field("video_path_kr", &f)?;
        } else {
            s.skip_field("video_path_kr")?;
        }
        if let Some(f) = self.video_teen_path_kr() {
            s.serialize_field("video_teen_path_kr", &f)?;
        } else {
            s.skip_field("video_teen_path_kr")?;
        }
        if let Some(f) = self.video_path_th() {
            s.serialize_field("video_path_th", &f)?;
        } else {
            s.skip_field("video_path_th")?;
        }
        if let Some(f) = self.video_teen_path_th() {
            s.serialize_field("video_teen_path_th", &f)?;
        } else {
            s.skip_field("video_teen_path_th")?;
        }
        if let Some(f) = self.video_path_tw() {
            s.serialize_field("video_path_tw", &f)?;
        } else {
            s.skip_field("video_path_tw")?;
        }
        if let Some(f) = self.video_teen_path_tw() {
            s.serialize_field("video_teen_path_tw", &f)?;
        } else {
            s.skip_field("video_teen_path_tw")?;
        }
        if let Some(f) = self.video_path_en() {
            s.serialize_field("video_path_en", &f)?;
        } else {
            s.skip_field("video_path_en")?;
        }
        if let Some(f) = self.video_teen_path_en() {
            s.serialize_field("video_teen_path_en", &f)?;
        } else {
            s.skip_field("video_teen_path_en")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for Video_GlobalExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("Video_GlobalExcel");
        ds.field("video_id", &self.video_id());
        ds.field("video_path_kr", &self.video_path_kr());
        ds.field("video_teen_path_kr", &self.video_teen_path_kr());
        ds.field("video_path_th", &self.video_path_th());
        ds.field("video_teen_path_th", &self.video_teen_path_th());
        ds.field("video_path_tw", &self.video_path_tw());
        ds.field("video_teen_path_tw", &self.video_teen_path_tw());
        ds.field("video_path_en", &self.video_path_en());
        ds.field("video_teen_path_en", &self.video_teen_path_en());
        ds.finish()
    }
}
