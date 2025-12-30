extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StatLevelUpType(pub i32);
#[allow(non_upper_case_globals)]
impl StatLevelUpType {
    pub const Standard: Self = Self(0);
    pub const Premature: Self = Self(1);
    pub const LateBloom: Self = Self(2);
    pub const Obstacle: Self = Self(3);
    pub const TimeAttack: Self = Self(4);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Standard,
        Self::Premature,
        Self::LateBloom,
        Self::Obstacle,
        Self::TimeAttack,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Standard => Some("Standard"),
            Self::Premature => Some("Premature"),
            Self::LateBloom => Some("LateBloom"),
            Self::Obstacle => Some("Obstacle"),
            Self::TimeAttack => Some("TimeAttack"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for StatLevelUpType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for StatLevelUpType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "StatLevelUpType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for StatLevelUpType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in StatLevelUpType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown StatLevelUpType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for StatLevelUpType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for StatLevelUpType {
    type Output = StatLevelUpType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for StatLevelUpType {
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

impl<'a> ::flatbuffers::Verifiable for StatLevelUpType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for StatLevelUpType {}
