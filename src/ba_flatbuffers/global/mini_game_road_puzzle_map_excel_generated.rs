extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameRoadPuzzleMapExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameRoadPuzzleMapExcel<'a> {
    type Inner = MiniGameRoadPuzzleMapExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameRoadPuzzleMapExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_MAP_GROUP_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_MAP: ::flatbuffers::VOffsetT = 10;
    pub const VT_MAP_BG: ::flatbuffers::VOffsetT = 12;
    pub const VT_BGM_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_AVAILABLE_RAIL_TILE: ::flatbuffers::VOffsetT = 16;
    pub const VT_AVAILABLE_RAIL_TILE_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_ORIGINAL_TILE_COUNT: ::flatbuffers::VOffsetT = 20;
    pub const VT_TRAIN_SPEED: ::flatbuffers::VOffsetT = 22;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleMapExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleMapExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn map_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleMapExcel::VT_MAP_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn map(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameRoadPuzzleMapExcel::VT_MAP,
                None,
            )
        }
    }
    #[inline]
    pub fn map_bg(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameRoadPuzzleMapExcel::VT_MAP_BG,
                None,
            )
        }
    }
    #[inline]
    pub fn bgm_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleMapExcel::VT_BGM_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn available_rail_tile(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameRoadPuzzleMapExcel::VT_AVAILABLE_RAIL_TILE,
                    None,
                )
        }
    }
    #[inline]
    pub fn available_rail_tile_amount(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameRoadPuzzleMapExcel::VT_AVAILABLE_RAIL_TILE_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn original_tile_count(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    MiniGameRoadPuzzleMapExcel::VT_ORIGINAL_TILE_COUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn train_speed(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(MiniGameRoadPuzzleMapExcel::VT_TRAIN_SPEED, Some(0.0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameRoadPuzzleMapExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<i64>("map_group_id", Self::VT_MAP_GROUP_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("map", Self::VT_MAP, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("map_bg", Self::VT_MAP_BG, false)?
            .visit_field::<i64>("bgm_id", Self::VT_BGM_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "available_rail_tile",
                Self::VT_AVAILABLE_RAIL_TILE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "available_rail_tile_amount",
                Self::VT_AVAILABLE_RAIL_TILE_AMOUNT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "original_tile_count",
                Self::VT_ORIGINAL_TILE_COUNT,
                false,
            )?
            .visit_field::<f32>("train_speed", Self::VT_TRAIN_SPEED, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameRoadPuzzleMapExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameRoadPuzzleMapExcel", 10)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("map_group_id", &self.map_group_id())?;
        if let Some(f) = self.map() {
            s.serialize_field("map", &f)?;
        } else {
            s.skip_field("map")?;
        }
        if let Some(f) = self.map_bg() {
            s.serialize_field("map_bg", &f)?;
        } else {
            s.skip_field("map_bg")?;
        }
        s.serialize_field("bgm_id", &self.bgm_id())?;
        if let Some(f) = self.available_rail_tile() {
            s.serialize_field("available_rail_tile", &f)?;
        } else {
            s.skip_field("available_rail_tile")?;
        }
        if let Some(f) = self.available_rail_tile_amount() {
            s.serialize_field("available_rail_tile_amount", &f)?;
        } else {
            s.skip_field("available_rail_tile_amount")?;
        }
        if let Some(f) = self.original_tile_count() {
            s.serialize_field("original_tile_count", &f)?;
        } else {
            s.skip_field("original_tile_count")?;
        }
        s.serialize_field("train_speed", &self.train_speed())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameRoadPuzzleMapExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameRoadPuzzleMapExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("unique_id", &self.unique_id());
        ds.field("map_group_id", &self.map_group_id());
        ds.field("map", &self.map());
        ds.field("map_bg", &self.map_bg());
        ds.field("bgm_id", &self.bgm_id());
        ds.field("available_rail_tile", &self.available_rail_tile());
        ds.field(
            "available_rail_tile_amount",
            &self.available_rail_tile_amount(),
        );
        ds.field("original_tile_count", &self.original_tile_count());
        ds.field("train_speed", &self.train_speed());
        ds.finish()
    }
}
