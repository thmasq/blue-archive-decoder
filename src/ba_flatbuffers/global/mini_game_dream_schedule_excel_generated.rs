extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamScheduleExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamScheduleExcel<'a> {
    type Inner = MiniGameDreamScheduleExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamScheduleExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DREAM_MAKER_SCHEDULE_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_ICON_PATH: ::flatbuffers::VOffsetT = 12;
    pub const VT_LOADING_RESOURCE01: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOADING_RESOURCE02: ::flatbuffers::VOffsetT = 16;
    pub const VT_ANIMATION_NAME: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamScheduleExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dream_maker_schedule_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDreamScheduleExcel::VT_DREAM_MAKER_SCHEDULE_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn display_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamScheduleExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MiniGameDreamScheduleExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn icon_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamScheduleExcel::VT_ICON_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn loading_resource01(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamScheduleExcel::VT_LOADING_RESOURCE01,
                None,
            )
        }
    }
    #[inline]
    pub fn loading_resource02(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MiniGameDreamScheduleExcel::VT_LOADING_RESOURCE02,
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
                MiniGameDreamScheduleExcel::VT_ANIMATION_NAME,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamScheduleExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>(
                "dream_maker_schedule_group_id",
                Self::VT_DREAM_MAKER_SCHEDULE_GROUP_ID,
                false,
            )?
            .visit_field::<i64>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "icon_path",
                Self::VT_ICON_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "loading_resource01",
                Self::VT_LOADING_RESOURCE01,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "loading_resource02",
                Self::VT_LOADING_RESOURCE02,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "animation_name",
                Self::VT_ANIMATION_NAME,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamScheduleExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamScheduleExcel", 8)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field(
            "dream_maker_schedule_group_id",
            &self.dream_maker_schedule_group_id(),
        )?;
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.icon_path() {
            s.serialize_field("icon_path", &f)?;
        } else {
            s.skip_field("icon_path")?;
        }
        if let Some(f) = self.loading_resource01() {
            s.serialize_field("loading_resource01", &f)?;
        } else {
            s.skip_field("loading_resource01")?;
        }
        if let Some(f) = self.loading_resource02() {
            s.serialize_field("loading_resource02", &f)?;
        } else {
            s.skip_field("loading_resource02")?;
        }
        if let Some(f) = self.animation_name() {
            s.serialize_field("animation_name", &f)?;
        } else {
            s.skip_field("animation_name")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamScheduleExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamScheduleExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field(
            "dream_maker_schedule_group_id",
            &self.dream_maker_schedule_group_id(),
        );
        ds.field("display_order", &self.display_order());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("icon_path", &self.icon_path());
        ds.field("loading_resource01", &self.loading_resource01());
        ds.field("loading_resource02", &self.loading_resource02());
        ds.field("animation_name", &self.animation_name());
        ds.finish()
    }
}
