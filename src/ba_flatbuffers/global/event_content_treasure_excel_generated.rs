extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentTreasureExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentTreasureExcel<'a> {
    type Inner = EventContentTreasureExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentTreasureExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TITLE_LOCALIZE: ::flatbuffers::VOffsetT = 6;
    pub const VT_LOOP_ROUND: ::flatbuffers::VOffsetT = 8;
    pub const VT_USE_PREFAB_NAME: ::flatbuffers::VOffsetT = 10;
    pub const VT_TREASURE_BG_IMAGE_PATH: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(EventContentTreasureExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn title_localize(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureExcel::VT_TITLE_LOCALIZE,
                None,
            )
        }
    }
    #[inline]
    pub fn loop_round(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(EventContentTreasureExcel::VT_LOOP_ROUND, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn use_prefab_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureExcel::VT_USE_PREFAB_NAME,
                None,
            )
        }
    }
    #[inline]
    pub fn treasure_bg_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                EventContentTreasureExcel::VT_TREASURE_BG_IMAGE_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentTreasureExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "title_localize",
                Self::VT_TITLE_LOCALIZE,
                false,
            )?
            .visit_field::<i32>("loop_round", Self::VT_LOOP_ROUND, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "use_prefab_name",
                Self::VT_USE_PREFAB_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "treasure_bg_image_path",
                Self::VT_TREASURE_BG_IMAGE_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for EventContentTreasureExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentTreasureExcel", 5)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        if let Some(f) = self.title_localize() {
            s.serialize_field("title_localize", &f)?;
        } else {
            s.skip_field("title_localize")?;
        }
        s.serialize_field("loop_round", &self.loop_round())?;
        if let Some(f) = self.use_prefab_name() {
            s.serialize_field("use_prefab_name", &f)?;
        } else {
            s.skip_field("use_prefab_name")?;
        }
        if let Some(f) = self.treasure_bg_image_path() {
            s.serialize_field("treasure_bg_image_path", &f)?;
        } else {
            s.skip_field("treasure_bg_image_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentTreasureExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentTreasureExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("title_localize", &self.title_localize());
        ds.field("loop_round", &self.loop_round());
        ds.field("use_prefab_name", &self.use_prefab_name());
        ds.field("treasure_bg_image_path", &self.treasure_bg_image_path());
        ds.finish()
    }
}
