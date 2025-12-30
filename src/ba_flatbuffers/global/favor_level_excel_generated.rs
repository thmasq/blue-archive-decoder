extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct FavorLevelExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for FavorLevelExcel<'a> {
    type Inner = FavorLevelExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> FavorLevelExcel<'a> {
    pub const VT_LEVEL: ::flatbuffers::VOffsetT = 4;
    pub const VT_EXP_TYPE: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FavorLevelExcel::VT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn exp_type(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    FavorLevelExcel::VT_EXP_TYPE,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for FavorLevelExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("level", Self::VT_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "exp_type",
                Self::VT_EXP_TYPE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for FavorLevelExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FavorLevelExcel", 2)?;
        s.serialize_field("level", &self.level())?;
        if let Some(f) = self.exp_type() {
            s.serialize_field("exp_type", &f)?;
        } else {
            s.skip_field("exp_type")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for FavorLevelExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("FavorLevelExcel");
        ds.field("level", &self.level());
        ds.field("exp_type", &self.exp_type());
        ds.finish()
    }
}
