extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameRoadPuzzleInfoExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameRoadPuzzleInfoExcel<'a> {
    type Inner = MiniGameRoadPuzzleInfoExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameRoadPuzzleInfoExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EVENT_USE_COST_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_EVENT_USE_COST_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_COST_GOODS_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_RAIL_SET_REWARD_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_INSTANT_CLEAR_ROUND: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleInfoExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_use_cost_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameRoadPuzzleInfoExcel::VT_EVENT_USE_COST_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn event_use_cost_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleInfoExcel::VT_EVENT_USE_COST_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn cost_goods_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleInfoExcel::VT_COST_GOODS_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn rail_set_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleInfoExcel::VT_RAIL_SET_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn instant_clear_round(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(MiniGameRoadPuzzleInfoExcel::VT_INSTANT_CLEAR_ROUND, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameRoadPuzzleInfoExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<ParcelType>("event_use_cost_type", Self::VT_EVENT_USE_COST_TYPE, false)?
            .visit_field::<i64>("event_use_cost_id", Self::VT_EVENT_USE_COST_ID, false)?
            .visit_field::<i64>("cost_goods_id", Self::VT_COST_GOODS_ID, false)?
            .visit_field::<i64>("rail_set_reward_id", Self::VT_RAIL_SET_REWARD_ID, false)?
            .visit_field::<i32>("instant_clear_round", Self::VT_INSTANT_CLEAR_ROUND, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameRoadPuzzleInfoExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameRoadPuzzleInfoExcel", 6)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("event_use_cost_type", &self.event_use_cost_type())?;
        s.serialize_field("event_use_cost_id", &self.event_use_cost_id())?;
        s.serialize_field("cost_goods_id", &self.cost_goods_id())?;
        s.serialize_field("rail_set_reward_id", &self.rail_set_reward_id())?;
        s.serialize_field("instant_clear_round", &self.instant_clear_round())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameRoadPuzzleInfoExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameRoadPuzzleInfoExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("event_use_cost_type", &self.event_use_cost_type());
        ds.field("event_use_cost_id", &self.event_use_cost_id());
        ds.field("cost_goods_id", &self.cost_goods_id());
        ds.field("rail_set_reward_id", &self.rail_set_reward_id());
        ds.field("instant_clear_round", &self.instant_clear_round());
        ds.finish()
    }
}
