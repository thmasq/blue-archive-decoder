extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ProductionStep, ScenarioCharacterShapes};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioCharacterNameExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioCharacterNameExcel<'a> {
    type Inner = ScenarioCharacterNameExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioCharacterNameExcel<'a> {
    pub const VT_CHARACTER_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_PRODUCTION_STEP: ::flatbuffers::VOffsetT = 6;
    pub const VT_NAME_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_NICKNAME_KR: ::flatbuffers::VOffsetT = 10;
    pub const VT_NAME_JP: ::flatbuffers::VOffsetT = 12;
    pub const VT_NICKNAME_JP: ::flatbuffers::VOffsetT = 14;
    pub const VT_NAME_TH: ::flatbuffers::VOffsetT = 16;
    pub const VT_NICKNAME_TH: ::flatbuffers::VOffsetT = 18;
    pub const VT_NAME_TW: ::flatbuffers::VOffsetT = 20;
    pub const VT_NICKNAME_TW: ::flatbuffers::VOffsetT = 22;
    pub const VT_NAME_EN: ::flatbuffers::VOffsetT = 24;
    pub const VT_NICKNAME_EN: ::flatbuffers::VOffsetT = 26;
    pub const VT_SHAPE: ::flatbuffers::VOffsetT = 28;
    pub const VT_SPINE_PREFAB_NAME: ::flatbuffers::VOffsetT = 30;
    pub const VT_SMALL_PORTRAIT: ::flatbuffers::VOffsetT = 32;

    #[inline]
    pub fn character_name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioCharacterNameExcel::VT_CHARACTER_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn production_step(&self) -> ProductionStep {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ProductionStep>(
                    ScenarioCharacterNameExcel::VT_PRODUCTION_STEP,
                    Some(ProductionStep::ToDo),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn name_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NAME_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn nickname_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NICKNAME_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn name_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NAME_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn nickname_jp(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NICKNAME_JP,
                None,
            )
        }
    }
    #[inline]
    pub fn name_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NAME_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn nickname_th(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NICKNAME_TH,
                None,
            )
        }
    }
    #[inline]
    pub fn name_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NAME_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn nickname_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NICKNAME_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn name_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NAME_EN,
                None,
            )
        }
    }
    #[inline]
    pub fn nickname_en(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_NICKNAME_EN,
                None,
            )
        }
    }
    #[inline]
    pub fn shape(&self) -> ScenarioCharacterShapes {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioCharacterShapes>(
                    ScenarioCharacterNameExcel::VT_SHAPE,
                    Some(ScenarioCharacterShapes::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn spine_prefab_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_SPINE_PREFAB_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn small_portrait(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioCharacterNameExcel::VT_SMALL_PORTRAIT,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioCharacterNameExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("character_name", Self::VT_CHARACTER_NAME, false)?
            .visit_field::<ProductionStep>("production_step", Self::VT_PRODUCTION_STEP, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_kr",
                Self::VT_NAME_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "nickname_kr",
                Self::VT_NICKNAME_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_jp",
                Self::VT_NAME_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "nickname_jp",
                Self::VT_NICKNAME_JP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_th",
                Self::VT_NAME_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "nickname_th",
                Self::VT_NICKNAME_TH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_tw",
                Self::VT_NAME_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "nickname_tw",
                Self::VT_NICKNAME_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "name_en",
                Self::VT_NAME_EN,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "nickname_en",
                Self::VT_NICKNAME_EN,
                false,
            )?
            .visit_field::<ScenarioCharacterShapes>("shape", Self::VT_SHAPE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "spine_prefab_name",
                Self::VT_SPINE_PREFAB_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "small_portrait",
                Self::VT_SMALL_PORTRAIT,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioCharacterNameExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioCharacterNameExcel", 15)?;
        s.serialize_field("character_name", &self.character_name())?;
        s.serialize_field("production_step", &self.production_step())?;
        if let Some(f) = self.name_kr() {
            s.serialize_field("name_kr", &f)?;
        } else {
            s.skip_field("name_kr")?;
        }
        if let Some(f) = self.nickname_kr() {
            s.serialize_field("nickname_kr", &f)?;
        } else {
            s.skip_field("nickname_kr")?;
        }
        if let Some(f) = self.name_jp() {
            s.serialize_field("name_jp", &f)?;
        } else {
            s.skip_field("name_jp")?;
        }
        if let Some(f) = self.nickname_jp() {
            s.serialize_field("nickname_jp", &f)?;
        } else {
            s.skip_field("nickname_jp")?;
        }
        if let Some(f) = self.name_th() {
            s.serialize_field("name_th", &f)?;
        } else {
            s.skip_field("name_th")?;
        }
        if let Some(f) = self.nickname_th() {
            s.serialize_field("nickname_th", &f)?;
        } else {
            s.skip_field("nickname_th")?;
        }
        if let Some(f) = self.name_tw() {
            s.serialize_field("name_tw", &f)?;
        } else {
            s.skip_field("name_tw")?;
        }
        if let Some(f) = self.nickname_tw() {
            s.serialize_field("nickname_tw", &f)?;
        } else {
            s.skip_field("nickname_tw")?;
        }
        if let Some(f) = self.name_en() {
            s.serialize_field("name_en", &f)?;
        } else {
            s.skip_field("name_en")?;
        }
        if let Some(f) = self.nickname_en() {
            s.serialize_field("nickname_en", &f)?;
        } else {
            s.skip_field("nickname_en")?;
        }
        s.serialize_field("shape", &self.shape())?;
        if let Some(f) = self.spine_prefab_name() {
            s.serialize_field("spine_prefab_name", &f)?;
        } else {
            s.skip_field("spine_prefab_name")?;
        }
        if let Some(f) = self.small_portrait() {
            s.serialize_field("small_portrait", &f)?;
        } else {
            s.skip_field("small_portrait")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioCharacterNameExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioCharacterNameExcel");
        ds.field("character_name", &self.character_name());
        ds.field("production_step", &self.production_step());
        ds.field("name_kr", &self.name_kr());
        ds.field("nickname_kr", &self.nickname_kr());
        ds.field("name_jp", &self.name_jp());
        ds.field("nickname_jp", &self.nickname_jp());
        ds.field("name_th", &self.name_th());
        ds.field("nickname_th", &self.nickname_th());
        ds.field("name_tw", &self.name_tw());
        ds.field("nickname_tw", &self.nickname_tw());
        ds.field("name_en", &self.name_en());
        ds.field("nickname_en", &self.nickname_en());
        ds.field("shape", &self.shape());
        ds.field("spine_prefab_name", &self.spine_prefab_name());
        ds.field("small_portrait", &self.small_portrait());
        ds.finish()
    }
}
