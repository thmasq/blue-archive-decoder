extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShortcutContentType(pub i32);
#[allow(non_upper_case_globals)]
impl ShortcutContentType {
    pub const None: Self = Self(0);
    pub const CampaignStage: Self = Self(1);
    pub const EventStage: Self = Self(2);
    pub const Blood: Self = Self(3);
    pub const WeekDungeon: Self = Self(4);
    pub const Arena: Self = Self(5);
    pub const Raid: Self = Self(6);
    pub const Shop: Self = Self(7);
    pub const ItemInventory: Self = Self(8);
    pub const Craft: Self = Self(9);
    pub const SchoolDungeon: Self = Self(10);
    pub const Academy: Self = Self(11);
    pub const Mission: Self = Self(12);
    pub const MultiFloorRaid: Self = Self(13);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::None,
        Self::CampaignStage,
        Self::EventStage,
        Self::Blood,
        Self::WeekDungeon,
        Self::Arena,
        Self::Raid,
        Self::Shop,
        Self::ItemInventory,
        Self::Craft,
        Self::SchoolDungeon,
        Self::Academy,
        Self::Mission,
        Self::MultiFloorRaid,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::None => Some("None"),
            Self::CampaignStage => Some("CampaignStage"),
            Self::EventStage => Some("EventStage"),
            Self::Blood => Some("Blood"),
            Self::WeekDungeon => Some("WeekDungeon"),
            Self::Arena => Some("Arena"),
            Self::Raid => Some("Raid"),
            Self::Shop => Some("Shop"),
            Self::ItemInventory => Some("ItemInventory"),
            Self::Craft => Some("Craft"),
            Self::SchoolDungeon => Some("SchoolDungeon"),
            Self::Academy => Some("Academy"),
            Self::Mission => Some("Mission"),
            Self::MultiFloorRaid => Some("MultiFloorRaid"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for ShortcutContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for ShortcutContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "ShortcutContentType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for ShortcutContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in ShortcutContentType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown ShortcutContentType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for ShortcutContentType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for ShortcutContentType {
    type Output = ShortcutContentType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for ShortcutContentType {
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

impl<'a> ::flatbuffers::Verifiable for ShortcutContentType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for ShortcutContentType {}
