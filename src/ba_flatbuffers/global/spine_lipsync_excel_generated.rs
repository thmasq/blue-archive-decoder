extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct SpineLipsyncExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for SpineLipsyncExcel<'a> {
    type Inner = SpineLipsyncExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> SpineLipsyncExcel<'a> {
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ANIM_JSON: ::flatbuffers::VOffsetT = 6;
    pub const VT_ANIM_JSON_KR: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn voice_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(SpineLipsyncExcel::VT_VOICE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn anim_json(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(SpineLipsyncExcel::VT_ANIM_JSON, None)
        }
    }
    #[inline]
    pub fn anim_json_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                SpineLipsyncExcel::VT_ANIM_JSON_KR,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for SpineLipsyncExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("voice_id", Self::VT_VOICE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "anim_json",
                Self::VT_ANIM_JSON,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "anim_json_kr",
                Self::VT_ANIM_JSON_KR,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for SpineLipsyncExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SpineLipsyncExcel", 3)?;
        s.serialize_field("voice_id", &self.voice_id())?;
        if let Some(f) = self.anim_json() {
            s.serialize_field("anim_json", &f)?;
        } else {
            s.skip_field("anim_json")?;
        }
        if let Some(f) = self.anim_json_kr() {
            s.serialize_field("anim_json_kr", &f)?;
        } else {
            s.skip_field("anim_json_kr")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for SpineLipsyncExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("SpineLipsyncExcel");
        ds.field("voice_id", &self.voice_id());
        ds.field("anim_json", &self.anim_json());
        ds.field("anim_json_kr", &self.anim_json_kr());
        ds.finish()
    }
}
