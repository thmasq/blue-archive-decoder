extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EmojiEvent(pub i32);
#[allow(non_upper_case_globals)]
impl EmojiEvent {
    pub const EnterConver: Self = Self(0);
    pub const EnterShelter: Self = Self(1);
    pub const SignalLeader: Self = Self(2);
    pub const Nice: Self = Self(3);
    pub const Reload: Self = Self(4);
    pub const Blind: Self = Self(5);
    pub const Panic: Self = Self(6);
    pub const Silence: Self = Self(7);
    pub const NearyDead: Self = Self(8);
    pub const Run: Self = Self(9);
    pub const TerrainAdaptionS: Self = Self(10);
    pub const TerrainAdaptionA: Self = Self(11);
    pub const TerrainAdaptionB: Self = Self(12);
    pub const TerrainAdaptionC: Self = Self(13);
    pub const TerrainAdaptionD: Self = Self(14);
    pub const TerrainAdaptionSS: Self = Self(15);
    pub const Dot: Self = Self(16);
    pub const Angry: Self = Self(17);
    pub const Bulb: Self = Self(18);
    pub const Exclaim: Self = Self(19);
    pub const Surprise: Self = Self(20);
    pub const Sad: Self = Self(21);
    pub const Sigh: Self = Self(22);
    pub const Steam: Self = Self(23);
    pub const Upset: Self = Self(24);
    pub const Respond: Self = Self(25);
    pub const Question: Self = Self(26);
    pub const Sweat: Self = Self(27);
    pub const Music: Self = Self(28);
    pub const Chat: Self = Self(29);
    pub const Twinkle: Self = Self(30);
    pub const Zzz: Self = Self(31);
    pub const Tear: Self = Self(32);
    pub const Heart: Self = Self(33);
    pub const Shy: Self = Self(34);
    pub const Think: Self = Self(35);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::EnterConver,
        Self::EnterShelter,
        Self::SignalLeader,
        Self::Nice,
        Self::Reload,
        Self::Blind,
        Self::Panic,
        Self::Silence,
        Self::NearyDead,
        Self::Run,
        Self::TerrainAdaptionS,
        Self::TerrainAdaptionA,
        Self::TerrainAdaptionB,
        Self::TerrainAdaptionC,
        Self::TerrainAdaptionD,
        Self::TerrainAdaptionSS,
        Self::Dot,
        Self::Angry,
        Self::Bulb,
        Self::Exclaim,
        Self::Surprise,
        Self::Sad,
        Self::Sigh,
        Self::Steam,
        Self::Upset,
        Self::Respond,
        Self::Question,
        Self::Sweat,
        Self::Music,
        Self::Chat,
        Self::Twinkle,
        Self::Zzz,
        Self::Tear,
        Self::Heart,
        Self::Shy,
        Self::Think,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::EnterConver => Some("EnterConver"),
            Self::EnterShelter => Some("EnterShelter"),
            Self::SignalLeader => Some("SignalLeader"),
            Self::Nice => Some("Nice"),
            Self::Reload => Some("Reload"),
            Self::Blind => Some("Blind"),
            Self::Panic => Some("Panic"),
            Self::Silence => Some("Silence"),
            Self::NearyDead => Some("NearyDead"),
            Self::Run => Some("Run"),
            Self::TerrainAdaptionS => Some("TerrainAdaptionS"),
            Self::TerrainAdaptionA => Some("TerrainAdaptionA"),
            Self::TerrainAdaptionB => Some("TerrainAdaptionB"),
            Self::TerrainAdaptionC => Some("TerrainAdaptionC"),
            Self::TerrainAdaptionD => Some("TerrainAdaptionD"),
            Self::TerrainAdaptionSS => Some("TerrainAdaptionSS"),
            Self::Dot => Some("Dot"),
            Self::Angry => Some("Angry"),
            Self::Bulb => Some("Bulb"),
            Self::Exclaim => Some("Exclaim"),
            Self::Surprise => Some("Surprise"),
            Self::Sad => Some("Sad"),
            Self::Sigh => Some("Sigh"),
            Self::Steam => Some("Steam"),
            Self::Upset => Some("Upset"),
            Self::Respond => Some("Respond"),
            Self::Question => Some("Question"),
            Self::Sweat => Some("Sweat"),
            Self::Music => Some("Music"),
            Self::Chat => Some("Chat"),
            Self::Twinkle => Some("Twinkle"),
            Self::Zzz => Some("Zzz"),
            Self::Tear => Some("Tear"),
            Self::Heart => Some("Heart"),
            Self::Shy => Some("Shy"),
            Self::Think => Some("Think"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for EmojiEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for EmojiEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant("EmojiEvent", self.0 as u32, self.variant_name().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for EmojiEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in EmojiEvent::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown EmojiEvent variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for EmojiEvent {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for EmojiEvent {
    type Output = EmojiEvent;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for EmojiEvent {
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

impl<'a> ::flatbuffers::Verifiable for EmojiEvent {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for EmojiEvent {}
