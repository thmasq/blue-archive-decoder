extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::DreamMakerParameterType;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDreamParameterExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDreamParameterExcel<'a> {
    type Inner = MiniGameDreamParameterExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDreamParameterExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_PARAMETER_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 10;
    pub const VT_ICON_PATH: ::flatbuffers::VOffsetT = 12;
    pub const VT_PARAMETER_BASE: ::flatbuffers::VOffsetT = 14;
    pub const VT_PARAMETER_BASE_MAX: ::flatbuffers::VOffsetT = 16;
    pub const VT_PARAMETER_MIN: ::flatbuffers::VOffsetT = 18;
    pub const VT_PARAMETER_MAX: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn parameter_type(&self) -> DreamMakerParameterType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DreamMakerParameterType>(
                    MiniGameDreamParameterExcel::VT_PARAMETER_TYPE,
                    Some(DreamMakerParameterType::None),
                )
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
                .get::<u32>(MiniGameDreamParameterExcel::VT_LOCALIZE_ETC_ID, Some(0))
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
                MiniGameDreamParameterExcel::VT_ICON_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn parameter_base(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_PARAMETER_BASE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn parameter_base_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_PARAMETER_BASE_MAX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn parameter_min(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_PARAMETER_MIN, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn parameter_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDreamParameterExcel::VT_PARAMETER_MAX, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDreamParameterExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<DreamMakerParameterType>(
                "parameter_type",
                Self::VT_PARAMETER_TYPE,
                false,
            )?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "icon_path",
                Self::VT_ICON_PATH,
                false,
            )?
            .visit_field::<i64>("parameter_base", Self::VT_PARAMETER_BASE, false)?
            .visit_field::<i64>("parameter_base_max", Self::VT_PARAMETER_BASE_MAX, false)?
            .visit_field::<i64>("parameter_min", Self::VT_PARAMETER_MIN, false)?
            .visit_field::<i64>("parameter_max", Self::VT_PARAMETER_MAX, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDreamParameterExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDreamParameterExcel", 9)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("parameter_type", &self.parameter_type())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.icon_path() {
            s.serialize_field("icon_path", &f)?;
        } else {
            s.skip_field("icon_path")?;
        }
        s.serialize_field("parameter_base", &self.parameter_base())?;
        s.serialize_field("parameter_base_max", &self.parameter_base_max())?;
        s.serialize_field("parameter_min", &self.parameter_min())?;
        s.serialize_field("parameter_max", &self.parameter_max())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDreamParameterExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDreamParameterExcel");
        ds.field("id", &self.id());
        ds.field("event_content_id", &self.event_content_id());
        ds.field("parameter_type", &self.parameter_type());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("icon_path", &self.icon_path());
        ds.field("parameter_base", &self.parameter_base());
        ds.field("parameter_base_max", &self.parameter_base_max());
        ds.field("parameter_min", &self.parameter_min());
        ds.field("parameter_max", &self.parameter_max());
        ds.finish()
    }
}
