extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct SoundUIExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for SoundUIExcel<'a> {
    type Inner = SoundUIExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> SoundUIExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SOUND_UNIQUE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_PATH: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(SoundUIExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn sound_unique_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(SoundUIExcel::VT_SOUND_UNIQUE_ID, None)
        }
    }
    #[inline]
    pub fn path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(SoundUIExcel::VT_PATH, None)
        }
    }
}

impl ::flatbuffers::Verifiable for SoundUIExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sound_unique_id",
                Self::VT_SOUND_UNIQUE_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("path", Self::VT_PATH, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for SoundUIExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SoundUIExcel", 3)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.sound_unique_id() {
            s.serialize_field("sound_unique_id", &f)?;
        } else {
            s.skip_field("sound_unique_id")?;
        }
        if let Some(f) = self.path() {
            s.serialize_field("path", &f)?;
        } else {
            s.skip_field("path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for SoundUIExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("SoundUIExcel");
        ds.field("id", &self.id());
        ds.field("sound_unique_id", &self.sound_unique_id());
        ds.field("path", &self.path());
        ds.finish()
    }
}
