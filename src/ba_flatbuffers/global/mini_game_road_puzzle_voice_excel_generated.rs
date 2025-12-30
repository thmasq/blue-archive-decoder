extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::RoadPuzzleVoiceCondition;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameRoadPuzzleVoiceExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameRoadPuzzleVoiceExcel<'a> {
    type Inner = MiniGameRoadPuzzleVoiceExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameRoadPuzzleVoiceExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_UNIQUE_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_VOICE_CONDITION: ::flatbuffers::VOffsetT = 8;
    pub const VT_VOICE_CLIP: ::flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameRoadPuzzleVoiceExcel::VT_EVENT_CONTENT_ID, Some(0))
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
                .get::<i64>(MiniGameRoadPuzzleVoiceExcel::VT_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn voice_condition(&self) -> RoadPuzzleVoiceCondition {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<RoadPuzzleVoiceCondition>(
                    MiniGameRoadPuzzleVoiceExcel::VT_VOICE_CONDITION,
                    Some(RoadPuzzleVoiceCondition::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn voice_clip(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(MiniGameRoadPuzzleVoiceExcel::VT_VOICE_CLIP, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameRoadPuzzleVoiceExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<i64>("unique_id", Self::VT_UNIQUE_ID, false)?
            .visit_field::<RoadPuzzleVoiceCondition>(
                "voice_condition",
                Self::VT_VOICE_CONDITION,
                false,
            )?
            .visit_field::<u32>("voice_clip", Self::VT_VOICE_CLIP, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameRoadPuzzleVoiceExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameRoadPuzzleVoiceExcel", 4)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field("unique_id", &self.unique_id())?;
        s.serialize_field("voice_condition", &self.voice_condition())?;
        s.serialize_field("voice_clip", &self.voice_clip())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameRoadPuzzleVoiceExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameRoadPuzzleVoiceExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field("unique_id", &self.unique_id());
        ds.field("voice_condition", &self.voice_condition());
        ds.field("voice_clip", &self.voice_clip());
        ds.finish()
    }
}
