extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ParcelType;

#[derive(Copy, Clone, PartialEq)]
pub struct MiniGameDefenseInfoExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MiniGameDefenseInfoExcel<'a> {
    type Inner = MiniGameDefenseInfoExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MiniGameDefenseInfoExcel<'a> {
    pub const VT_EVENT_CONTENT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_DEFENSE_BATTLE_PARCEL_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_DEFENSE_BATTLE_PARCEL_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_DEFENSE_BATTLE_MULTIPLIER_MAX: ::flatbuffers::VOffsetT = 10;
    pub const VT_DISABLE_ROOT_MOTION: ::flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn event_content_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MiniGameDefenseInfoExcel::VT_EVENT_CONTENT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn defense_battle_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    MiniGameDefenseInfoExcel::VT_DEFENSE_BATTLE_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn defense_battle_parcel_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseInfoExcel::VT_DEFENSE_BATTLE_PARCEL_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn defense_battle_multiplier_max(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    MiniGameDefenseInfoExcel::VT_DEFENSE_BATTLE_MULTIPLIER_MAX,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn disable_root_motion(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MiniGameDefenseInfoExcel::VT_DISABLE_ROOT_MOTION,
                    Some(false),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for MiniGameDefenseInfoExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("event_content_id", Self::VT_EVENT_CONTENT_ID, false)?
            .visit_field::<ParcelType>(
                "defense_battle_parcel_type",
                Self::VT_DEFENSE_BATTLE_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>(
                "defense_battle_parcel_id",
                Self::VT_DEFENSE_BATTLE_PARCEL_ID,
                false,
            )?
            .visit_field::<i64>(
                "defense_battle_multiplier_max",
                Self::VT_DEFENSE_BATTLE_MULTIPLIER_MAX,
                false,
            )?
            .visit_field::<bool>("disable_root_motion", Self::VT_DISABLE_ROOT_MOTION, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for MiniGameDefenseInfoExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MiniGameDefenseInfoExcel", 5)?;
        s.serialize_field("event_content_id", &self.event_content_id())?;
        s.serialize_field(
            "defense_battle_parcel_type",
            &self.defense_battle_parcel_type(),
        )?;
        s.serialize_field("defense_battle_parcel_id", &self.defense_battle_parcel_id())?;
        s.serialize_field(
            "defense_battle_multiplier_max",
            &self.defense_battle_multiplier_max(),
        )?;
        s.serialize_field("disable_root_motion", &self.disable_root_motion())?;
        s.end()
    }
}

impl ::core::fmt::Debug for MiniGameDefenseInfoExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MiniGameDefenseInfoExcel");
        ds.field("event_content_id", &self.event_content_id());
        ds.field(
            "defense_battle_parcel_type",
            &self.defense_battle_parcel_type(),
        );
        ds.field("defense_battle_parcel_id", &self.defense_battle_parcel_id());
        ds.field(
            "defense_battle_multiplier_max",
            &self.defense_battle_multiplier_max(),
        );
        ds.field("disable_root_motion", &self.disable_root_motion());
        ds.finish()
    }
}
