extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct StatLevelInterpolationExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for StatLevelInterpolationExcel<'a> {
    type Inner = StatLevelInterpolationExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> StatLevelInterpolationExcel<'a> {
    pub const VT_LEVEL: ::flatbuffers::VOffsetT = 4;
    pub const VT_STAT_TYPE_INDEX: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(StatLevelInterpolationExcel::VT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stat_type_index(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    StatLevelInterpolationExcel::VT_STAT_TYPE_INDEX,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for StatLevelInterpolationExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("level", Self::VT_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stat_type_index",
                Self::VT_STAT_TYPE_INDEX,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for StatLevelInterpolationExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StatLevelInterpolationExcel", 2)?;
        s.serialize_field("level", &self.level())?;
        if let Some(f) = self.stat_type_index() {
            s.serialize_field("stat_type_index", &f)?;
        } else {
            s.skip_field("stat_type_index")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for StatLevelInterpolationExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("StatLevelInterpolationExcel");
        ds.field("level", &self.level());
        ds.field("stat_type_index", &self.stat_type_index());
        ds.finish()
    }
}
