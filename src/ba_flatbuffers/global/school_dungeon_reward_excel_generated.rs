extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ParcelType, RewardTag, SchoolDungeonType};

#[derive(Copy, Clone, PartialEq)]
pub struct SchoolDungeonRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for SchoolDungeonRewardExcel<'a> {
    type Inner = SchoolDungeonRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> SchoolDungeonRewardExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DUNGEON_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_TAG: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 14;
    pub const VT_REWARD_PARCEL_PROBABILITY: ::flatbuffers::VOffsetT = 16;
    pub const VT_IS_DISPLAYED: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonRewardExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn dungeon_type(&self) -> SchoolDungeonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<SchoolDungeonType>(
                    SchoolDungeonRewardExcel::VT_DUNGEON_TYPE,
                    Some(SchoolDungeonType::SchoolA),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_tag(&self) -> RewardTag {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<RewardTag>(
                    SchoolDungeonRewardExcel::VT_REWARD_TAG,
                    Some(RewardTag::Default),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    SchoolDungeonRewardExcel::VT_REWARD_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonRewardExcel::VT_REWARD_PARCEL_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(SchoolDungeonRewardExcel::VT_REWARD_PARCEL_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_parcel_probability(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    SchoolDungeonRewardExcel::VT_REWARD_PARCEL_PROBABILITY,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn is_displayed(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(SchoolDungeonRewardExcel::VT_IS_DISPLAYED, Some(false))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for SchoolDungeonRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<SchoolDungeonType>("dungeon_type", Self::VT_DUNGEON_TYPE, false)?
            .visit_field::<RewardTag>("reward_tag", Self::VT_REWARD_TAG, false)?
            .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
            .visit_field::<i64>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
            .visit_field::<i64>("reward_parcel_amount", Self::VT_REWARD_PARCEL_AMOUNT, false)?
            .visit_field::<i64>(
                "reward_parcel_probability",
                Self::VT_REWARD_PARCEL_PROBABILITY,
                false,
            )?
            .visit_field::<bool>("is_displayed", Self::VT_IS_DISPLAYED, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for SchoolDungeonRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SchoolDungeonRewardExcel", 8)?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("dungeon_type", &self.dungeon_type())?;
        s.serialize_field("reward_tag", &self.reward_tag())?;
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_parcel_id", &self.reward_parcel_id())?;
        s.serialize_field("reward_parcel_amount", &self.reward_parcel_amount())?;
        s.serialize_field(
            "reward_parcel_probability",
            &self.reward_parcel_probability(),
        )?;
        s.serialize_field("is_displayed", &self.is_displayed())?;
        s.end()
    }
}

impl ::core::fmt::Debug for SchoolDungeonRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("SchoolDungeonRewardExcel");
        ds.field("group_id", &self.group_id());
        ds.field("dungeon_type", &self.dungeon_type());
        ds.field("reward_tag", &self.reward_tag());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.field(
            "reward_parcel_probability",
            &self.reward_parcel_probability(),
        );
        ds.field("is_displayed", &self.is_displayed());
        ds.finish()
    }
}
