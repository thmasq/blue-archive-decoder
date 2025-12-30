extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct FormationLocationExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for FormationLocationExcel<'a> {
    type Inner = FormationLocationExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> FormationLocationExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_SLOT_Z: ::flatbuffers::VOffsetT = 8;
    pub const VT_SLOT_X: ::flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FormationLocationExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FormationLocationExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn slot_z(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    FormationLocationExcel::VT_SLOT_Z,
                    None,
                )
        }
    }
    #[inline]
    pub fn slot_x(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    FormationLocationExcel::VT_SLOT_X,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for FormationLocationExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "slot_z",
                Self::VT_SLOT_Z,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "slot_x",
                Self::VT_SLOT_X,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for FormationLocationExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FormationLocationExcel", 4)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("group_id", &self.group_id())?;
        if let Some(f) = self.slot_z() {
            s.serialize_field("slot_z", &f)?;
        } else {
            s.skip_field("slot_z")?;
        }
        if let Some(f) = self.slot_x() {
            s.serialize_field("slot_x", &f)?;
        } else {
            s.skip_field("slot_x")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for FormationLocationExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("FormationLocationExcel");
        ds.field("id", &self.id());
        ds.field("group_id", &self.group_id());
        ds.field("slot_z", &self.slot_z());
        ds.field("slot_x", &self.slot_x());
        ds.finish()
    }
}
