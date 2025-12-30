extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PotentialStatBonusRateType(pub i32);
#[allow(non_upper_case_globals)]
impl PotentialStatBonusRateType {
    pub const None: Self = Self(0);
    pub const MaxHP: Self = Self(1);
    pub const AttackPower: Self = Self(2);
    pub const HealPower: Self = Self(3);

    pub const ENUM_VALUES: &'static [Self] =
        &[Self::None, Self::MaxHP, Self::AttackPower, Self::HealPower];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::None => Some("None"),
            Self::MaxHP => Some("MaxHP"),
            Self::AttackPower => Some("AttackPower"),
            Self::HealPower => Some("HealPower"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for PotentialStatBonusRateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for PotentialStatBonusRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "PotentialStatBonusRateType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for PotentialStatBonusRateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in PotentialStatBonusRateType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown PotentialStatBonusRateType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for PotentialStatBonusRateType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for PotentialStatBonusRateType {
    type Output = PotentialStatBonusRateType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for PotentialStatBonusRateType {
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

impl<'a> ::flatbuffers::Verifiable for PotentialStatBonusRateType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for PotentialStatBonusRateType {}
