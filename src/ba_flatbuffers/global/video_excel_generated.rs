extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::Nation;

#[derive(Copy, Clone, PartialEq)]
pub struct VideoExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for VideoExcel<'a> {
    type Inner = VideoExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> VideoExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_NATION: ::flatbuffers::VOffsetT = 6;
    pub const VT_VIDEO_PATH: ::flatbuffers::VOffsetT = 8;
    pub const VT_VIDEO_TEEN_PATH: ::flatbuffers::VOffsetT = 10;
    pub const VT_SOUND_PATH: ::flatbuffers::VOffsetT = 12;
    pub const VT_SOUND_VOLUME: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(VideoExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn nation(&self) -> Option<::flatbuffers::Vector<'a, Nation>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Nation>>>(
                    VideoExcel::VT_NATION,
                    None,
                )
        }
    }
    #[inline]
    pub fn video_path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(VideoExcel::VT_VIDEO_PATH, None)
        }
    }
    #[inline]
    pub fn video_teen_path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(VideoExcel::VT_VIDEO_TEEN_PATH, None)
        }
    }
    #[inline]
    pub fn sound_path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(VideoExcel::VT_SOUND_PATH, None)
        }
    }
    #[inline]
    pub fn sound_volume(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    VideoExcel::VT_SOUND_VOLUME,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for VideoExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, Nation>>>(
                "nation",
                Self::VT_NATION,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("video_path", Self::VT_VIDEO_PATH, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("video_teen_path", Self::VT_VIDEO_TEEN_PATH, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("sound_path", Self::VT_SOUND_PATH, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "sound_volume",
                Self::VT_SOUND_VOLUME,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for VideoExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VideoExcel", 6)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.nation() {
            s.serialize_field("nation", &f)?;
        } else {
            s.skip_field("nation")?;
        }
        if let Some(f) = self.video_path() {
            s.serialize_field("video_path", &f)?;
        } else {
            s.skip_field("video_path")?;
        }
        if let Some(f) = self.video_teen_path() {
            s.serialize_field("video_teen_path", &f)?;
        } else {
            s.skip_field("video_teen_path")?;
        }
        if let Some(f) = self.sound_path() {
            s.serialize_field("sound_path", &f)?;
        } else {
            s.skip_field("sound_path")?;
        }
        if let Some(f) = self.sound_volume() {
            s.serialize_field("sound_volume", &f)?;
        } else {
            s.skip_field("sound_volume")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for VideoExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("VideoExcel");
        ds.field("id", &self.id());
        ds.field("nation", &self.nation());
        ds.field("video_path", &self.video_path());
        ds.field("video_teen_path", &self.video_teen_path());
        ds.field("sound_path", &self.sound_path());
        ds.field("sound_volume", &self.sound_volume());
        ds.finish()
    }
}
