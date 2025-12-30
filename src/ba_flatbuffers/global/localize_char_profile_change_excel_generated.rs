extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct LocalizeCharProfileChangeExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for LocalizeCharProfileChangeExcel<'a> {
    type Inner = LocalizeCharProfileChangeExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> LocalizeCharProfileChangeExcel<'a> {
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_SCENARIO_MODE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_CHANGE_CHARACTER_ID: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(LocalizeCharProfileChangeExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(LocalizeCharProfileChangeExcel::VT_SCENARIO_MODE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn change_character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    LocalizeCharProfileChangeExcel::VT_CHANGE_CHARACTER_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for LocalizeCharProfileChangeExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .visit_field::<i64>("scenario_mode_id", Self::VT_SCENARIO_MODE_ID, false)?
            .visit_field::<i64>("change_character_id", Self::VT_CHANGE_CHARACTER_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for LocalizeCharProfileChangeExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LocalizeCharProfileChangeExcel", 3)?;
        s.serialize_field("character_id", &self.character_id())?;
        s.serialize_field("scenario_mode_id", &self.scenario_mode_id())?;
        s.serialize_field("change_character_id", &self.change_character_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for LocalizeCharProfileChangeExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("LocalizeCharProfileChangeExcel");
        ds.field("character_id", &self.character_id());
        ds.field("scenario_mode_id", &self.scenario_mode_id());
        ds.field("change_character_id", &self.change_character_id());
        ds.finish()
    }
}
