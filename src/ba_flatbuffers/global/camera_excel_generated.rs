extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct CameraExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CameraExcel<'a> {
    type Inner = CameraExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CameraExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_MIN_DISTANCE: ::flatbuffers::VOffsetT = 6;
    pub const VT_MAX_DISTANCE: ::flatbuffers::VOffsetT = 8;
    pub const VT_ROTATION_X: ::flatbuffers::VOffsetT = 10;
    pub const VT_ROTATION_Y: ::flatbuffers::VOffsetT = 12;
    pub const VT_MOVE_INSTANTLY: ::flatbuffers::VOffsetT = 14;
    pub const VT_MOVE_INSTANTLY_ROTATION_SAVE: ::flatbuffers::VOffsetT = 16;
    pub const VT_LEFT_MARGIN: ::flatbuffers::VOffsetT = 18;
    pub const VT_BOTTOM_MARGIN: ::flatbuffers::VOffsetT = 20;
    pub const VT_IGNORE_ENEMIES: ::flatbuffers::VOffsetT = 22;
    pub const VT_USE_RAIL_POINT_COMPENSATION: ::flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CameraExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn min_distance(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_MIN_DISTANCE, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn max_distance(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_MAX_DISTANCE, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn rotation_x(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_ROTATION_X, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn rotation_y(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_ROTATION_Y, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn move_instantly(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CameraExcel::VT_MOVE_INSTANTLY, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn move_instantly_rotation_save(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CameraExcel::VT_MOVE_INSTANTLY_ROTATION_SAVE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn left_margin(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_LEFT_MARGIN, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn bottom_margin(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(CameraExcel::VT_BOTTOM_MARGIN, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn ignore_enemies(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CameraExcel::VT_IGNORE_ENEMIES, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn use_rail_point_compensation(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CameraExcel::VT_USE_RAIL_POINT_COMPENSATION, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CameraExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<f32>("min_distance", Self::VT_MIN_DISTANCE, false)?
            .visit_field::<f32>("max_distance", Self::VT_MAX_DISTANCE, false)?
            .visit_field::<f32>("rotation_x", Self::VT_ROTATION_X, false)?
            .visit_field::<f32>("rotation_y", Self::VT_ROTATION_Y, false)?
            .visit_field::<bool>("move_instantly", Self::VT_MOVE_INSTANTLY, false)?
            .visit_field::<bool>(
                "move_instantly_rotation_save",
                Self::VT_MOVE_INSTANTLY_ROTATION_SAVE,
                false,
            )?
            .visit_field::<f32>("left_margin", Self::VT_LEFT_MARGIN, false)?
            .visit_field::<f32>("bottom_margin", Self::VT_BOTTOM_MARGIN, false)?
            .visit_field::<bool>("ignore_enemies", Self::VT_IGNORE_ENEMIES, false)?
            .visit_field::<bool>(
                "use_rail_point_compensation",
                Self::VT_USE_RAIL_POINT_COMPENSATION,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for CameraExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CameraExcel", 11)?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("min_distance", &self.min_distance())?;
        s.serialize_field("max_distance", &self.max_distance())?;
        s.serialize_field("rotation_x", &self.rotation_x())?;
        s.serialize_field("rotation_y", &self.rotation_y())?;
        s.serialize_field("move_instantly", &self.move_instantly())?;
        s.serialize_field(
            "move_instantly_rotation_save",
            &self.move_instantly_rotation_save(),
        )?;
        s.serialize_field("left_margin", &self.left_margin())?;
        s.serialize_field("bottom_margin", &self.bottom_margin())?;
        s.serialize_field("ignore_enemies", &self.ignore_enemies())?;
        s.serialize_field(
            "use_rail_point_compensation",
            &self.use_rail_point_compensation(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for CameraExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CameraExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("min_distance", &self.min_distance());
        ds.field("max_distance", &self.max_distance());
        ds.field("rotation_x", &self.rotation_x());
        ds.field("rotation_y", &self.rotation_y());
        ds.field("move_instantly", &self.move_instantly());
        ds.field(
            "move_instantly_rotation_save",
            &self.move_instantly_rotation_save(),
        );
        ds.field("left_margin", &self.left_margin());
        ds.field("bottom_margin", &self.bottom_margin());
        ds.field("ignore_enemies", &self.ignore_enemies());
        ds.field(
            "use_rail_point_compensation",
            &self.use_rail_point_compensation(),
        );
        ds.finish()
    }
}
