extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct VoiceLogicEffectExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for VoiceLogicEffectExcel<'a> {
    type Inner = VoiceLogicEffectExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> VoiceLogicEffectExcel<'a> {
    pub const VT_LOGIC_EFFECT_NAME_HASH: ::flatbuffers::VOffsetT = 4;
    pub const VT_SELF_: ::flatbuffers::VOffsetT = 6;
    pub const VT_PRIORITY: ::flatbuffers::VOffsetT = 8;
    pub const VT_VOICE_HASH: ::flatbuffers::VOffsetT = 10;
    pub const VT_VOICE_ID: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn logic_effect_name_hash(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(VoiceLogicEffectExcel::VT_LOGIC_EFFECT_NAME_HASH, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn self_(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(VoiceLogicEffectExcel::VT_SELF_, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn priority(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(VoiceLogicEffectExcel::VT_PRIORITY, Some(0))
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
                    VoiceLogicEffectExcel::VT_VOICE_HASH,
                    None,
                )
        }
    }
    #[inline]
    pub fn voice_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(VoiceLogicEffectExcel::VT_VOICE_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for VoiceLogicEffectExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>(
                "logic_effect_name_hash",
                Self::VT_LOGIC_EFFECT_NAME_HASH,
                false,
            )?
            .visit_field::<bool>("self_", Self::VT_SELF_, false)?
            .visit_field::<i32>("priority", Self::VT_PRIORITY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>(
                "voice_hash",
                Self::VT_VOICE_HASH,
                false,
            )?
            .visit_field::<u32>("voice_id", Self::VT_VOICE_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for VoiceLogicEffectExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VoiceLogicEffectExcel", 5)?;
        s.serialize_field("logic_effect_name_hash", &self.logic_effect_name_hash())?;
        s.serialize_field("self_", &self.self_())?;
        s.serialize_field("priority", &self.priority())?;
        if let Some(f) = self.voice_hash() {
            s.serialize_field("voice_hash", &f)?;
        } else {
            s.skip_field("voice_hash")?;
        }
        s.serialize_field("voice_id", &self.voice_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for VoiceLogicEffectExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("VoiceLogicEffectExcel");
        ds.field("logic_effect_name_hash", &self.logic_effect_name_hash());
        ds.field("self_", &self.self_());
        ds.field("priority", &self.priority());
        ds.field("voice_hash", &self.voice_hash());
        ds.field("voice_id", &self.voice_id());
        ds.finish()
    }
}
