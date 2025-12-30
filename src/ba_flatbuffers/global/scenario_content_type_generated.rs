extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ScenarioContentType(pub i32);
#[allow(non_upper_case_globals)]
impl ScenarioContentType {
    pub const Prologue: Self = Self(0);
    pub const WeekDungeon: Self = Self(1);
    pub const Raid: Self = Self(2);
    pub const Arena: Self = Self(3);
    pub const Favor: Self = Self(4);
    pub const Shop: Self = Self(5);
    pub const EventContent: Self = Self(6);
    pub const Craft: Self = Self(7);
    pub const Chaser: Self = Self(8);
    pub const EventContentMeetup: Self = Self(9);
    pub const TimeAttack: Self = Self(10);
    pub const Mission: Self = Self(11);
    pub const EventContentPermanentPrologue: Self = Self(12);
    pub const EventContentReturnSeason: Self = Self(13);
    pub const MiniEvent: Self = Self(14);
    pub const EliminateRaid: Self = Self(15);
    pub const MultiFloorRaid: Self = Self(16);
    pub const EventContentPermanent: Self = Self(17);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Prologue,
        Self::WeekDungeon,
        Self::Raid,
        Self::Arena,
        Self::Favor,
        Self::Shop,
        Self::EventContent,
        Self::Craft,
        Self::Chaser,
        Self::EventContentMeetup,
        Self::TimeAttack,
        Self::Mission,
        Self::EventContentPermanentPrologue,
        Self::EventContentReturnSeason,
        Self::MiniEvent,
        Self::EliminateRaid,
        Self::MultiFloorRaid,
        Self::EventContentPermanent,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Prologue => Some("Prologue"),
            Self::WeekDungeon => Some("WeekDungeon"),
            Self::Raid => Some("Raid"),
            Self::Arena => Some("Arena"),
            Self::Favor => Some("Favor"),
            Self::Shop => Some("Shop"),
            Self::EventContent => Some("EventContent"),
            Self::Craft => Some("Craft"),
            Self::Chaser => Some("Chaser"),
            Self::EventContentMeetup => Some("EventContentMeetup"),
            Self::TimeAttack => Some("TimeAttack"),
            Self::Mission => Some("Mission"),
            Self::EventContentPermanentPrologue => Some("EventContentPermanentPrologue"),
            Self::EventContentReturnSeason => Some("EventContentReturnSeason"),
            Self::MiniEvent => Some("MiniEvent"),
            Self::EliminateRaid => Some("EliminateRaid"),
            Self::MultiFloorRaid => Some("MultiFloorRaid"),
            Self::EventContentPermanent => Some("EventContentPermanent"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for ScenarioContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for ScenarioContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "ScenarioContentType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for ScenarioContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in ScenarioContentType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown ScenarioContentType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioContentType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for ScenarioContentType {
    type Output = ScenarioContentType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for ScenarioContentType {
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

impl<'a> ::flatbuffers::Verifiable for ScenarioContentType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for ScenarioContentType {}
