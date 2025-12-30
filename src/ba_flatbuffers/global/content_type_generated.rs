extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ContentType(pub i32);
#[allow(non_upper_case_globals)]
impl ContentType {
    pub const None: Self = Self(0);
    pub const CampaignMainStage: Self = Self(1);
    pub const CampaignSubStage: Self = Self(2);
    pub const WeekDungeon: Self = Self(3);
    pub const EventContentMainStage: Self = Self(4);
    pub const EventContentSubStage: Self = Self(5);
    pub const CampaignTutorialStage: Self = Self(6);
    pub const EventContentMainGroundStage: Self = Self(7);
    pub const SchoolDungeon: Self = Self(8);
    pub const TimeAttackDungeon: Self = Self(9);
    pub const Raid: Self = Self(10);
    pub const Conquest: Self = Self(11);
    pub const EventContentStoryStage: Self = Self(12);
    pub const CampaignExtraStage: Self = Self(13);
    pub const StoryStrategyStage: Self = Self(14);
    pub const ScenarioMode: Self = Self(15);
    pub const EventContent: Self = Self(16);
    pub const WorldRaid: Self = Self(17);
    pub const EliminateRaid: Self = Self(18);
    pub const Chaser: Self = Self(19);
    pub const FieldContentStage: Self = Self(20);
    pub const MultiFloorRaid: Self = Self(21);
    pub const MinigameDefense: Self = Self(22);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::None,
        Self::CampaignMainStage,
        Self::CampaignSubStage,
        Self::WeekDungeon,
        Self::EventContentMainStage,
        Self::EventContentSubStage,
        Self::CampaignTutorialStage,
        Self::EventContentMainGroundStage,
        Self::SchoolDungeon,
        Self::TimeAttackDungeon,
        Self::Raid,
        Self::Conquest,
        Self::EventContentStoryStage,
        Self::CampaignExtraStage,
        Self::StoryStrategyStage,
        Self::ScenarioMode,
        Self::EventContent,
        Self::WorldRaid,
        Self::EliminateRaid,
        Self::Chaser,
        Self::FieldContentStage,
        Self::MultiFloorRaid,
        Self::MinigameDefense,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::None => Some("None"),
            Self::CampaignMainStage => Some("CampaignMainStage"),
            Self::CampaignSubStage => Some("CampaignSubStage"),
            Self::WeekDungeon => Some("WeekDungeon"),
            Self::EventContentMainStage => Some("EventContentMainStage"),
            Self::EventContentSubStage => Some("EventContentSubStage"),
            Self::CampaignTutorialStage => Some("CampaignTutorialStage"),
            Self::EventContentMainGroundStage => Some("EventContentMainGroundStage"),
            Self::SchoolDungeon => Some("SchoolDungeon"),
            Self::TimeAttackDungeon => Some("TimeAttackDungeon"),
            Self::Raid => Some("Raid"),
            Self::Conquest => Some("Conquest"),
            Self::EventContentStoryStage => Some("EventContentStoryStage"),
            Self::CampaignExtraStage => Some("CampaignExtraStage"),
            Self::StoryStrategyStage => Some("StoryStrategyStage"),
            Self::ScenarioMode => Some("ScenarioMode"),
            Self::EventContent => Some("EventContent"),
            Self::WorldRaid => Some("WorldRaid"),
            Self::EliminateRaid => Some("EliminateRaid"),
            Self::Chaser => Some("Chaser"),
            Self::FieldContentStage => Some("FieldContentStage"),
            Self::MultiFloorRaid => Some("MultiFloorRaid"),
            Self::MinigameDefense => Some("MinigameDefense"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for ContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "ContentType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in ContentType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown ContentType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for ContentType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for ContentType {
    type Output = ContentType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for ContentType {
    type Scalar = i32;
    #[inline]
    fn to_little_endian(self) -> i32 {
        self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: i32) -> Self {
        let b = i32::from_le(v);
        Self(b)
    }
}

impl<'a> ::flatbuffers::Verifiable for ContentType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for ContentType {}
