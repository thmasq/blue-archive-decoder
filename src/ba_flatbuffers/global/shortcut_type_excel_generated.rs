extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ShortcutContentType;

#[derive(Copy, Clone, PartialEq)]
pub struct ShortcutTypeExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ShortcutTypeExcel<'a> {
    type Inner = ShortcutTypeExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ShortcutTypeExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_IS_ASCENDING: ::flatbuffers::VOffsetT = 6;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ShortcutTypeExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_ascending(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ShortcutTypeExcel::VT_IS_ASCENDING, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn content_type(&self) -> Option<::flatbuffers::Vector<'a, ShortcutContentType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, ShortcutContentType>>>(ShortcutTypeExcel::VT_CONTENT_TYPE, None)
        }
    }
}

impl ::flatbuffers::Verifiable for ShortcutTypeExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<bool>("is_ascending", Self::VT_IS_ASCENDING, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ShortcutContentType>>>("content_type", Self::VT_CONTENT_TYPE, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for ShortcutTypeExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ShortcutTypeExcel", 3)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("is_ascending", &self.is_ascending())?;
        if let Some(f) = self.content_type() {
            s.serialize_field("content_type", &f)?;
        } else {
            s.skip_field("content_type")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ShortcutTypeExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ShortcutTypeExcel");
        ds.field("id", &self.id());
        ds.field("is_ascending", &self.is_ascending());
        ds.field("content_type", &self.content_type());
        ds.finish()
    }
}
