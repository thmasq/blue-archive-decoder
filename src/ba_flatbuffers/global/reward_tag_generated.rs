extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RewardTag(pub i32);
#[allow(non_upper_case_globals)]
impl RewardTag {
    pub const Default: Self = Self(0);
    pub const FirstClear: Self = Self(1);
    pub const StrategyObject: Self = Self(2);
    pub const Event: Self = Self(3);
    pub const ThreeStar: Self = Self(4);
    pub const ProductMonthly: Self = Self(5);
    pub const Rare: Self = Self(6);
    pub const EventBonus: Self = Self(7);
    pub const TimeWeight: Self = Self(8);
    pub const ProductWeekly: Self = Self(9);
    pub const ProductBiweekly: Self = Self(10);
    pub const EventPermanentReward: Self = Self(11);
    pub const ConquestManageEvent: Self = Self(12);
    pub const ConquestManageDefault: Self = Self(13);
    pub const ConquestCalculateDefault: Self = Self(14);
    pub const ConquestCalculateLevel2: Self = Self(15);
    pub const ConquestCalculateLevel3: Self = Self(16);
    pub const ConquestFootholdUpgrade2: Self = Self(17);
    pub const ConquestFootholdUpgrade3: Self = Self(18);
    pub const ConquestErosionPenalty: Self = Self(19);
    pub const GemBonus: Self = Self(20);
    pub const GemPaid: Self = Self(21);
    pub const ConquestTileConquer: Self = Self(22);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Default,
        Self::FirstClear,
        Self::StrategyObject,
        Self::Event,
        Self::ThreeStar,
        Self::ProductMonthly,
        Self::Rare,
        Self::EventBonus,
        Self::TimeWeight,
        Self::ProductWeekly,
        Self::ProductBiweekly,
        Self::EventPermanentReward,
        Self::ConquestManageEvent,
        Self::ConquestManageDefault,
        Self::ConquestCalculateDefault,
        Self::ConquestCalculateLevel2,
        Self::ConquestCalculateLevel3,
        Self::ConquestFootholdUpgrade2,
        Self::ConquestFootholdUpgrade3,
        Self::ConquestErosionPenalty,
        Self::GemBonus,
        Self::GemPaid,
        Self::ConquestTileConquer,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Default => Some("Default"),
            Self::FirstClear => Some("FirstClear"),
            Self::StrategyObject => Some("StrategyObject"),
            Self::Event => Some("Event"),
            Self::ThreeStar => Some("ThreeStar"),
            Self::ProductMonthly => Some("ProductMonthly"),
            Self::Rare => Some("Rare"),
            Self::EventBonus => Some("EventBonus"),
            Self::TimeWeight => Some("TimeWeight"),
            Self::ProductWeekly => Some("ProductWeekly"),
            Self::ProductBiweekly => Some("ProductBiweekly"),
            Self::EventPermanentReward => Some("EventPermanentReward"),
            Self::ConquestManageEvent => Some("ConquestManageEvent"),
            Self::ConquestManageDefault => Some("ConquestManageDefault"),
            Self::ConquestCalculateDefault => Some("ConquestCalculateDefault"),
            Self::ConquestCalculateLevel2 => Some("ConquestCalculateLevel2"),
            Self::ConquestCalculateLevel3 => Some("ConquestCalculateLevel3"),
            Self::ConquestFootholdUpgrade2 => Some("ConquestFootholdUpgrade2"),
            Self::ConquestFootholdUpgrade3 => Some("ConquestFootholdUpgrade3"),
            Self::ConquestErosionPenalty => Some("ConquestErosionPenalty"),
            Self::GemBonus => Some("GemBonus"),
            Self::GemPaid => Some("GemPaid"),
            Self::ConquestTileConquer => Some("ConquestTileConquer"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for RewardTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for RewardTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant("RewardTag", self.0 as u32, self.variant_name().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for RewardTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in RewardTag::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown RewardTag variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for RewardTag {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for RewardTag {
    type Output = RewardTag;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for RewardTag {
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

impl<'a> ::flatbuffers::Verifiable for RewardTag {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for RewardTag {}
