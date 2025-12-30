extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{EventNotifyType, EventTargetType};

#[derive(Copy, Clone, PartialEq)]
pub struct EventContentNotifyExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for EventContentNotifyExcel<'a> {
    type Inner = EventContentNotifyExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> EventContentNotifyExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_ICON_PATH: ::flatbuffers::VOffsetT = 8;
    pub const VT_EVENT_NOTIFY_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_EVENT_TARGET_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_SHORTCUT_EVENT_TARGET_TYPE: ::flatbuffers::VOffsetT = 14;
    pub const VT_IS_SHORTCUT_ENABLE: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn id(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(EventContentNotifyExcel::VT_ID, Some(0))
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
                .get::<u32>(EventContentNotifyExcel::VT_LOCALIZE_ETC_ID, Some(0))
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
                EventContentNotifyExcel::VT_ICON_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn event_notify_type(&self) -> EventNotifyType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EventNotifyType>(
                    EventContentNotifyExcel::VT_EVENT_NOTIFY_TYPE,
                    Some(EventNotifyType::RewardIncreaseEvent),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn event_target_type(&self) -> EventTargetType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EventTargetType>(
                    EventContentNotifyExcel::VT_EVENT_TARGET_TYPE,
                    Some(EventTargetType::WeekDungeon),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn shortcut_event_target_type(&self) -> EventTargetType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EventTargetType>(
                    EventContentNotifyExcel::VT_SHORTCUT_EVENT_TARGET_TYPE,
                    Some(EventTargetType::WeekDungeon),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn is_shortcut_enable(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(EventContentNotifyExcel::VT_IS_SHORTCUT_ENABLE, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for EventContentNotifyExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i32>("id", Self::VT_ID, false)?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "icon_path",
                Self::VT_ICON_PATH,
                false,
            )?
            .visit_field::<EventNotifyType>("event_notify_type", Self::VT_EVENT_NOTIFY_TYPE, false)?
            .visit_field::<EventTargetType>("event_target_type", Self::VT_EVENT_TARGET_TYPE, false)?
            .visit_field::<EventTargetType>(
                "shortcut_event_target_type",
                Self::VT_SHORTCUT_EVENT_TARGET_TYPE,
                false,
            )?
            .visit_field::<bool>("is_shortcut_enable", Self::VT_IS_SHORTCUT_ENABLE, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for EventContentNotifyExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventContentNotifyExcel", 7)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.icon_path() {
            s.serialize_field("icon_path", &f)?;
        } else {
            s.skip_field("icon_path")?;
        }
        s.serialize_field("event_notify_type", &self.event_notify_type())?;
        s.serialize_field("event_target_type", &self.event_target_type())?;
        s.serialize_field(
            "shortcut_event_target_type",
            &self.shortcut_event_target_type(),
        )?;
        s.serialize_field("is_shortcut_enable", &self.is_shortcut_enable())?;
        s.end()
    }
}

impl ::core::fmt::Debug for EventContentNotifyExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("EventContentNotifyExcel");
        ds.field("id", &self.id());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("icon_path", &self.icon_path());
        ds.field("event_notify_type", &self.event_notify_type());
        ds.field("event_target_type", &self.event_target_type());
        ds.field(
            "shortcut_event_target_type",
            &self.shortcut_event_target_type(),
        );
        ds.field("is_shortcut_enable", &self.is_shortcut_enable());
        ds.finish()
    }
}
