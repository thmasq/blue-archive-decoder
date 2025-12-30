extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{MessagePopupButtonType, MessagePopupImagePositionType, MessagePopupLayout};

#[derive(Copy, Clone, PartialEq)]
pub struct MessagePopupExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MessagePopupExcel<'a> {
    type Inner = MessagePopupExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MessagePopupExcel<'a> {
    pub const VT_STRING_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_MESSAGE_POPUP_LAYOUT: ::flatbuffers::VOffsetT = 6;
    pub const VT_ORDER_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_IMAGE: ::flatbuffers::VOffsetT = 10;
    pub const VT_TITLE_TEXT: ::flatbuffers::VOffsetT = 12;
    pub const VT_SUB_TITLE_TEXT: ::flatbuffers::VOffsetT = 14;
    pub const VT_MESSAGE_TEXT: ::flatbuffers::VOffsetT = 16;
    pub const VT_CONDITION_TEXT: ::flatbuffers::VOffsetT = 18;
    pub const VT_DISPLAY_X_BUTTON: ::flatbuffers::VOffsetT = 20;
    pub const VT_BUTTON: ::flatbuffers::VOffsetT = 22;
    pub const VT_BUTTON_TEXT: ::flatbuffers::VOffsetT = 24;
    pub const VT_BUTTON_COMMAND: ::flatbuffers::VOffsetT = 26;
    pub const VT_BUTTON_PARAMETER: ::flatbuffers::VOffsetT = 28;

    #[inline]
    pub fn string_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MessagePopupExcel::VT_STRING_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn message_popup_layout(&self) -> MessagePopupLayout {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MessagePopupLayout>(
                    MessagePopupExcel::VT_MESSAGE_POPUP_LAYOUT,
                    Some(MessagePopupLayout::TextOnly),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn order_type(&self) -> MessagePopupImagePositionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MessagePopupImagePositionType>(
                    MessagePopupExcel::VT_ORDER_TYPE,
                    Some(MessagePopupImagePositionType::ImageFirst),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn image(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(MessagePopupExcel::VT_IMAGE, None)
        }
    }
    #[inline]
    pub fn title_text(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MessagePopupExcel::VT_TITLE_TEXT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sub_title_text(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MessagePopupExcel::VT_SUB_TITLE_TEXT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn message_text(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MessagePopupExcel::VT_MESSAGE_TEXT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn condition_text(&self) -> Option<::flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, u32>>>(
                    MessagePopupExcel::VT_CONDITION_TEXT,
                    None,
                )
        }
    }
    #[inline]
    pub fn display_x_button(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(MessagePopupExcel::VT_DISPLAY_X_BUTTON, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn button(&self) -> Option<::flatbuffers::Vector<'a, MessagePopupButtonType>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, MessagePopupButtonType>,
            >>(MessagePopupExcel::VT_BUTTON, None)
        }
    }
    #[inline]
    pub fn button_text(&self) -> Option<::flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, u32>>>(
                    MessagePopupExcel::VT_BUTTON_TEXT,
                    None,
                )
        }
    }
    #[inline]
    pub fn button_command(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(MessagePopupExcel::VT_BUTTON_COMMAND, None)
        }
    }
    #[inline]
    pub fn button_parameter(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(MessagePopupExcel::VT_BUTTON_PARAMETER, None)
        }
    }
}

impl ::flatbuffers::Verifiable for MessagePopupExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
     .visit_field::<u32>("string_id", Self::VT_STRING_ID, false)?
     .visit_field::<MessagePopupLayout>("message_popup_layout", Self::VT_MESSAGE_POPUP_LAYOUT, false)?
     .visit_field::<MessagePopupImagePositionType>("order_type", Self::VT_ORDER_TYPE, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("image", Self::VT_IMAGE, false)?
     .visit_field::<u32>("title_text", Self::VT_TITLE_TEXT, false)?
     .visit_field::<u32>("sub_title_text", Self::VT_SUB_TITLE_TEXT, false)?
     .visit_field::<u32>("message_text", Self::VT_MESSAGE_TEXT, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>("condition_text", Self::VT_CONDITION_TEXT, false)?
     .visit_field::<bool>("display_x_button", Self::VT_DISPLAY_X_BUTTON, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, MessagePopupButtonType>>>("button", Self::VT_BUTTON, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, u32>>>("button_text", Self::VT_BUTTON_TEXT, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>>>("button_command", Self::VT_BUTTON_COMMAND, false)?
     .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>>>("button_parameter", Self::VT_BUTTON_PARAMETER, false)?
     .finish();
        Ok(())
    }
}

impl Serialize for MessagePopupExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MessagePopupExcel", 13)?;
        s.serialize_field("string_id", &self.string_id())?;
        s.serialize_field("message_popup_layout", &self.message_popup_layout())?;
        s.serialize_field("order_type", &self.order_type())?;
        if let Some(f) = self.image() {
            s.serialize_field("image", &f)?;
        } else {
            s.skip_field("image")?;
        }
        s.serialize_field("title_text", &self.title_text())?;
        s.serialize_field("sub_title_text", &self.sub_title_text())?;
        s.serialize_field("message_text", &self.message_text())?;
        if let Some(f) = self.condition_text() {
            s.serialize_field("condition_text", &f)?;
        } else {
            s.skip_field("condition_text")?;
        }
        s.serialize_field("display_x_button", &self.display_x_button())?;
        if let Some(f) = self.button() {
            s.serialize_field("button", &f)?;
        } else {
            s.skip_field("button")?;
        }
        if let Some(f) = self.button_text() {
            s.serialize_field("button_text", &f)?;
        } else {
            s.skip_field("button_text")?;
        }
        if let Some(f) = self.button_command() {
            s.serialize_field("button_command", &f)?;
        } else {
            s.skip_field("button_command")?;
        }
        if let Some(f) = self.button_parameter() {
            s.serialize_field("button_parameter", &f)?;
        } else {
            s.skip_field("button_parameter")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MessagePopupExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MessagePopupExcel");
        ds.field("string_id", &self.string_id());
        ds.field("message_popup_layout", &self.message_popup_layout());
        ds.field("order_type", &self.order_type());
        ds.field("image", &self.image());
        ds.field("title_text", &self.title_text());
        ds.field("sub_title_text", &self.sub_title_text());
        ds.field("message_text", &self.message_text());
        ds.field("condition_text", &self.condition_text());
        ds.field("display_x_button", &self.display_x_button());
        ds.field("button", &self.button());
        ds.field("button_text", &self.button_text());
        ds.field("button_command", &self.button_command());
        ds.field("button_parameter", &self.button_parameter());
        ds.finish()
    }
}
