extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{CollectionUnlockType, MultipleConditionCheckType};

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioContentCollectionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioContentCollectionExcel<'a> {
    type Inner = ScenarioContentCollectionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioContentCollectionExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_GROUP_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_UNLOCK_CONDITION_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_UNLOCK_CONDITION_PARAMETER: ::flatbuffers::VOffsetT = 10;
    pub const VT_MULTIPLE_CONDITION_CHECK_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_UNLOCK_CONDITION_COUNT: ::flatbuffers::VOffsetT = 14;
    pub const VT_IS_OBJECT: ::flatbuffers::VOffsetT = 16;
    pub const VT_IS_HORIZON: ::flatbuffers::VOffsetT = 18;
    pub const VT_EMBLEM_RESOURCE: ::flatbuffers::VOffsetT = 20;
    pub const VT_THUMB_RESOURCE: ::flatbuffers::VOffsetT = 22;
    pub const VT_FULL_RESOURCE: ::flatbuffers::VOffsetT = 24;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 26;
    pub const VT_SUB_NAME_LOCALIZE_CODE_ID: ::flatbuffers::VOffsetT = 28;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioContentCollectionExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioContentCollectionExcel::VT_GROUP_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_condition_type(&self) -> CollectionUnlockType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CollectionUnlockType>(
                    ScenarioContentCollectionExcel::VT_UNLOCK_CONDITION_TYPE,
                    Some(CollectionUnlockType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_condition_parameter(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    ScenarioContentCollectionExcel::VT_UNLOCK_CONDITION_PARAMETER,
                    None,
                )
        }
    }
    #[inline]
    pub fn multiple_condition_check_type(&self) -> MultipleConditionCheckType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MultipleConditionCheckType>(
                    ScenarioContentCollectionExcel::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                    Some(MultipleConditionCheckType::And),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn unlock_condition_count(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ScenarioContentCollectionExcel::VT_UNLOCK_CONDITION_COUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn is_object(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioContentCollectionExcel::VT_IS_OBJECT, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_horizon(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ScenarioContentCollectionExcel::VT_IS_HORIZON, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn emblem_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioContentCollectionExcel::VT_EMBLEM_RESOURCE,
                None,
            )
        }
    }
    #[inline]
    pub fn thumb_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioContentCollectionExcel::VT_THUMB_RESOURCE,
                None,
            )
        }
    }
    #[inline]
    pub fn full_resource(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioContentCollectionExcel::VT_FULL_RESOURCE,
                None,
            )
        }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioContentCollectionExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sub_name_localize_code_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioContentCollectionExcel::VT_SUB_NAME_LOCALIZE_CODE_ID,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioContentCollectionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("group_id", Self::VT_GROUP_ID, false)?
            .visit_field::<CollectionUnlockType>(
                "unlock_condition_type",
                Self::VT_UNLOCK_CONDITION_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "unlock_condition_parameter",
                Self::VT_UNLOCK_CONDITION_PARAMETER,
                false,
            )?
            .visit_field::<MultipleConditionCheckType>(
                "multiple_condition_check_type",
                Self::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "unlock_condition_count",
                Self::VT_UNLOCK_CONDITION_COUNT,
                false,
            )?
            .visit_field::<bool>("is_object", Self::VT_IS_OBJECT, false)?
            .visit_field::<bool>("is_horizon", Self::VT_IS_HORIZON, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "emblem_resource",
                Self::VT_EMBLEM_RESOURCE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "thumb_resource",
                Self::VT_THUMB_RESOURCE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "full_resource",
                Self::VT_FULL_RESOURCE,
                false,
            )?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sub_name_localize_code_id",
                Self::VT_SUB_NAME_LOCALIZE_CODE_ID,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioContentCollectionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioContentCollectionExcel", 13)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("group_id", &self.group_id())?;
        s.serialize_field("unlock_condition_type", &self.unlock_condition_type())?;
        if let Some(f) = self.unlock_condition_parameter() {
            s.serialize_field("unlock_condition_parameter", &f)?;
        } else {
            s.skip_field("unlock_condition_parameter")?;
        }
        s.serialize_field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        )?;
        s.serialize_field("unlock_condition_count", &self.unlock_condition_count())?;
        s.serialize_field("is_object", &self.is_object())?;
        s.serialize_field("is_horizon", &self.is_horizon())?;
        if let Some(f) = self.emblem_resource() {
            s.serialize_field("emblem_resource", &f)?;
        } else {
            s.skip_field("emblem_resource")?;
        }
        if let Some(f) = self.thumb_resource() {
            s.serialize_field("thumb_resource", &f)?;
        } else {
            s.skip_field("thumb_resource")?;
        }
        if let Some(f) = self.full_resource() {
            s.serialize_field("full_resource", &f)?;
        } else {
            s.skip_field("full_resource")?;
        }
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.sub_name_localize_code_id() {
            s.serialize_field("sub_name_localize_code_id", &f)?;
        } else {
            s.skip_field("sub_name_localize_code_id")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioContentCollectionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioContentCollectionExcel");
        ds.field("id", &self.id());
        ds.field("group_id", &self.group_id());
        ds.field("unlock_condition_type", &self.unlock_condition_type());
        ds.field(
            "unlock_condition_parameter",
            &self.unlock_condition_parameter(),
        );
        ds.field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        );
        ds.field("unlock_condition_count", &self.unlock_condition_count());
        ds.field("is_object", &self.is_object());
        ds.field("is_horizon", &self.is_horizon());
        ds.field("emblem_resource", &self.emblem_resource());
        ds.field("thumb_resource", &self.thumb_resource());
        ds.field("full_resource", &self.full_resource());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field(
            "sub_name_localize_code_id",
            &self.sub_name_localize_code_id(),
        );
        ds.finish()
    }
}
