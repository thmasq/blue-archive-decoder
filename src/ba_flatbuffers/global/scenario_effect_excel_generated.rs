extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioEffectExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioEffectExcel<'a> {
    type Inner = ScenarioEffectExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioEffectExcel<'a> {
    pub const VT_EFFECT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME: ::flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn effect_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioEffectExcel::VT_EFFECT_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioEffectExcel::VT_NAME, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioEffectExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "effect_name",
                Self::VT_EFFECT_NAME,
                false,
            )?
            .visit_field::<u32>("name", Self::VT_NAME, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioEffectExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioEffectExcel", 2)?;
        if let Some(f) = self.effect_name() {
            s.serialize_field("effect_name", &f)?;
        } else {
            s.skip_field("effect_name")?;
        }
        s.serialize_field("name", &self.name())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioEffectExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioEffectExcel");
        ds.field("effect_name", &self.effect_name());
        ds.field("name", &self.name());
        ds.finish()
    }
}
