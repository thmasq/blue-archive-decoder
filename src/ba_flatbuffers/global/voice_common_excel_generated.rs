extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::VoiceEvent;

#[derive(Copy, Clone, PartialEq)]
pub struct VoiceCommonExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for VoiceCommonExcel<'a> {
    type Inner = VoiceCommonExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> VoiceCommonExcel<'a> {
    pub const VT_VOICE_EVENT: ::flatbuffers::VOffsetT = 4;
    pub const VT_RATE: ::flatbuffers::VOffsetT = 6;
    pub const VT_VOICE_HASH: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn voice_event(&self) -> VoiceEvent {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<VoiceEvent>(VoiceCommonExcel::VT_VOICE_EVENT, Some(VoiceEvent::OnTSA))
                .unwrap()
        }
    }
    #[inline]
    pub fn rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(VoiceCommonExcel::VT_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn voice_hash(&self) -> Option<::flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, u32>>>(
                    VoiceCommonExcel::VT_VOICE_HASH,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for VoiceCommonExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<VoiceEvent>("voice_event", Self::VT_VOICE_EVENT, false)?
            .visit_field::<i64>("rate", Self::VT_RATE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>(
                "voice_hash",
                Self::VT_VOICE_HASH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for VoiceCommonExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VoiceCommonExcel", 3)?;
        s.serialize_field("voice_event", &self.voice_event())?;
        s.serialize_field("rate", &self.rate())?;
        if let Some(f) = self.voice_hash() {
            s.serialize_field("voice_hash", &f)?;
        } else {
            s.skip_field("voice_hash")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for VoiceCommonExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("VoiceCommonExcel");
        ds.field("voice_event", &self.voice_event());
        ds.field("rate", &self.rate());
        ds.field("voice_hash", &self.voice_hash());
        ds.finish()
    }
}
