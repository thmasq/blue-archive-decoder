extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ContentType, SchoolDungeonType, WeekDungeonType};

#[derive(Copy, Clone, PartialEq)]
pub struct FarmingDungeonLocationManageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for FarmingDungeonLocationManageExcel<'a> {
    type Inner = FarmingDungeonLocationManageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> FarmingDungeonLocationManageExcel<'a> {
    pub const VT_FARMING_DUNGEON_LOCATION_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_WEEK_DUNGEON_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_SCHOOL_DUNGEON_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_ORDER: ::flatbuffers::VOffsetT = 12;
    pub const VT_OPEN_START_DATE_TIME: ::flatbuffers::VOffsetT = 14;
    pub const VT_OPEN_END_DATE_TIME: ::flatbuffers::VOffsetT = 16;
    pub const VT_LOCATION_BUTTON_IMAGE_PATH: ::flatbuffers::VOffsetT = 18;
    pub const VT_LOCALIZE_CODE_TITLE: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOCALIZE_CODE_INFO: ::flatbuffers::VOffsetT = 22;

    #[inline]
    pub fn farming_dungeon_location_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    FarmingDungeonLocationManageExcel::VT_FARMING_DUNGEON_LOCATION_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn content_type(&self) -> ContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ContentType>(
                    FarmingDungeonLocationManageExcel::VT_CONTENT_TYPE,
                    Some(ContentType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn week_dungeon_type(&self) -> WeekDungeonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<WeekDungeonType>(
                    FarmingDungeonLocationManageExcel::VT_WEEK_DUNGEON_TYPE,
                    Some(WeekDungeonType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn school_dungeon_type(&self) -> SchoolDungeonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<SchoolDungeonType>(
                    FarmingDungeonLocationManageExcel::VT_SCHOOL_DUNGEON_TYPE,
                    Some(SchoolDungeonType::SchoolA),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(FarmingDungeonLocationManageExcel::VT_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn open_start_date_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                FarmingDungeonLocationManageExcel::VT_OPEN_START_DATE_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn open_end_date_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                FarmingDungeonLocationManageExcel::VT_OPEN_END_DATE_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn location_button_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                FarmingDungeonLocationManageExcel::VT_LOCATION_BUTTON_IMAGE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_code_title(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    FarmingDungeonLocationManageExcel::VT_LOCALIZE_CODE_TITLE,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_code_info(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    FarmingDungeonLocationManageExcel::VT_LOCALIZE_CODE_INFO,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for FarmingDungeonLocationManageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "farming_dungeon_location_id",
                Self::VT_FARMING_DUNGEON_LOCATION_ID,
                false,
            )?
            .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
            .visit_field::<WeekDungeonType>("week_dungeon_type", Self::VT_WEEK_DUNGEON_TYPE, false)?
            .visit_field::<SchoolDungeonType>(
                "school_dungeon_type",
                Self::VT_SCHOOL_DUNGEON_TYPE,
                false,
            )?
            .visit_field::<i64>("order", Self::VT_ORDER, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "open_start_date_time",
                Self::VT_OPEN_START_DATE_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "open_end_date_time",
                Self::VT_OPEN_END_DATE_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "location_button_image_path",
                Self::VT_LOCATION_BUTTON_IMAGE_PATH,
                false,
            )?
            .visit_field::<u32>("localize_code_title", Self::VT_LOCALIZE_CODE_TITLE, false)?
            .visit_field::<u32>("localize_code_info", Self::VT_LOCALIZE_CODE_INFO, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for FarmingDungeonLocationManageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FarmingDungeonLocationManageExcel", 10)?;
        s.serialize_field(
            "farming_dungeon_location_id",
            &self.farming_dungeon_location_id(),
        )?;
        s.serialize_field("content_type", &self.content_type())?;
        s.serialize_field("week_dungeon_type", &self.week_dungeon_type())?;
        s.serialize_field("school_dungeon_type", &self.school_dungeon_type())?;
        s.serialize_field("order", &self.order())?;
        if let Some(f) = self.open_start_date_time() {
            s.serialize_field("open_start_date_time", &f)?;
        } else {
            s.skip_field("open_start_date_time")?;
        }
        if let Some(f) = self.open_end_date_time() {
            s.serialize_field("open_end_date_time", &f)?;
        } else {
            s.skip_field("open_end_date_time")?;
        }
        if let Some(f) = self.location_button_image_path() {
            s.serialize_field("location_button_image_path", &f)?;
        } else {
            s.skip_field("location_button_image_path")?;
        }
        s.serialize_field("localize_code_title", &self.localize_code_title())?;
        s.serialize_field("localize_code_info", &self.localize_code_info())?;
        s.end()
    }
}

impl ::core::fmt::Debug for FarmingDungeonLocationManageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("FarmingDungeonLocationManageExcel");
        ds.field(
            "farming_dungeon_location_id",
            &self.farming_dungeon_location_id(),
        );
        ds.field("content_type", &self.content_type());
        ds.field("week_dungeon_type", &self.week_dungeon_type());
        ds.field("school_dungeon_type", &self.school_dungeon_type());
        ds.field("order", &self.order());
        ds.field("open_start_date_time", &self.open_start_date_time());
        ds.field("open_end_date_time", &self.open_end_date_time());
        ds.field(
            "location_button_image_path",
            &self.location_button_image_path(),
        );
        ds.field("localize_code_title", &self.localize_code_title());
        ds.field("localize_code_info", &self.localize_code_info());
        ds.finish()
    }
}
