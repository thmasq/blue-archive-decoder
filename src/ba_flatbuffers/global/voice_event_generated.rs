extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VoiceEvent(pub i32);
#[allow(non_upper_case_globals)]
impl VoiceEvent {
    pub const OnTSA: Self = Self(0);
    pub const FormationPickUp: Self = Self(1);
    pub const CampaignResultDefeat: Self = Self(2);
    pub const CampaignResultVictory: Self = Self(3);
    pub const CharacterLevelUp: Self = Self(4);
    pub const CharacterTranscendence: Self = Self(5);
    pub const SkillLevelUp: Self = Self(6);
    pub const Formation: Self = Self(7);
    pub const CampaignCharacterSpawn: Self = Self(8);
    pub const BattleStartTimeline: Self = Self(9);
    pub const BattleVictoryTimeline: Self = Self(10);
    pub const CharacterFavor: Self = Self(11);
    pub const BattleMiss: Self = Self(12);
    pub const BattleBlock: Self = Self(13);
    pub const BattleCover: Self = Self(14);
    pub const BattleMove: Self = Self(15);
    pub const BattleMoveToForamtionBeacon: Self = Self(16);
    pub const MGS_GameStart: Self = Self(17);
    pub const MGS_CharacterSelect: Self = Self(18);
    pub const MGS_Attacking: Self = Self(19);
    pub const MGS_GeasGet: Self = Self(20);
    pub const EXSkill: Self = Self(21);
    pub const EXSkillLevel: Self = Self(22);
    pub const EXSkill2: Self = Self(23);
    pub const EXSkillLevel2: Self = Self(24);
    pub const EXSkill3: Self = Self(25);
    pub const EXSkillLevel3: Self = Self(26);
    pub const EXSkill4: Self = Self(27);
    pub const EXSkillLevel4: Self = Self(28);
    pub const PublicSkill01: Self = Self(29);
    pub const PublicSkill02: Self = Self(30);
    pub const InteractionPublicSkill01: Self = Self(31);
    pub const InteractionPublicSkill02: Self = Self(32);
    pub const FormationStyleChange: Self = Self(33);
    pub const BattleInteractionVictoryTimeline: Self = Self(34);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::OnTSA,
        Self::FormationPickUp,
        Self::CampaignResultDefeat,
        Self::CampaignResultVictory,
        Self::CharacterLevelUp,
        Self::CharacterTranscendence,
        Self::SkillLevelUp,
        Self::Formation,
        Self::CampaignCharacterSpawn,
        Self::BattleStartTimeline,
        Self::BattleVictoryTimeline,
        Self::CharacterFavor,
        Self::BattleMiss,
        Self::BattleBlock,
        Self::BattleCover,
        Self::BattleMove,
        Self::BattleMoveToForamtionBeacon,
        Self::MGS_GameStart,
        Self::MGS_CharacterSelect,
        Self::MGS_Attacking,
        Self::MGS_GeasGet,
        Self::EXSkill,
        Self::EXSkillLevel,
        Self::EXSkill2,
        Self::EXSkillLevel2,
        Self::EXSkill3,
        Self::EXSkillLevel3,
        Self::EXSkill4,
        Self::EXSkillLevel4,
        Self::PublicSkill01,
        Self::PublicSkill02,
        Self::InteractionPublicSkill01,
        Self::InteractionPublicSkill02,
        Self::FormationStyleChange,
        Self::BattleInteractionVictoryTimeline,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::OnTSA => Some("OnTSA"),
            Self::FormationPickUp => Some("FormationPickUp"),
            Self::CampaignResultDefeat => Some("CampaignResultDefeat"),
            Self::CampaignResultVictory => Some("CampaignResultVictory"),
            Self::CharacterLevelUp => Some("CharacterLevelUp"),
            Self::CharacterTranscendence => Some("CharacterTranscendence"),
            Self::SkillLevelUp => Some("SkillLevelUp"),
            Self::Formation => Some("Formation"),
            Self::CampaignCharacterSpawn => Some("CampaignCharacterSpawn"),
            Self::BattleStartTimeline => Some("BattleStartTimeline"),
            Self::BattleVictoryTimeline => Some("BattleVictoryTimeline"),
            Self::CharacterFavor => Some("CharacterFavor"),
            Self::BattleMiss => Some("BattleMiss"),
            Self::BattleBlock => Some("BattleBlock"),
            Self::BattleCover => Some("BattleCover"),
            Self::BattleMove => Some("BattleMove"),
            Self::BattleMoveToForamtionBeacon => Some("BattleMoveToForamtionBeacon"),
            Self::MGS_GameStart => Some("MGS_GameStart"),
            Self::MGS_CharacterSelect => Some("MGS_CharacterSelect"),
            Self::MGS_Attacking => Some("MGS_Attacking"),
            Self::MGS_GeasGet => Some("MGS_GeasGet"),
            Self::EXSkill => Some("EXSkill"),
            Self::EXSkillLevel => Some("EXSkillLevel"),
            Self::EXSkill2 => Some("EXSkill2"),
            Self::EXSkillLevel2 => Some("EXSkillLevel2"),
            Self::EXSkill3 => Some("EXSkill3"),
            Self::EXSkillLevel3 => Some("EXSkillLevel3"),
            Self::EXSkill4 => Some("EXSkill4"),
            Self::EXSkillLevel4 => Some("EXSkillLevel4"),
            Self::PublicSkill01 => Some("PublicSkill01"),
            Self::PublicSkill02 => Some("PublicSkill02"),
            Self::InteractionPublicSkill01 => Some("InteractionPublicSkill01"),
            Self::InteractionPublicSkill02 => Some("InteractionPublicSkill02"),
            Self::FormationStyleChange => Some("FormationStyleChange"),
            Self::BattleInteractionVictoryTimeline => Some("BattleInteractionVictoryTimeline"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for VoiceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for VoiceEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant("VoiceEvent", self.0 as u32, self.variant_name().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for VoiceEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in VoiceEvent::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown VoiceEvent variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for VoiceEvent {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for VoiceEvent {
    type Output = VoiceEvent;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for VoiceEvent {
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

impl<'a> ::flatbuffers::Verifiable for VoiceEvent {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for VoiceEvent {}
