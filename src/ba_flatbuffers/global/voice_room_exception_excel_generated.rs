extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::CVPrintType;

#[derive(Copy, Clone, PartialEq)]
pub struct VoiceRoomExceptionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for VoiceRoomExceptionExcel<'a> {
    type Inner = VoiceRoomExceptionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> VoiceRoomExceptionExcel<'a> {
    pub const VT_COSTUME_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LINKED_CHARACTER_VOICE_PRINT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_LINKED_COSTUME_UNIQUE_ID: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn costume_unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(VoiceRoomExceptionExcel::VT_COSTUME_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn linked_character_voice_print_type(&self) -> CVPrintType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CVPrintType>(
                    VoiceRoomExceptionExcel::VT_LINKED_CHARACTER_VOICE_PRINT_TYPE,
                    Some(CVPrintType::CharacterOverwrite),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn linked_costume_unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    VoiceRoomExceptionExcel::VT_LINKED_COSTUME_UNIQUE_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for VoiceRoomExceptionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("costume_unique_id", Self::VT_COSTUME_UNIQUE_ID, false)?
            .visit_field::<CVPrintType>(
                "linked_character_voice_print_type",
                Self::VT_LINKED_CHARACTER_VOICE_PRINT_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "linked_costume_unique_id",
                Self::VT_LINKED_COSTUME_UNIQUE_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for VoiceRoomExceptionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VoiceRoomExceptionExcel", 3)?;
        s.serialize_field("costume_unique_id", &self.costume_unique_id())?;
        s.serialize_field(
            "linked_character_voice_print_type",
            &self.linked_character_voice_print_type(),
        )?;
        s.serialize_field("linked_costume_unique_id", &self.linked_costume_unique_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for VoiceRoomExceptionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("VoiceRoomExceptionExcel");
        ds.field("costume_unique_id", &self.costume_unique_id());
        ds.field(
            "linked_character_voice_print_type",
            &self.linked_character_voice_print_type(),
        );
        ds.field("linked_costume_unique_id", &self.linked_costume_unique_id());
        ds.finish()
    }
}
