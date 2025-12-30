extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EquipmentOptionType(pub i32);
#[allow(non_upper_case_globals)]
impl EquipmentOptionType {
    pub const None: Self = Self(0);
    pub const MaxHP_Base: Self = Self(1);
    pub const MaxHP_Coefficient: Self = Self(2);
    pub const AttackPower_Base: Self = Self(3);
    pub const AttackPower_Coefficient: Self = Self(4);
    pub const DefensePower_Base: Self = Self(5);
    pub const DefensePower_Coefficient: Self = Self(6);
    pub const HealPower_Base: Self = Self(7);
    pub const HealPower_Coefficient: Self = Self(8);
    pub const CriticalPoint_Base: Self = Self(9);
    pub const CriticalPoint_Coefficient: Self = Self(10);
    pub const CriticalChanceRate_Base: Self = Self(11);
    pub const CriticalDamageRate_Base: Self = Self(12);
    pub const CriticalDamageRate_Coefficient: Self = Self(13);
    pub const SightRange_Base: Self = Self(14);
    pub const SightRange_Coefficient: Self = Self(15);
    pub const MaxBulletCount_Base: Self = Self(16);
    pub const MaxBulletCount_Coefficient: Self = Self(17);
    pub const HPRecoverOnKill_Base: Self = Self(18);
    pub const HPRecoverOnKill_Coefficient: Self = Self(19);
    pub const StreetBattleAdaptation_Base: Self = Self(20);
    pub const OutdoorBattleAdaptation_Base: Self = Self(21);
    pub const IndoorBattleAdaptation_Base: Self = Self(22);
    pub const HealEffectivenessRate_Base: Self = Self(23);
    pub const HealEffectivenessRate_Coefficient: Self = Self(24);
    pub const CriticalChanceResistPoint_Base: Self = Self(25);
    pub const CriticalChanceResistPoint_Coefficient: Self = Self(26);
    pub const CriticalDamageResistRate_Base: Self = Self(27);
    pub const CriticalDamageResistRate_Coefficient: Self = Self(28);
    pub const ExSkillUpgrade: Self = Self(29);
    pub const OppressionPower_Base: Self = Self(30);
    pub const OppressionPower_Coefficient: Self = Self(31);
    pub const OppressionResist_Base: Self = Self(32);
    pub const OppressionResist_Coefficient: Self = Self(33);
    pub const StabilityPoint_Base: Self = Self(34);
    pub const StabilityPoint_Coefficient: Self = Self(35);
    pub const AccuracyPoint_Base: Self = Self(36);
    pub const AccuracyPoint_Coefficient: Self = Self(37);
    pub const DodgePoint_Base: Self = Self(38);
    pub const DodgePoint_Coefficient: Self = Self(39);
    pub const MoveSpeed_Base: Self = Self(40);
    pub const MoveSpeed_Coefficient: Self = Self(41);
    pub const Max: Self = Self(42);
    pub const NormalAttackSpeed_Base: Self = Self(43);
    pub const NormalAttackSpeed_Coefficient: Self = Self(44);
    pub const DefensePenetration_Base: Self = Self(45);
    pub const DefensePenetrationResisit_Base: Self = Self(46);
    pub const ExtendBuffDuration_Base: Self = Self(47);
    pub const ExtendDebuffDuration_Base: Self = Self(48);
    pub const ExtendCrowdControlDuration_Base: Self = Self(49);
    pub const EnhanceExplosionRate_Base: Self = Self(50);
    pub const EnhanceExplosionRate_Coefficient: Self = Self(51);
    pub const EnhancePierceRate_Base: Self = Self(52);
    pub const EnhancePierceRate_Coefficient: Self = Self(53);
    pub const EnhanceMysticRate_Base: Self = Self(54);
    pub const EnhanceMysticRate_Coefficient: Self = Self(55);
    pub const EnhanceLightArmorRate_Base: Self = Self(56);
    pub const EnhanceLightArmorRate_Coefficient: Self = Self(57);
    pub const EnhanceHeavyArmorRate_Base: Self = Self(58);
    pub const EnhanceHeavyArmorRate_Coefficient: Self = Self(59);
    pub const EnhanceUnarmedRate_Base: Self = Self(60);
    pub const EnhanceUnarmedRate_Coefficient: Self = Self(61);
    pub const EnhanceSiegeRate_Base: Self = Self(62);
    pub const EnhanceSiegeRate_Coefficient: Self = Self(63);
    pub const EnhanceNormalRate_Base: Self = Self(64);
    pub const EnhanceNormalRate_Coefficient: Self = Self(65);
    pub const EnhanceStructureRate_Base: Self = Self(66);
    pub const EnhanceStructureRate_Coefficient: Self = Self(67);
    pub const EnhanceNormalArmorRate_Base: Self = Self(68);
    pub const EnhanceNormalArmorRate_Coefficient: Self = Self(69);
    pub const DamageRatio2Increase_Base: Self = Self(70);
    pub const DamageRatio2Increase_Coefficient: Self = Self(71);
    pub const DamageRatio2Decrease_Base: Self = Self(72);
    pub const DamageRatio2Decrease_Coefficient: Self = Self(73);
    pub const DamagedRatio2Increase_Base: Self = Self(74);
    pub const DamagedRatio2Increase_Coefficient: Self = Self(75);
    pub const DamagedRatio2Decrease_Base: Self = Self(76);
    pub const DamagedRatio2Decrease_Coefficient: Self = Self(77);
    pub const EnhanceSonicRate_Base: Self = Self(78);
    pub const EnhanceSonicRate_Coefficient: Self = Self(79);
    pub const EnhanceElasticArmorRate_Base: Self = Self(80);
    pub const EnhanceElasticArmorRate_Coefficient: Self = Self(81);
    pub const IgnoreDelayCount_Base: Self = Self(82);
    pub const WeaponRange_Base: Self = Self(83);
    pub const BlockRate_Base: Self = Self(84);
    pub const BlockRate_Coefficient: Self = Self(85);
    pub const AmmoCost_Base: Self = Self(86);
    pub const RegenCost_Base: Self = Self(87);
    pub const RegenCost_Coefficient: Self = Self(88);
    pub const MaxCostIncrease_Base: Self = Self(89);

