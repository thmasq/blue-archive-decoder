extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct GroundModuleRewardExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for GroundModuleRewardExcel<'a> {
    type Inner = GroundModuleRewardExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> GroundModuleRewardExcel<'a> {
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_REWARD_PARCEL_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_REWARD_PARCEL_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_PARCEL_AMOUNT: ::flatbuffers::VOffsetT = 10;
    pub const VT_REWARD_PARCEL_PROBABILITY: ::flatbuffers::VOffsetT = 12;
    pub const VT_IS_DISPLAYED: ::flatbuffers::VOffsetT = 14;
    pub const VT_DROP_ITEM_MODEL_PREFAB_PATH: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn group_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(GroundModuleRewardExcel::VT_GROUP_ID, Some(0))
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
                    GroundModuleRewardExcel::VT_REWARD_PARCEL_TYPE,
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
                .get::<i64>(GroundModuleRewardExcel::VT_REWARD_PARCEL_ID, Some(0))
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
                .get::<i64>(GroundModuleRewardExcel::VT_REWARD_PARCEL_AMOUNT, Some(0))
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
                    GroundModuleRewardExcel::VT_REWARD_PARCEL_PROBABILITY,
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
                .get::<bool>(GroundModuleRewardExcel::VT_IS_DISPLAYED, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn drop_item_model_prefab_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                GroundModuleRewardExcel::VT_DROP_ITEM_MODEL_PREFAB_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for GroundModuleRewardExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<ParcelType>("reward_parcel_type", Self::VT_REWARD_PARCEL_TYPE, false)?
            .visit_field::<i64>("reward_parcel_id", Self::VT_REWARD_PARCEL_ID, false)?
            .visit_field::<i64>("reward_parcel_amount", Self::VT_REWARD_PARCEL_AMOUNT, false)?
            .visit_field::<i64>(
                "reward_parcel_probability",
                Self::VT_REWARD_PARCEL_PROBABILITY,
                false,
            )?
            .visit_field::<bool>("is_displayed", Self::VT_IS_DISPLAYED, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "drop_item_model_prefab_path",
                Self::VT_DROP_ITEM_MODEL_PREFAB_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for GroundModuleRewardExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("GroundModuleRewardExcel", 7)?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("reward_parcel_type", &self.reward_parcel_type())?;
        s.serialize_field("reward_parcel_id", &self.reward_parcel_id())?;
        s.serialize_field("reward_parcel_amount", &self.reward_parcel_amount())?;
        s.serialize_field(
            "reward_parcel_probability",
            &self.reward_parcel_probability(),
        )?;
        s.serialize_field("is_displayed", &self.is_displayed())?;
        if let Some(f) = self.drop_item_model_prefab_path() {
            s.serialize_field("drop_item_model_prefab_path", &f)?;
        } else {
            s.skip_field("drop_item_model_prefab_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for GroundModuleRewardExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("GroundModuleRewardExcel");
        ds.field("group_id", &self.group_id());
        ds.field("reward_parcel_type", &self.reward_parcel_type());
        ds.field("reward_parcel_id", &self.reward_parcel_id());
        ds.field("reward_parcel_amount", &self.reward_parcel_amount());
        ds.field(
            "reward_parcel_probability",
            &self.reward_parcel_probability(),
        );
        ds.field("is_displayed", &self.is_displayed());
        ds.field(
            "drop_item_model_prefab_path",
            &self.drop_item_model_prefab_path(),
        );
        ds.finish()
    }
}
