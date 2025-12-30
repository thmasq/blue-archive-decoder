extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::StatType;

#[derive(Copy, Clone, PartialEq)]
pub struct MultiFloorRaidStatChangeExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MultiFloorRaidStatChangeExcel<'a> {
    type Inner = MultiFloorRaidStatChangeExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MultiFloorRaidStatChangeExcel<'a> {
    pub const VT_STAT_CHANGE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_STAT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_STAT_ADD: ::flatbuffers::VOffsetT = 8;
    pub const VT_STAT_MULTIPLY: ::flatbuffers::VOffsetT = 10;
    pub const VT_APPLY_CHARACTER_ID: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn stat_change_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidStatChangeExcel::VT_STAT_CHANGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn stat_type(&self) -> Option<::flatbuffers::Vector<'a, StatType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, StatType>>>(
                    MultiFloorRaidStatChangeExcel::VT_STAT_TYPE,
                    None,
                )
        }
    }
    #[inline]
    pub fn stat_add(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MultiFloorRaidStatChangeExcel::VT_STAT_ADD,
                    None,
                )
        }
    }
    #[inline]
    pub fn stat_multiply(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MultiFloorRaidStatChangeExcel::VT_STAT_MULTIPLY,
                    None,
                )
        }
    }
    #[inline]
    pub fn apply_character_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MultiFloorRaidStatChangeExcel::VT_APPLY_CHARACTER_ID,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for MultiFloorRaidStatChangeExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("stat_change_id", Self::VT_STAT_CHANGE_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, StatType>>>(
                "stat_type",
                Self::VT_STAT_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stat_add",
                Self::VT_STAT_ADD,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "stat_multiply",
                Self::VT_STAT_MULTIPLY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "apply_character_id",
                Self::VT_APPLY_CHARACTER_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MultiFloorRaidStatChangeExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MultiFloorRaidStatChangeExcel", 5)?;
        s.serialize_field("stat_change_id", &self.stat_change_id())?;
        if let Some(f) = self.stat_type() {
            s.serialize_field("stat_type", &f)?;
        } else {
            s.skip_field("stat_type")?;
        }
        if let Some(f) = self.stat_add() {
            s.serialize_field("stat_add", &f)?;
        } else {
            s.skip_field("stat_add")?;
        }
        if let Some(f) = self.stat_multiply() {
            s.serialize_field("stat_multiply", &f)?;
        } else {
            s.skip_field("stat_multiply")?;
        }
        if let Some(f) = self.apply_character_id() {
            s.serialize_field("apply_character_id", &f)?;
        } else {
            s.skip_field("apply_character_id")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MultiFloorRaidStatChangeExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MultiFloorRaidStatChangeExcel");
        ds.field("stat_change_id", &self.stat_change_id());
        ds.field("stat_type", &self.stat_type());
        ds.field("stat_add", &self.stat_add());
        ds.field("stat_multiply", &self.stat_multiply());
        ds.field("apply_character_id", &self.apply_character_id());
        ds.finish()
    }
}
