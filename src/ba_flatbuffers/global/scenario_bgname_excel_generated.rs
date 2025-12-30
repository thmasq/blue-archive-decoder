extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ProductionStep, ScenarioBGType};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioBGNameExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioBGNameExcel<'a> {
    type Inner = ScenarioBGNameExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioBGNameExcel<'a> {
    pub const VT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_PRODUCTION_STEP: ::flatbuffers::VOffsetT = 6;
    pub const VT_BG_FILE_NAME: ::flatbuffers::VOffsetT = 8;
    pub const VT_BG_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_ANIMATION_ROOT: ::flatbuffers::VOffsetT = 12;
    pub const VT_ANIMATION_NAME: ::flatbuffers::VOffsetT = 14;
    pub const VT_SPINE_SCALE: ::flatbuffers::VOffsetT = 16;
    pub const VT_SPINE_LOCAL_POS_X: ::flatbuffers::VOffsetT = 18;
    pub const VT_SPINE_LOCAL_POS_Y: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGNameExcel::VT_NAME, Some(0))
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
                    ScenarioBGNameExcel::VT_PRODUCTION_STEP,
                    Some(ProductionStep::ToDo),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn bg_file_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioBGNameExcel::VT_BG_FILE_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn bg_type(&self) -> ScenarioBGType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioBGType>(ScenarioBGNameExcel::VT_BG_TYPE, Some(ScenarioBGType::None))
                .unwrap()
        }
    }
    #[inline]
    pub fn animation_root(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioBGNameExcel::VT_ANIMATION_ROOT,
                None,
            )
        }
    }
    #[inline]
    pub fn animation_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioBGNameExcel::VT_ANIMATION_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn spine_scale(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(ScenarioBGNameExcel::VT_SPINE_SCALE, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn spine_local_pos_x(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioBGNameExcel::VT_SPINE_LOCAL_POS_X, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn spine_local_pos_y(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(ScenarioBGNameExcel::VT_SPINE_LOCAL_POS_Y, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioBGNameExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("name", Self::VT_NAME, false)?
            .visit_field::<ProductionStep>("production_step", Self::VT_PRODUCTION_STEP, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "bg_file_name",
                Self::VT_BG_FILE_NAME,
                false,
            )?
            .visit_field::<ScenarioBGType>("bg_type", Self::VT_BG_TYPE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "animation_root",
                Self::VT_ANIMATION_ROOT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "animation_name",
                Self::VT_ANIMATION_NAME,
                false,
            )?
            .visit_field::<f32>("spine_scale", Self::VT_SPINE_SCALE, false)?
            .visit_field::<i32>("spine_local_pos_x", Self::VT_SPINE_LOCAL_POS_X, false)?
            .visit_field::<i32>("spine_local_pos_y", Self::VT_SPINE_LOCAL_POS_Y, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioBGNameExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioBGNameExcel", 9)?;
        s.serialize_field("name", &self.name())?;
        s.serialize_field("production_step", &self.production_step())?;
        if let Some(f) = self.bg_file_name() {
            s.serialize_field("bg_file_name", &f)?;
        } else {
            s.skip_field("bg_file_name")?;
        }
        s.serialize_field("bg_type", &self.bg_type())?;
        if let Some(f) = self.animation_root() {
            s.serialize_field("animation_root", &f)?;
        } else {
            s.skip_field("animation_root")?;
        }
        if let Some(f) = self.animation_name() {
            s.serialize_field("animation_name", &f)?;
        } else {
            s.skip_field("animation_name")?;
        }
        s.serialize_field("spine_scale", &self.spine_scale())?;
        s.serialize_field("spine_local_pos_x", &self.spine_local_pos_x())?;
        s.serialize_field("spine_local_pos_y", &self.spine_local_pos_y())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioBGNameExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioBGNameExcel");
        ds.field("name", &self.name());
        ds.field("production_step", &self.production_step());
        ds.field("bg_file_name", &self.bg_file_name());
        ds.field("bg_type", &self.bg_type());
        ds.field("animation_root", &self.animation_root());
        ds.field("animation_name", &self.animation_name());
        ds.field("spine_scale", &self.spine_scale());
        ds.field("spine_local_pos_x", &self.spine_local_pos_x());
        ds.field("spine_local_pos_y", &self.spine_local_pos_y());
        ds.finish()
    }
}