    pub const ENUM_VALUES: &'static [Self] = &[
        Self::None,
        Self::MaxHP_Base,
        Self::MaxHP_Coefficient,
        Self::AttackPower_Base,
        Self::AttackPower_Coefficient,
        Self::DefensePower_Base,
        Self::DefensePower_Coefficient,
        Self::HealPower_Base,
        Self::HealPower_Coefficient,
        Self::CriticalPoint_Base,
        Self::CriticalPoint_Coefficient,
        Self::CriticalChanceRate_Base,
        Self::CriticalDamageRate_Base,
        Self::CriticalDamageRate_Coefficient,
        Self::SightRange_Base,
        Self::SightRange_Coefficient,
        Self::MaxBulletCount_Base,
        Self::MaxBulletCount_Coefficient,
        Self::HPRecoverOnKill_Base,
        Self::HPRecoverOnKill_Coefficient,
        Self::StreetBattleAdaptation_Base,
        Self::OutdoorBattleAdaptation_Base,
        Self::IndoorBattleAdaptation_Base,
        Self::HealEffectivenessRate_Base,
        Self::HealEffectivenessRate_Coefficient,
        Self::CriticalChanceResistPoint_Base,
        Self::CriticalChanceResistPoint_Coefficient,
        Self::CriticalDamageResistRate_Base,
        Self::CriticalDamageResistRate_Coefficient,
        Self::ExSkillUpgrade,
        Self::OppressionPower_Base,
        Self::OppressionPower_Coefficient,
        Self::OppressionResist_Base,
        Self::OppressionResist_Coefficient,
        Self::StabilityPoint_Base,
        Self::StabilityPoint_Coefficient,
        Self::AccuracyPoint_Base,
        Self::AccuracyPoint_Coefficient,
        Self::DodgePoint_Base,
        Self::DodgePoint_Coefficient,
        Self::MoveSpeed_Base,
        Self::MoveSpeed_Coefficient,
        Self::Max,
        Self::NormalAttackSpeed_Base,
        Self::NormalAttackSpeed_Coefficient,
        Self::DefensePenetration_Base,
        Self::DefensePenetrationResisit_Base,
        Self::ExtendBuffDuration_Base,
        Self::ExtendDebuffDuration_Base,
        Self::ExtendCrowdControlDuration_Base,
        Self::EnhanceExplosionRate_Base,
        Self::EnhanceExplosionRate_Coefficient,
        Self::EnhancePierceRate_Base,
        Self::EnhancePierceRate_Coefficient,
        Self::EnhanceMysticRate_Base,
        Self::EnhanceMysticRate_Coefficient,
        Self::EnhanceLightArmorRate_Base,
        Self::EnhanceLightArmorRate_Coefficient,
        Self::EnhanceHeavyArmorRate_Base,
        Self::EnhanceHeavyArmorRate_Coefficient,
        Self::EnhanceUnarmedRate_Base,
        Self::EnhanceUnarmedRate_Coefficient,
        Self::EnhanceSiegeRate_Base,
        Self::EnhanceSiegeRate_Coefficient,
        Self::EnhanceNormalRate_Base,
        Self::EnhanceNormalRate_Coefficient,
        Self::EnhanceStructureRate_Base,
        Self::EnhanceStructureRate_Coefficient,
        Self::EnhanceNormalArmorRate_Base,
        Self::EnhanceNormalArmorRate_Coefficient,
        Self::DamageRatio2Increase_Base,
        Self::DamageRatio2Increase_Coefficient,
        Self::DamageRatio2Decrease_Base,
        Self::DamageRatio2Decrease_Coefficient,
        Self::DamagedRatio2Increase_Base,
        Self::DamagedRatio2Increase_Coefficient,
        Self::DamagedRatio2Decrease_Base,
        Self::DamagedRatio2Decrease_Coefficient,
        Self::EnhanceSonicRate_Base,
        Self::EnhanceSonicRate_Coefficient,
        Self::EnhanceElasticArmorRate_Base,
        Self::EnhanceElasticArmorRate_Coefficient,
        Self::IgnoreDelayCount_Base,
        Self::WeaponRange_Base,
        Self::BlockRate_Base,
        Self::BlockRate_Coefficient,
        Self::AmmoCost_Base,
        Self::RegenCost_Base,
        Self::RegenCost_Coefficient,
        Self::MaxCostIncrease_Base,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::None => Some("None"),
            Self::MaxHP_Base => Some("MaxHP_Base"),
            Self::MaxHP_Coefficient => Some("MaxHP_Coefficient"),
            Self::AttackPower_Base => Some("AttackPower_Base"),
            Self::AttackPower_Coefficient => Some("AttackPower_Coefficient"),
            Self::DefensePower_Base => Some("DefensePower_Base"),
            Self::DefensePower_Coefficient => Some("DefensePower_Coefficient"),
            Self::HealPower_Base => Some("HealPower_Base"),
            Self::HealPower_Coefficient => Some("HealPower_Coefficient"),
            Self::CriticalPoint_Base => Some("CriticalPoint_Base"),
            Self::CriticalPoint_Coefficient => Some("CriticalPoint_Coefficient"),
            Self::CriticalChanceRate_Base => Some("CriticalChanceRate_Base"),
            Self::CriticalDamageRate_Base => Some("CriticalDamageRate_Base"),
            Self::CriticalDamageRate_Coefficient => Some("CriticalDamageRate_Coefficient"),
            Self::SightRange_Base => Some("SightRange_Base"),
            Self::SightRange_Coefficient => Some("SightRange_Coefficient"),
            Self::MaxBulletCount_Base => Some("MaxBulletCount_Base"),
            Self::MaxBulletCount_Coefficient => Some("MaxBulletCount_Coefficient"),
            Self::HPRecoverOnKill_Base => Some("HPRecoverOnKill_Base"),
            Self::HPRecoverOnKill_Coefficient => Some("HPRecoverOnKill_Coefficient"),
            Self::StreetBattleAdaptation_Base => Some("StreetBattleAdaptation_Base"),
            Self::OutdoorBattleAdaptation_Base => Some("OutdoorBattleAdaptation_Base"),
            Self::IndoorBattleAdaptation_Base => Some("IndoorBattleAdaptation_Base"),
            Self::HealEffectivenessRate_Base => Some("HealEffectivenessRate_Base"),
            Self::HealEffectivenessRate_Coefficient => Some("HealEffectivenessRate_Coefficient"),
            Self::CriticalChanceResistPoint_Base => Some("CriticalChanceResistPoint_Base"),
            Self::CriticalChanceResistPoint_Coefficient => {
                Some("CriticalChanceResistPoint_Coefficient")
            }
            Self::CriticalDamageResistRate_Base => Some("CriticalDamageResistRate_Base"),
            Self::CriticalDamageResistRate_Coefficient => {
                Some("CriticalDamageResistRate_Coefficient")
            }
            Self::ExSkillUpgrade => Some("ExSkillUpgrade"),
            Self::OppressionPower_Base => Some("OppressionPower_Base"),
            Self::OppressionPower_Coefficient => Some("OppressionPower_Coefficient"),
            Self::OppressionResist_Base => Some("OppressionResist_Base"),
            Self::OppressionResist_Coefficient => Some("OppressionResist_Coefficient"),
            Self::StabilityPoint_Base => Some("StabilityPoint_Base"),
            Self::StabilityPoint_Coefficient => Some("StabilityPoint_Coefficient"),
            Self::AccuracyPoint_Base => Some("AccuracyPoint_Base"),
            Self::AccuracyPoint_Coefficient => Some("AccuracyPoint_Coefficient"),
            Self::DodgePoint_Base => Some("DodgePoint_Base"),
            Self::DodgePoint_Coefficient => Some("DodgePoint_Coefficient"),
            Self::MoveSpeed_Base => Some("MoveSpeed_Base"),
            Self::MoveSpeed_Coefficient => Some("MoveSpeed_Coefficient"),
            Self::Max => Some("Max"),
            Self::NormalAttackSpeed_Base => Some("NormalAttackSpeed_Base"),
            Self::NormalAttackSpeed_Coefficient => Some("NormalAttackSpeed_Coefficient"),
            Self::DefensePenetration_Base => Some("DefensePenetration_Base"),
            Self::DefensePenetrationResisit_Base => Some("DefensePenetrationResisit_Base"),
            Self::ExtendBuffDuration_Base => Some("ExtendBuffDuration_Base"),
            Self::ExtendDebuffDuration_Base => Some("ExtendDebuffDuration_Base"),
            Self::ExtendCrowdControlDuration_Base => Some("ExtendCrowdControlDuration_Base"),
            Self::EnhanceExplosionRate_Base => Some("EnhanceExplosionRate_Base"),
            Self::EnhanceExplosionRate_Coefficient => Some("EnhanceExplosionRate_Coefficient"),
            Self::EnhancePierceRate_Base => Some("EnhancePierceRate_Base"),
            Self::EnhancePierceRate_Coefficient => Some("EnhancePierceRate_Coefficient"),
            Self::EnhanceMysticRate_Base => Some("EnhanceMysticRate_Base"),
            Self::EnhanceMysticRate_Coefficient => Some("EnhanceMysticRate_Coefficient"),
            Self::EnhanceLightArmorRate_Base => Some("EnhanceLightArmorRate_Base"),
            Self::EnhanceLightArmorRate_Coefficient => Some("EnhanceLightArmorRate_Coefficient"),
            Self::EnhanceHeavyArmorRate_Base => Some("EnhanceHeavyArmorRate_Base"),
            Self::EnhanceHeavyArmorRate_Coefficient => Some("EnhanceHeavyArmorRate_Coefficient"),
            Self::EnhanceUnarmedRate_Base => Some("EnhanceUnarmedRate_Base"),
            Self::EnhanceUnarmedRate_Coefficient => Some("EnhanceUnarmedRate_Coefficient"),
            Self::EnhanceSiegeRate_Base => Some("EnhanceSiegeRate_Base"),
            Self::EnhanceSiegeRate_Coefficient => Some("EnhanceSiegeRate_Coefficient"),
            Self::EnhanceNormalRate_Base => Some("EnhanceNormalRate_Base"),
            Self::EnhanceNormalRate_Coefficient => Some("EnhanceNormalRate_Coefficient"),
            Self::EnhanceStructureRate_Base => Some("EnhanceStructureRate_Base"),
            Self::EnhanceStructureRate_Coefficient => Some("EnhanceStructureRate_Coefficient"),
            Self::EnhanceNormalArmorRate_Base => Some("EnhanceNormalArmorRate_Base"),
            Self::EnhanceNormalArmorRate_Coefficient => Some("EnhanceNormalArmorRate_Coefficient"),
            Self::DamageRatio2Increase_Base => Some("DamageRatio2Increase_Base"),
            Self::DamageRatio2Increase_Coefficient => Some("DamageRatio2Increase_Coefficient"),
            Self::DamageRatio2Decrease_Base => Some("DamageRatio2Decrease_Base"),
            Self::DamageRatio2Decrease_Coefficient => Some("DamageRatio2Decrease_Coefficient"),
            Self::DamagedRatio2Increase_Base => Some("DamagedRatio2Increase_Base"),
            Self::DamagedRatio2Increase_Coefficient => Some("DamagedRatio2Increase_Coefficient"),
            Self::DamagedRatio2Decrease_Base => Some("DamagedRatio2Decrease_Base"),
            Self::DamagedRatio2Decrease_Coefficient => Some("DamagedRatio2Decrease_Coefficient"),
            Self::EnhanceSonicRate_Base => Some("EnhanceSonicRate_Base"),
            Self::EnhanceSonicRate_Coefficient => Some("EnhanceSonicRate_Coefficient"),
            Self::EnhanceElasticArmorRate_Base => Some("EnhanceElasticArmorRate_Base"),
            Self::EnhanceElasticArmorRate_Coefficient => {
                Some("EnhanceElasticArmorRate_Coefficient")
            }
            Self::IgnoreDelayCount_Base => Some("IgnoreDelayCount_Base"),
            Self::WeaponRange_Base => Some("WeaponRange_Base"),
            Self::BlockRate_Base => Some("BlockRate_Base"),
            Self::BlockRate_Coefficient => Some("BlockRate_Coefficient"),
            Self::AmmoCost_Base => Some("AmmoCost_Base"),
            Self::RegenCost_Base => Some("RegenCost_Base"),
            Self::RegenCost_Coefficient => Some("RegenCost_Coefficient"),
            Self::MaxCostIncrease_Base => Some("MaxCostIncrease_Base"),
            _ => None,
        }
    }
}
impl ::core::fmt::Debug for EquipmentOptionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl Serialize for EquipmentOptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(
            "EquipmentOptionType",
            self.0 as u32,
            self.variant_name().unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for EquipmentOptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        for item in EquipmentOptionType::ENUM_VALUES {
            if let Some(item_name) = item.variant_name() {
                if item_name == s {
                    return Ok(item.clone());
                }
            }
        }
        Err(serde::de::Error::custom(format!(
            "Unknown EquipmentOptionType variant: {s}"
        )))
    }
}

impl<'a> ::flatbuffers::Follow<'a> for EquipmentOptionType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { ::flatbuffers::read_scalar_at::<i32>(buf, loc) };
        Self(b)
    }
}

impl ::flatbuffers::Push for EquipmentOptionType {
    type Output = EquipmentOptionType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { ::flatbuffers::emplace_scalar::<i32>(dst, self.0) };
    }
}

impl ::flatbuffers::EndianScalar for EquipmentOptionType {
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

impl<'a> ::flatbuffers::Verifiable for EquipmentOptionType {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        i32::run_verifier(v, pos)
    }
}

impl ::flatbuffers::SimpleToVerifyInSlice for EquipmentOptionType {}
