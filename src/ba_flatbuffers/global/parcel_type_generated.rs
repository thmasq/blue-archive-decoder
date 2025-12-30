extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ParcelType(pub i32);
#[allow(non_upper_case_globals)]
impl ParcelType {
    pub const None: Self = Self(0);
    pub const Character: Self = Self(1);
    pub const Currency: Self = Self(2);
    pub const Equipment: Self = Self(3);
    pub const Item: Self = Self(4);
    pub const GachaGroup: Self = Self(5);
    pub const Product: Self = Self(6);
    pub const Shop: Self = Self(7);
    pub const MemoryLobby: Self = Self(8);
    pub const AccountExp: Self = Self(9);
    pub const CharacterExp: Self = Self(10);
    pub const FavorExp: Self = Self(11);
    pub const TSS: Self = Self(12);
    pub const Furniture: Self = Self(13);
    pub const ShopRefresh: Self = Self(14);
    pub const LocationExp: Self = Self(15);
    pub const Recipe: Self = Self(16);
    pub const CharacterWeapon: Self = Self(17);
    pub const ProductMonthly: Self = Self(18);
    pub const CharacterGear: Self = Self(19);
    pub const IdCardBackground: Self = Self(20);
    pub const Emblem: Self = Self(21);
    pub const Sticker: Self = Self(22);
    pub const Costume: Self = Self(23);
    pub const PossessionCheck: Self = Self(24);
    pub const BattlePassExp: Self = Self(25);
    pub const SelectedCharacter: Self = Self(26);
    pub const UnSelectedCharacter: Self = Self(27);
    pub const ProductBattlePass: Self = Self(28);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::None,
        Self::Character,
        Self::Currency,
        Self::Equipment,
        Self::Item,
        Self::GachaGroup,
        Self::Product,
        Self::Shop,
        Self::MemoryLobby,
        Self::AccountExp,
        Self::CharacterExp,
        Self::FavorExp,
        Self::TSS,
        Self::Furniture,
        Self::ShopRefresh,
        Self::LocationExp,
        Self::Recipe,
        Self::CharacterWeapon,
        Self::ProductMonthly,
        Self::CharacterGear,
        Self::IdCardBackground,
        Self::Emblem,
        Self::Sticker,
        Self::Costume,
        Self::PossessionCheck,
        Self::BattlePassExp,
        Self::SelectedCharacter,
        Self::UnSelectedCharacter,
        Self::ProductBattlePass,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::None => Some("None"),
            Self::Character => Some("Character"),
            Self::Currency => Some("Currency"),
            Self::Equipment => Some("Equipment"),
            Self::Item => Some("Item"),
            Self::GachaGroup => Some("GachaGroup"),
            Self::Product => Some("Product"),
            Self::Shop => Some("Shop"),
            Self::MemoryLobby => Some("MemoryLobby"),
            Self::AccountExp => Some("AccountExp"),
            Self::CharacterExp => Some("CharacterExp"),
            Self::FavorExp => Some("FavorExp"),
            Self::TSS => Some("TSS"),
            Self::Furniture => Some("Furniture"),
            Self::ShopRefresh => Some("ShopRefresh"),
            Self::LocationExp => Some("LocationExp"),
            Self::Recipe => Some("Recipe"),
            Self::CharacterWeapon => Some("CharacterWeapon"),
            Self::ProductMonthly => Some("ProductMonthly"),
            Self::CharacterGear => Some("CharacterGear"),
            Self::IdCardBackground => Some("IdCardBackground"),
            Self::Emblem => Some("Emblem"),
            Self::Sticker => Some("Sticker"),
            Self::Costume => Some("Costume"),
            Self::PossessionCheck => Some("PossessionCheck"),
            Self::BattlePassExp => Some("BattlePassExp"),
            Self::SelectedCharacter => Some("SelectedCharacter"),
            Self::UnSelectedCharacter => Some("UnSelectedCharacter"),
            Self::ProductBattlePass => Some("ProductBattlePass"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for ParcelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for ParcelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant("ParcelType", self.0 as u32, self.variant_name().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for ParcelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in ParcelType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown ParcelType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for ParcelType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for ParcelType {
    type Output = ParcelType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for ParcelType {
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

impl<'a> ::flatbuffers::Verifiable for ParcelType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for ParcelType {}
