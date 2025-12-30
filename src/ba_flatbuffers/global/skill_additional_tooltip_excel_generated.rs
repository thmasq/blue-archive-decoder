extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct SkillAdditionalTooltipExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for SkillAdditionalTooltipExcel<'a> {
    type Inner = SkillAdditionalTooltipExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> SkillAdditionalTooltipExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ADDITIONAL_SKILL_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_SHOW_SKILL_SLOT: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SkillAdditionalTooltipExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn additional_skill_group_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                SkillAdditionalTooltipExcel::VT_ADDITIONAL_SKILL_GROUP_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn show_skill_slot(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                SkillAdditionalTooltipExcel::VT_SHOW_SKILL_SLOT,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for SkillAdditionalTooltipExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "additional_skill_group_id",
                Self::VT_ADDITIONAL_SKILL_GROUP_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "show_skill_slot",
                Self::VT_SHOW_SKILL_SLOT,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for SkillAdditionalTooltipExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SkillAdditionalTooltipExcel", 3)?;
        s.serialize_field("group_id", &self.group_id())?;
        if let Some(f) = self.additional_skill_group_id() {
            s.serialize_field("additional_skill_group_id", &f)?;
        } else {
            s.skip_field("additional_skill_group_id")?;
        }
        if let Some(f) = self.show_skill_slot() {
            s.serialize_field("show_skill_slot", &f)?;
        } else {
            s.skip_field("show_skill_slot")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for SkillAdditionalTooltipExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("SkillAdditionalTooltipExcel");
        ds.field("group_id", &self.group_id());
        ds.field(
            "additional_skill_group_id",
            &self.additional_skill_group_id(),
        );
        ds.field("show_skill_slot", &self.show_skill_slot());
        ds.finish()
    }
}
