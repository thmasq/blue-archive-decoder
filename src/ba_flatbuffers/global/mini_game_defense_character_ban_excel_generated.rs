extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDefenseCharacterBanExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDefenseCharacterBanExcel<'a> {
    type Inner = MiniGameDefenseCharacterBanExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDefenseCharacterBanExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseCharacterBanExcel::VT_EVENT_CONTENT_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseCharacterBanExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDefenseCharacterBanExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDefenseCharacterBanExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDefenseCharacterBanExcel", 2)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("character_id", &self.character_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDefenseCharacterBanExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDefenseCharacterBanExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("character_id", &self.character_id());
        ds.finish()
    }
}
