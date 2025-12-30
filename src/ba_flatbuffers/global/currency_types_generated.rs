extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CurrencyTypes(pub i32);
#[allow(non_upper_case_globals)]
impl CurrencyTypes {
    pub const Invalid: Self = Self(0);
    pub const Gold: Self = Self(1);
    pub const GemPaid: Self = Self(2);
    pub const GemBonus: Self = Self(3);
    pub const Gem: Self = Self(4);
    pub const ActionPoint: Self = Self(5);
    pub const AcademyTicket: Self = Self(6);
    pub const ArenaTicket: Self = Self(7);
    pub const RaidTicket: Self = Self(8);
    pub const WeekDungeonChaserATicket: Self = Self(9);
    pub const WeekDungeonFindGiftTicket: Self = Self(10);
    pub const WeekDungeonBloodTicket: Self = Self(11);
    pub const WeekDungeonChaserBTicket: Self = Self(12);
    pub const WeekDungeonChaserCTicket: Self = Self(13);
    pub const SchoolDungeonATicket: Self = Self(14);
    pub const SchoolDungeonBTicket: Self = Self(15);
    pub const SchoolDungeonCTicket: Self = Self(16);
    pub const TimeAttackDungeonTicket: Self = Self(17);
    pub const MasterCoin: Self = Self(18);
    pub const WorldRaidTicketA: Self = Self(19);
    pub const WorldRaidTicketB: Self = Self(20);
    pub const WorldRaidTicketC: Self = Self(21);
    pub const ChaserTotalTicket: Self = Self(22);
    pub const SchoolDungeonTotalTicket: Self = Self(23);
    pub const EliminateTicketA: Self = Self(24);
    pub const EliminateTicketB: Self = Self(25);
    pub const EliminateTicketC: Self = Self(26);
    pub const EliminateTicketD: Self = Self(27);
    pub const Max: Self = Self(28);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Invalid,
        Self::Gold,
        Self::GemPaid,
        Self::GemBonus,
        Self::Gem,
        Self::ActionPoint,
        Self::AcademyTicket,
        Self::ArenaTicket,
        Self::RaidTicket,
        Self::WeekDungeonChaserATicket,
        Self::WeekDungeonFindGiftTicket,
        Self::WeekDungeonBloodTicket,
        Self::WeekDungeonChaserBTicket,
        Self::WeekDungeonChaserCTicket,
        Self::SchoolDungeonATicket,
        Self::SchoolDungeonBTicket,
        Self::SchoolDungeonCTicket,
        Self::TimeAttackDungeonTicket,
        Self::MasterCoin,
        Self::WorldRaidTicketA,
        Self::WorldRaidTicketB,
        Self::WorldRaidTicketC,
        Self::ChaserTotalTicket,
        Self::SchoolDungeonTotalTicket,
        Self::EliminateTicketA,
        Self::EliminateTicketB,
        Self::EliminateTicketC,
        Self::EliminateTicketD,
        Self::Max,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Invalid => Some("Invalid"),
            Self::Gold => Some("Gold"),
            Self::GemPaid => Some("GemPaid"),
            Self::GemBonus => Some("GemBonus"),
            Self::Gem => Some("Gem"),
            Self::ActionPoint => Some("ActionPoint"),
            Self::AcademyTicket => Some("AcademyTicket"),
            Self::ArenaTicket => Some("ArenaTicket"),
            Self::RaidTicket => Some("RaidTicket"),
            Self::WeekDungeonChaserATicket => Some("WeekDungeonChaserATicket"),
            Self::WeekDungeonFindGiftTicket => Some("WeekDungeonFindGiftTicket"),
            Self::WeekDungeonBloodTicket => Some("WeekDungeonBloodTicket"),
            Self::WeekDungeonChaserBTicket => Some("WeekDungeonChaserBTicket"),
            Self::WeekDungeonChaserCTicket => Some("WeekDungeonChaserCTicket"),
            Self::SchoolDungeonATicket => Some("SchoolDungeonATicket"),
            Self::SchoolDungeonBTicket => Some("SchoolDungeonBTicket"),
            Self::SchoolDungeonCTicket => Some("SchoolDungeonCTicket"),
            Self::TimeAttackDungeonTicket => Some("TimeAttackDungeonTicket"),
            Self::MasterCoin => Some("MasterCoin"),
            Self::WorldRaidTicketA => Some("WorldRaidTicketA"),
            Self::WorldRaidTicketB => Some("WorldRaidTicketB"),
            Self::WorldRaidTicketC => Some("WorldRaidTicketC"),
            Self::ChaserTotalTicket => Some("ChaserTotalTicket"),
            Self::SchoolDungeonTotalTicket => Some("SchoolDungeonTotalTicket"),
            Self::EliminateTicketA => Some("EliminateTicketA"),
            Self::EliminateTicketB => Some("EliminateTicketB"),
            Self::EliminateTicketC => Some("EliminateTicketC"),
            Self::EliminateTicketD => Some("EliminateTicketD"),
            Self::Max => Some("Max"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for CurrencyTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for CurrencyTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "CurrencyTypes",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for CurrencyTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in CurrencyTypes::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown CurrencyTypes variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for CurrencyTypes {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for CurrencyTypes {
    type Output = CurrencyTypes;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for CurrencyTypes {
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

impl<'a> ::flatbuffers::Verifiable for CurrencyTypes {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for CurrencyTypes {}
