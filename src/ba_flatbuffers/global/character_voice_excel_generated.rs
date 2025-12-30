extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{CVCollectionType, Nation};

#[derive(Copy, Clone, PartialEq)]
pub struct CharacterVoiceExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CharacterVoiceExcel<'a> {
    type Inner = CharacterVoiceExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CharacterVoiceExcel<'a> {
    pub const VT_CHARACTER_VOICE_UNIQUE_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CHARACTER_VOICE_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_VOICE_HASH: ::flatbuffers::VOffsetT = 8;
    pub const VT_ONLY_ONE: ::flatbuffers::VOffsetT = 10;
    pub const VT_PRIORITY: ::flatbuffers::VOffsetT = 12;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 14;
    pub const VT_COLLECTION_VISIBLE: ::flatbuffers::VOffsetT = 16;
    pub const VT_CV_COLLECTION_TYPE: ::flatbuffers::VOffsetT = 18;
    pub const VT_UNLOCK_FAVOR_RANK: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOCALIZE_CV_GROUP: ::flatbuffers::VOffsetT = 22;
    pub const VT_NATION: ::flatbuffers::VOffsetT = 24;
    pub const VT_VOLUME: ::flatbuffers::VOffsetT = 26;
    pub const VT_DELAY: ::flatbuffers::VOffsetT = 28;
    pub const VT_PATH: ::flatbuffers::VOffsetT = 30;

    #[inline]
    pub fn character_voice_unique_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterVoiceExcel::VT_CHARACTER_VOICE_UNIQUE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn character_voice_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterVoiceExcel::VT_CHARACTER_VOICE_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn voice_hash(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(CharacterVoiceExcel::VT_VOICE_HASH, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn only_one(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterVoiceExcel::VT_ONLY_ONE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn priority(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CharacterVoiceExcel::VT_PRIORITY, Some(0))
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
                .get::<i64>(CharacterVoiceExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn collection_visible(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(CharacterVoiceExcel::VT_COLLECTION_VISIBLE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn cv_collection_type(&self) -> CVCollectionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CVCollectionType>(
                    CharacterVoiceExcel::VT_CV_COLLECTION_TYPE,
                    Some(CVCollectionType::CVNormal),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_favor_rank(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CharacterVoiceExcel::VT_UNLOCK_FAVOR_RANK, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn localize_cv_group(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CharacterVoiceExcel::VT_LOCALIZE_CV_GROUP,
                None,
            )
        }
    }
    #[inline]
    pub fn nation(&self) -> Option<::flatbuffers::Vector<'a, Nation>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, Nation>>>(
                    CharacterVoiceExcel::VT_NATION,
                    None,
                )
        }
    }
    #[inline]
    pub fn volume(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    CharacterVoiceExcel::VT_VOLUME,
                    None,
                )
        }
    }
    #[inline]
    pub fn delay(&self) -> Option<::flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, f32>>>(
                    CharacterVoiceExcel::VT_DELAY,
                    None,
                )
        }
    }
    #[inline]
    pub fn path(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(CharacterVoiceExcel::VT_PATH, None)
        }
    }
}

impl ::flatbuffers::Verifiable for CharacterVoiceExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "character_voice_unique_id",
                Self::VT_CHARACTER_VOICE_UNIQUE_ID,
                false,
            )?
            .visit_field::<i64>(
                "character_voice_group_id",
                Self::VT_CHARACTER_VOICE_GROUP_ID,
                false,
            )?
            .visit_field::<u32>("voice_hash", Self::VT_VOICE_HASH, false)?
            .visit_field::<bool>("only_one", Self::VT_ONLY_ONE, false)?
            .visit_field::<i32>("priority", Self::VT_PRIORITY, false)?
            .visit_field::<i64>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<bool>("collection_visible", Self::VT_COLLECTION_VISIBLE, false)?
            .visit_field::<CVCollectionType>(
                "cv_collection_type",
                Self::VT_CV_COLLECTION_TYPE,
                false,
            )?
            .visit_field::<i64>("unlock_favor_rank", Self::VT_UNLOCK_FAVOR_RANK, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "localize_cv_group",
                Self::VT_LOCALIZE_CV_GROUP,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, Nation>>>(
                "nation",
                Self::VT_NATION,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "volume",
                Self::VT_VOLUME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, f32>>>(
                "delay",
                Self::VT_DELAY,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("path", Self::VT_PATH, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CharacterVoiceExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CharacterVoiceExcel", 14)?;
        s.serialize_field(
            "character_voice_unique_id",
            &self.character_voice_unique_id(),
        )?;
        s.serialize_field("character_voice_group_id", &self.character_voice_group_id())?;
        s.serialize_field("voice_hash", &self.voice_hash())?;
        s.serialize_field("only_one", &self.only_one())?;
        s.serialize_field("priority", &self.priority())?;
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("collection_visible", &self.collection_visible())?;
        s.serialize_field("cv_collection_type", &self.cv_collection_type())?;
        s.serialize_field("unlock_favor_rank", &self.unlock_favor_rank())?;
        if let Some(f) = self.localize_cv_group() {
            s.serialize_field("localize_cv_group", &f)?;
        } else {
            s.skip_field("localize_cv_group")?;
        }
        if let Some(f) = self.nation() {
            s.serialize_field("nation", &f)?;
        } else {
            s.skip_field("nation")?;
        }
        if let Some(f) = self.volume() {
            s.serialize_field("volume", &f)?;
        } else {
            s.skip_field("volume")?;
        }
        if let Some(f) = self.delay() {
            s.serialize_field("delay", &f)?;
        } else {
            s.skip_field("delay")?;
        }
        if let Some(f) = self.path() {
            s.serialize_field("path", &f)?;
        } else {
            s.skip_field("path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for CharacterVoiceExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CharacterVoiceExcel");
        ds.field(
            "character_voice_unique_id",
            &self.character_voice_unique_id(),
        );
        ds.field("character_voice_group_id", &self.character_voice_group_id());
        ds.field("voice_hash", &self.voice_hash());
        ds.field("only_one", &self.only_one());
        ds.field("priority", &self.priority());
        ds.field("display_order", &self.display_order());
        ds.field("collection_visible", &self.collection_visible());
        ds.field("cv_collection_type", &self.cv_collection_type());
        ds.field("unlock_favor_rank", &self.unlock_favor_rank());
        ds.field("localize_cv_group", &self.localize_cv_group());
        ds.field("nation", &self.nation());
        ds.field("volume", &self.volume());
        ds.field("delay", &self.delay());
        ds.field("path", &self.path());
        ds.finish()
    }
}
