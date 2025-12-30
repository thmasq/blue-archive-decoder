extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::Nation;

pub struct BGMExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for BGMExcel<'a> {
    type Inner = BGMExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> BGMExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_NATION: ::flatbuffers::VOffsetT = 6;
    pub const VT_PATH: ::flatbuffers::VOffsetT = 8;
    pub const VT_VOLUME: ::flatbuffers::VOffsetT = 10;
    pub const VT_LOOP_START_TIME: ::flatbuffers::VOffsetT = 12;
    pub const VT_LOOP_END_TIME: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOOP_TRANSTION_TIME: ::flatbuffers::VOffsetT = 16;
    pub const VT_LOOP_OFFSET_TIME: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(BGMExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn nation(&self) -> Option<::flatbuffers::Vector<'a, Nation>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Nation>>>(
                    BGMExcel::VT_NATION,
                    None,
                )
        }
    }
    #[inline]
    pub fn path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(BGMExcel::VT_PATH, None)
        }
    }
    #[inline]
    pub fn volume(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    BGMExcel::VT_VOLUME,
                    None,
                )
        }
    }
    #[inline]
    pub fn loop_start_time(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    BGMExcel::VT_LOOP_START_TIME,
                    None,
                )
        }
    }
    #[inline]
    pub fn loop_end_time(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    BGMExcel::VT_LOOP_END_TIME,
                    None,
                )
        }
    }
    #[inline]
    pub fn loop_transtion_time(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    BGMExcel::VT_LOOP_TRANSTION_TIME,
                    None,
                )
        }
    }
    #[inline]
    pub fn loop_offset_time(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    BGMExcel::VT_LOOP_OFFSET_TIME,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for BGMExcel<'_> {
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
            >>("path", Self::VT_PATH, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "volume",
                Self::VT_VOLUME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "loop_start_time",
                Self::VT_LOOP_START_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "loop_end_time",
                Self::VT_LOOP_END_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "loop_transtion_time",
                Self::VT_LOOP_TRANSTION_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "loop_offset_time",
                Self::VT_LOOP_OFFSET_TIME,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for BGMExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BGMExcel", 8)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.nation() {
            s.serialize_field("nation", &f)?;
        } else {
            s.skip_field("nation")?;
        }
        if let Some(f) = self.path() {
            s.serialize_field("path", &f)?;
        } else {
            s.skip_field("path")?;
        }
        if let Some(f) = self.volume() {
            s.serialize_field("volume", &f)?;
        } else {
            s.skip_field("volume")?;
        }
        if let Some(f) = self.loop_start_time() {
            s.serialize_field("loop_start_time", &f)?;
        } else {
            s.skip_field("loop_start_time")?;
        }
        if let Some(f) = self.loop_end_time() {
            s.serialize_field("loop_end_time", &f)?;
        } else {
            s.skip_field("loop_end_time")?;
        }
        if let Some(f) = self.loop_transtion_time() {
            s.serialize_field("loop_transtion_time", &f)?;
        } else {
            s.skip_field("loop_transtion_time")?;
        }
        if let Some(f) = self.loop_offset_time() {
            s.serialize_field("loop_offset_time", &f)?;
        } else {
            s.skip_field("loop_offset_time")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for BGMExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("BGMExcel");
        ds.field("id", &self.id());
        ds.field("nation", &self.nation());
        ds.field("path", &self.path());
        ds.field("volume", &self.volume());
        ds.field("loop_start_time", &self.loop_start_time());
        ds.field("loop_end_time", &self.loop_end_time());
        ds.field("loop_transtion_time", &self.loop_transtion_time());
        ds.field("loop_offset_time", &self.loop_offset_time());
        ds.finish()
    }
}
