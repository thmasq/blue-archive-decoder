extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct ObstacleStatExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ObstacleStatExcel<'a> {
    type Inner = ObstacleStatExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ObstacleStatExcel<'a> {
    pub const VT_STRING_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_NAME: ::flatbuffers::VOffsetT = 6;
    pub const VT_MAX_HP1: ::flatbuffers::VOffsetT = 8;
    pub const VT_MAX_HP100: ::flatbuffers::VOffsetT = 10;
    pub const VT_BLOCK_RATE: ::flatbuffers::VOffsetT = 12;
    pub const VT_DODGE: ::flatbuffers::VOffsetT = 14;
    pub const VT_CAN_NOT_STAND_RANGE: ::flatbuffers::VOffsetT = 16;
    pub const VT_HIGHLIGHT_FLOATER_HEIGHT: ::flatbuffers::VOffsetT = 18;
    pub const VT_ENHANCE_LIGHT_ARMOR_RATE: ::flatbuffers::VOffsetT = 20;
    pub const VT_ENHANCE_HEAVY_ARMOR_RATE: ::flatbuffers::VOffsetT = 22;
    pub const VT_ENHANCE_UNARMED_RATE: ::flatbuffers::VOffsetT = 24;
    pub const VT_ENHANCE_ELASTIC_ARMOR_RATE: ::flatbuffers::VOffsetT = 26;
    pub const VT_ENHANCE_STRUCTURE_RATE: ::flatbuffers::VOffsetT = 28;
    pub const VT_ENHANCE_NORMAL_ARMOR_RATE: ::flatbuffers::VOffsetT = 30;
    pub const VT_REDUCE_EX_DAMAGED_RATE: ::flatbuffers::VOffsetT = 32;

    #[inline]
    pub fn string_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ObstacleStatExcel::VT_STRING_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ObstacleStatExcel::VT_NAME, None)
        }
    }
    #[inline]
    pub fn max_hp1(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_MAX_HP1, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn max_hp100(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_MAX_HP100, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn block_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_BLOCK_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dodge(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_DODGE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn can_not_stand_range(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_CAN_NOT_STAND_RANGE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn highlight_floater_height(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(ObstacleStatExcel::VT_HIGHLIGHT_FLOATER_HEIGHT, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_light_armor_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_LIGHT_ARMOR_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_heavy_armor_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_HEAVY_ARMOR_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_unarmed_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_UNARMED_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_elastic_armor_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_ELASTIC_ARMOR_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_structure_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_STRUCTURE_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn enhance_normal_armor_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_ENHANCE_NORMAL_ARMOR_RATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reduce_ex_damaged_rate(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ObstacleStatExcel::VT_REDUCE_EX_DAMAGED_RATE, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ObstacleStatExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("string_id", Self::VT_STRING_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
            .visit_field::<i64>("max_hp1", Self::VT_MAX_HP1, false)?
            .visit_field::<i64>("max_hp100", Self::VT_MAX_HP100, false)?
            .visit_field::<i64>("block_rate", Self::VT_BLOCK_RATE, false)?
            .visit_field::<i64>("dodge", Self::VT_DODGE, false)?
            .visit_field::<i64>("can_not_stand_range", Self::VT_CAN_NOT_STAND_RANGE, false)?
            .visit_field::<f32>(
                "highlight_floater_height",
                Self::VT_HIGHLIGHT_FLOATER_HEIGHT,
                false,
            )?
            .visit_field::<i64>(
                "enhance_light_armor_rate",
                Self::VT_ENHANCE_LIGHT_ARMOR_RATE,
                false,
            )?
            .visit_field::<i64>(
                "enhance_heavy_armor_rate",
                Self::VT_ENHANCE_HEAVY_ARMOR_RATE,
                false,
            )?
            .visit_field::<i64>("enhance_unarmed_rate", Self::VT_ENHANCE_UNARMED_RATE, false)?
            .visit_field::<i64>(
                "enhance_elastic_armor_rate",
                Self::VT_ENHANCE_ELASTIC_ARMOR_RATE,
                false,
            )?
            .visit_field::<i64>(
                "enhance_structure_rate",
                Self::VT_ENHANCE_STRUCTURE_RATE,
                false,
            )?
            .visit_field::<i64>(
                "enhance_normal_armor_rate",
                Self::VT_ENHANCE_NORMAL_ARMOR_RATE,
                false,
            )?
            .visit_field::<i64>(
                "reduce_ex_damaged_rate",
                Self::VT_REDUCE_EX_DAMAGED_RATE,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ObstacleStatExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ObstacleStatExcel", 15)?;
        s.serialize_field("string_id", &self.string_id())?;
        if let Some(f) = self.name() {
            s.serialize_field("name", &f)?;
        } else {
            s.skip_field("name")?;
        }
        s.serialize_field("max_hp1", &self.max_hp1())?;
        s.serialize_field("max_hp100", &self.max_hp100())?;
        s.serialize_field("block_rate", &self.block_rate())?;
        s.serialize_field("dodge", &self.dodge())?;
        s.serialize_field("can_not_stand_range", &self.can_not_stand_range())?;
        s.serialize_field("highlight_floater_height", &self.highlight_floater_height())?;
        s.serialize_field("enhance_light_armor_rate", &self.enhance_light_armor_rate())?;
        s.serialize_field("enhance_heavy_armor_rate", &self.enhance_heavy_armor_rate())?;
        s.serialize_field("enhance_unarmed_rate", &self.enhance_unarmed_rate())?;
        s.serialize_field(
            "enhance_elastic_armor_rate",
            &self.enhance_elastic_armor_rate(),
        )?;
        s.serialize_field("enhance_structure_rate", &self.enhance_structure_rate())?;
        s.serialize_field(
            "enhance_normal_armor_rate",
            &self.enhance_normal_armor_rate(),
        )?;
        s.serialize_field("reduce_ex_damaged_rate", &self.reduce_ex_damaged_rate())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ObstacleStatExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ObstacleStatExcel");
        ds.field("string_id", &self.string_id());
        ds.field("name", &self.name());
        ds.field("max_hp1", &self.max_hp1());
        ds.field("max_hp100", &self.max_hp100());
        ds.field("block_rate", &self.block_rate());
        ds.field("dodge", &self.dodge());
        ds.field("can_not_stand_range", &self.can_not_stand_range());
        ds.field("highlight_floater_height", &self.highlight_floater_height());
        ds.field("enhance_light_armor_rate", &self.enhance_light_armor_rate());
        ds.field("enhance_heavy_armor_rate", &self.enhance_heavy_armor_rate());
        ds.field("enhance_unarmed_rate", &self.enhance_unarmed_rate());
        ds.field(
            "enhance_elastic_armor_rate",
            &self.enhance_elastic_armor_rate(),
        );
        ds.field("enhance_structure_rate", &self.enhance_structure_rate());
        ds.field(
            "enhance_normal_armor_rate",
            &self.enhance_normal_armor_rate(),
        );
        ds.field("reduce_ex_damaged_rate", &self.reduce_ex_damaged_rate());
        ds.finish()
    }
}
