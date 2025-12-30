extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioCharacterSituationSetExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioCharacterSituationSetExcel<'a> {
    type Inner = ScenarioCharacterSituationSetExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioCharacterSituationSetExcel<'a> {
    pub const VT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_FACE: ::flatbuffers::VOffsetT = 6;
    pub const VT_BEHAVIOR: ::flatbuffers::VOffsetT = 8;
    pub const VT_ACTION: ::flatbuffers::VOffsetT = 10;
    pub const VT_SHAPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_EFFECT: ::flatbuffers::VOffsetT = 14;
    pub const VT_EMOTION: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioCharacterSituationSetExcel::VT_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn face(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterSituationSetExcel::VT_FACE,
                None,
            )
        }
    }
    #[inline]
    pub fn behavior(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterSituationSetExcel::VT_BEHAVIOR,
                None,
            )
        }
    }
    #[inline]
    pub fn action(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterSituationSetExcel::VT_ACTION,
                None,
            )
        }
    }
    #[inline]
    pub fn shape(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterSituationSetExcel::VT_SHAPE,
                None,
            )
        }
    }
    #[inline]
    pub fn effect(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioCharacterSituationSetExcel::VT_EFFECT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn emotion(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioCharacterSituationSetExcel::VT_EMOTION, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioCharacterSituationSetExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("name", Self::VT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("face", Self::VT_FACE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "behavior",
                Self::VT_BEHAVIOR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("action", Self::VT_ACTION, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("shape", Self::VT_SHAPE, false)?
            .visit_field::<u32>("effect", Self::VT_EFFECT, false)?
            .visit_field::<u32>("emotion", Self::VT_EMOTION, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioCharacterSituationSetExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioCharacterSituationSetExcel", 7)?;
        s.serialize_field("name", &self.name())?;
        if let Some(f) = self.face() {
            s.serialize_field("face", &f)?;
        } else {
            s.skip_field("face")?;
        }
        if let Some(f) = self.behavior() {
            s.serialize_field("behavior", &f)?;
        } else {
            s.skip_field("behavior")?;
        }
        if let Some(f) = self.action() {
            s.serialize_field("action", &f)?;
        } else {
            s.skip_field("action")?;
        }
        if let Some(f) = self.shape() {
            s.serialize_field("shape", &f)?;
        } else {
            s.skip_field("shape")?;
        }
        s.serialize_field("effect", &self.effect())?;
        s.serialize_field("emotion", &self.emotion())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioCharacterSituationSetExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioCharacterSituationSetExcel");
        ds.field("name", &self.name());
        ds.field("face", &self.face());
        ds.field("behavior", &self.behavior());
        ds.field("action", &self.action());
        ds.field("shape", &self.shape());
        ds.field("effect", &self.effect());
        ds.field("emotion", &self.emotion());
        ds.finish()
    }
}
