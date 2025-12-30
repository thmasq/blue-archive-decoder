extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::EchelonType;
#[derive(Copy, Clone, PartialEq)]

pub struct AssistEchelonTypeConvertExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for AssistEchelonTypeConvertExcel<'a> {
    type Inner = AssistEchelonTypeConvertExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> AssistEchelonTypeConvertExcel<'a> {
    pub const VT_CONTENTS: ::flatbuffers::VOffsetT = 4;
    pub const VT_CONVERT_TO: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn contents(&self) -> EchelonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EchelonType>(
                    AssistEchelonTypeConvertExcel::VT_CONTENTS,
                    Some(EchelonType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn convert_to(&self) -> EchelonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EchelonType>(
                    AssistEchelonTypeConvertExcel::VT_CONVERT_TO,
                    Some(EchelonType::None),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for AssistEchelonTypeConvertExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<EchelonType>("contents", Self::VT_CONTENTS, false)?
            .visit_field::<EchelonType>("convert_to", Self::VT_CONVERT_TO, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for AssistEchelonTypeConvertExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AssistEchelonTypeConvertExcel", 2)?;
        s.serialize_field("contents", &self.contents())?;
        s.serialize_field("convert_to", &self.convert_to())?;
        s.end()
    }
}

impl ::core::fmt::Debug for AssistEchelonTypeConvertExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("AssistEchelonTypeConvertExcel");
        ds.field("contents", &self.contents());
        ds.field("convert_to", &self.convert_to());
        ds.finish()
    }
}
