extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::Nation;

#[derive(Copy, Clone, PartialEq)]
pub struct VoiceExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for VoiceExcel<'a> {
    type Inner = VoiceExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> VoiceExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_NATION: ::flatbuffers::VOffsetT = 8;
    pub const VT_PATH: ::flatbuffers::VOffsetT = 10;
    pub const VT_VOLUME: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(VoiceExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u32>(VoiceExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn nation(&self) -> Option<::flatbuffers::Vector<'a, Nation>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Nation>>>(
                    VoiceExcel::VT_NATION,
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
            >>(VoiceExcel::VT_PATH, None)
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
                    VoiceExcel::VT_VOLUME,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for VoiceExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<u32>("id", Self::VT_ID, false)?
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
            .finish();
        Ok(())
    }
}

impl Serialize for VoiceExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VoiceExcel", 5)?;
        s.serialize_field("unique_id", &self.unique_id())?;
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
        s.end()
    }
}

impl ::core::fmt::Debug for VoiceExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("VoiceExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("id", &self.id());
        ds.field("nation", &self.nation());
        ds.field("path", &self.path());
        ds.field("volume", &self.volume());
        ds.finish()
    }
}
