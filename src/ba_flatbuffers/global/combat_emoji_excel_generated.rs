extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::EmojiEvent;

#[derive(Copy, Clone, PartialEq)]
pub struct CombatEmojiExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CombatEmojiExcel<'a> {
    type Inner = CombatEmojiExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CombatEmojiExcel<'a> {
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_EMOJI_EVENT: ::flatbuffers::VOffsetT = 6;
    pub const VT_ORDER_OF_PRIORITY: ::flatbuffers::VOffsetT = 8;
    pub const VT_EMOJI_DURATION: ::flatbuffers::VOffsetT = 10;
    pub const VT_EMOJI_REVERSAL: ::flatbuffers::VOffsetT = 12;
    pub const VT_EMOJI_TURN_ON: ::flatbuffers::VOffsetT = 14;
    pub const VT_SHOW_EMOJI_DELAY: ::flatbuffers::VOffsetT = 16;
    pub const VT_SHOW_DEFAULT_BG: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CombatEmojiExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn emoji_event(&self) -> EmojiEvent {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EmojiEvent>(
                    CombatEmojiExcel::VT_EMOJI_EVENT,
                    Some(EmojiEvent::EnterConver),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn order_of_priority(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CombatEmojiExcel::VT_ORDER_OF_PRIORITY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn emoji_duration(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CombatEmojiExcel::VT_EMOJI_DURATION, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn emoji_reversal(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CombatEmojiExcel::VT_EMOJI_REVERSAL, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn emoji_turn_on(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CombatEmojiExcel::VT_EMOJI_TURN_ON, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn show_emoji_delay(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CombatEmojiExcel::VT_SHOW_EMOJI_DELAY, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn show_default_bg(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CombatEmojiExcel::VT_SHOW_DEFAULT_BG, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CombatEmojiExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<EmojiEvent>("emoji_event", Self::VT_EMOJI_EVENT, false)?
            .visit_field::<i32>("order_of_priority", Self::VT_ORDER_OF_PRIORITY, false)?
            .visit_field::<bool>("emoji_duration", Self::VT_EMOJI_DURATION, false)?
            .visit_field::<bool>("emoji_reversal", Self::VT_EMOJI_REVERSAL, false)?
            .visit_field::<bool>("emoji_turn_on", Self::VT_EMOJI_TURN_ON, false)?
            .visit_field::<i32>("show_emoji_delay", Self::VT_SHOW_EMOJI_DELAY, false)?
            .visit_field::<bool>("show_default_bg", Self::VT_SHOW_DEFAULT_BG, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CombatEmojiExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CombatEmojiExcel", 8)?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("emoji_event", &self.emoji_event())?;
        s.serialize_field("order_of_priority", &self.order_of_priority())?;
        s.serialize_field("emoji_duration", &self.emoji_duration())?;
        s.serialize_field("emoji_reversal", &self.emoji_reversal())?;
        s.serialize_field("emoji_turn_on", &self.emoji_turn_on())?;
        s.serialize_field("show_emoji_delay", &self.show_emoji_delay())?;
        s.serialize_field("show_default_bg", &self.show_default_bg())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CombatEmojiExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CombatEmojiExcel");
        ds.field("unique_id", &self.unique_id());
        ds.field("emoji_event", &self.emoji_event());
        ds.field("order_of_priority", &self.order_of_priority());
        ds.field("emoji_duration", &self.emoji_duration());
        ds.field("emoji_reversal", &self.emoji_reversal());
        ds.field("emoji_turn_on", &self.emoji_turn_on());
        ds.field("show_emoji_delay", &self.show_emoji_delay());
        ds.field("show_default_bg", &self.show_default_bg());
        ds.finish()
    }
}
