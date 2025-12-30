extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::MultipleConditionCheckType;

#[derive(Copy, Clone, PartialEq)]
pub struct WorldRaidConditionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for WorldRaidConditionExcel<'a> {
    type Inner = WorldRaidConditionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> WorldRaidConditionExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCK_UI: ::flatbuffers::VOffsetT = 6;
    pub const VT_HIDE_WHEN_LOCKED: ::flatbuffers::VOffsetT = 8;
    pub const VT_ACCOUNT_LEVEL: ::flatbuffers::VOffsetT = 10;
    pub const VT_SCENARIO_MODE_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_CAMPAIGN_STAGE_ID: ::flatbuffers::VOffsetT = 14;
    pub const VT_MULTIPLE_CONDITION_CHECK_TYPE: ::flatbuffers::VOffsetT = 16;
    pub const VT_AFTER_WHEN_DATE: ::flatbuffers::VOffsetT = 18;
    pub const VT_WORLD_RAID_BOSS_KILL: ::flatbuffers::VOffsetT = 20;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(WorldRaidConditionExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn lock_ui(
        &self,
    ) -> Option<::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'a, ::flatbuffers::ForwardsUOffset<&'a str>>,
            >>(WorldRaidConditionExcel::VT_LOCK_UI, None)
        }
    }
    #[inline]
    pub fn hide_when_locked(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(WorldRaidConditionExcel::VT_HIDE_WHEN_LOCKED, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn account_level(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(WorldRaidConditionExcel::VT_ACCOUNT_LEVEL, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scenario_mode_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    WorldRaidConditionExcel::VT_SCENARIO_MODE_ID,
                    None,
                )
        }
    }
    #[inline]
    pub fn campaign_stage_id(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    WorldRaidConditionExcel::VT_CAMPAIGN_STAGE_ID,
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
                    WorldRaidConditionExcel::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                    Some(MultipleConditionCheckType::And),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn after_when_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                WorldRaidConditionExcel::VT_AFTER_WHEN_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn world_raid_boss_kill(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    WorldRaidConditionExcel::VT_WORLD_RAID_BOSS_KILL,
                    None,
                )
        }
    }
}

impl ::flatbuffers::Verifiable for WorldRaidConditionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<
                ::flatbuffers::Vector<'_, ::flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("lock_ui", Self::VT_LOCK_UI, false)?
            .visit_field::<bool>("hide_when_locked", Self::VT_HIDE_WHEN_LOCKED, false)?
            .visit_field::<i64>("account_level", Self::VT_ACCOUNT_LEVEL, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "scenario_mode_id",
                Self::VT_SCENARIO_MODE_ID,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "campaign_stage_id",
                Self::VT_CAMPAIGN_STAGE_ID,
                false,
            )?
            .visit_field::<MultipleConditionCheckType>(
                "multiple_condition_check_type",
                Self::VT_MULTIPLE_CONDITION_CHECK_TYPE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "after_when_date",
                Self::VT_AFTER_WHEN_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "world_raid_boss_kill",
                Self::VT_WORLD_RAID_BOSS_KILL,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for WorldRaidConditionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("WorldRaidConditionExcel", 9)?;
        s.serialize_field("id", &self.id())?;
        if let Some(f) = self.lock_ui() {
            s.serialize_field("lock_ui", &f)?;
        } else {
            s.skip_field("lock_ui")?;
        }
        s.serialize_field("hide_when_locked", &self.hide_when_locked())?;
        s.serialize_field("account_level", &self.account_level())?;
        if let Some(f) = self.scenario_mode_id() {
            s.serialize_field("scenario_mode_id", &f)?;
        } else {
            s.skip_field("scenario_mode_id")?;
        }
        if let Some(f) = self.campaign_stage_id() {
            s.serialize_field("campaign_stage_id", &f)?;
        } else {
            s.skip_field("campaign_stage_id")?;
        }
        s.serialize_field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        )?;
        if let Some(f) = self.after_when_date() {
            s.serialize_field("after_when_date", &f)?;
        } else {
            s.skip_field("after_when_date")?;
        }
        if let Some(f) = self.world_raid_boss_kill() {
            s.serialize_field("world_raid_boss_kill", &f)?;
        } else {
            s.skip_field("world_raid_boss_kill")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for WorldRaidConditionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("WorldRaidConditionExcel");
        ds.field("id", &self.id());
        ds.field("lock_ui", &self.lock_ui());
        ds.field("hide_when_locked", &self.hide_when_locked());
        ds.field("account_level", &self.account_level());
        ds.field("scenario_mode_id", &self.scenario_mode_id());
        ds.field("campaign_stage_id", &self.campaign_stage_id());
        ds.field(
            "multiple_condition_check_type",
            &self.multiple_condition_check_type(),
        );
        ds.field("after_when_date", &self.after_when_date());
        ds.field("world_raid_boss_kill", &self.world_raid_boss_kill());
        ds.finish()
    }
}
