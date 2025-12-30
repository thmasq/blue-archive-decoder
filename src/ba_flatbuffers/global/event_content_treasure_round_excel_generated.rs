extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentTreasureRoundExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentTreasureRoundExcel<'a> {
    type Inner = EventContentTreasureRoundExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentTreasureRoundExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TREASURE_ROUND: ::flatbuffers::VOffsetT = 6;
    pub const VT_TREASURE_ROUND_SIZE: ::flatbuffers::VOffsetT = 8;
    pub const VT_CELL_VISUAL_SORT_UNSTRUCTED: ::flatbuffers::VOffsetT = 10;
    pub const VT_CELL_CHECK_GOODS_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_CELL_REWARD_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_REWARD_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_TREASURE_CELL_IMAGE_PATH: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentTreasureRoundExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn treasure_round(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(EventContentTreasureRoundExcel::VT_TREASURE_ROUND, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn treasure_round_size(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    EventContentTreasureRoundExcel::VT_TREASURE_ROUND_SIZE,
                    None,
                )
        }
    }
    #[inline]
    pub fn cell_visual_sort_unstructed(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    EventContentTreasureRoundExcel::VT_CELL_VISUAL_SORT_UNSTRUCTED,
                    Some(false),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn cell_check_goods_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    EventContentTreasureRoundExcel::VT_CELL_CHECK_GOODS_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn cell_reward_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentTreasureRoundExcel::VT_CELL_REWARD_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    EventContentTreasureRoundExcel::VT_REWARD_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn reward_amount(&self) -> Option<::flatbuffers::Vector<'a, i32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i32>>>(
                    EventContentTreasureRoundExcel::VT_REWARD_AMOUNT,
                    None,
                )
        }
    }
    #[inline]
    pub fn treasure_cell_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureRoundExcel::VT_TREASURE_CELL_IMAGE_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentTreasureRoundExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i32>("treasure_round", Self::VT_TREASURE_ROUND, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "treasure_round_size",
                Self::VT_TREASURE_ROUND_SIZE,
                false,
            )?
            .visit_field::<bool>(
                "cell_visual_sort_unstructed",
                Self::VT_CELL_VISUAL_SORT_UNSTRUCTED,
                false,
            )?
            .visit_field::<i64>("cell_check_goods_id", Self::VT_CELL_CHECK_GOODS_ID, false)?
            .visit_field::<i64>("cell_reward_id", Self::VT_CELL_REWARD_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "reward_id",
                Self::VT_REWARD_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i32>>>(
                "reward_amount",
                Self::VT_REWARD_AMOUNT,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "treasure_cell_image_path",
                Self::VT_TREASURE_CELL_IMAGE_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for EventContentTreasureRoundExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentTreasureRoundExcel", 9)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("treasure_round", &self.treasure_round())?;
        if let Some(f) = self.treasure_round_size() {
            s.serialize_field("treasure_round_size", &f)?;
        } else {
            s.skip_field("treasure_round_size")?;
        }
        s.serialize_field(
            "cell_visual_sort_unstructed",
            &self.cell_visual_sort_unstructed(),
        )?;
        s.serialize_field("cell_check_goods_id", &self.cell_check_goods_id())?;
        s.serialize_field("cell_reward_id", &self.cell_reward_id())?;
        if let Some(f) = self.reward_id() {
            s.serialize_field("reward_id", &f)?;
        } else {
            s.skip_field("reward_id")?;
        }
        if let Some(f) = self.reward_amount() {
            s.serialize_field("reward_amount", &f)?;
        } else {
            s.skip_field("reward_amount")?;
        }
        if let Some(f) = self.treasure_cell_image_path() {
            s.serialize_field("treasure_cell_image_path", &f)?;
        } else {
            s.skip_field("treasure_cell_image_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentTreasureRoundExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentTreasureRoundExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("treasure_round", &self.treasure_round());
        ds.field("treasure_round_size", &self.treasure_round_size());
        ds.field(
            "cell_visual_sort_unstructed",
            &self.cell_visual_sort_unstructed(),
        );
        ds.field("cell_check_goods_id", &self.cell_check_goods_id());
        ds.field("cell_reward_id", &self.cell_reward_id());
        ds.field("reward_id", &self.reward_id());
        ds.field("reward_amount", &self.reward_amount());
        ds.field("treasure_cell_image_path", &self.treasure_cell_image_path());
        ds.finish()
    }
}
