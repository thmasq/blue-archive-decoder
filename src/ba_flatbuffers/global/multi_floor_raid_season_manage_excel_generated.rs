extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MultiFloorRaidSeasonManageExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MultiFloorRaidSeasonManageExcel<'a> {
    type Inner = MultiFloorRaidSeasonManageExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MultiFloorRaidSeasonManageExcel<'a> {
    pub const VT_SEASON_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOBBY_ENTER_SCENARIO: ::flatbuffers::VOffsetT = 6;
    pub const VT_SHOW_LOBBY_BANNER: ::flatbuffers::VOffsetT = 8;
    pub const VT_SEASON_START_DATE: ::flatbuffers::VOffsetT = 10;
    pub const VT_END_NOTE_LABEL_START_DATE: ::flatbuffers::VOffsetT = 12;
    pub const VT_SEASON_END_DATE: ::flatbuffers::VOffsetT = 14;
    pub const VT_SETTLEMENT_END_DATE: ::flatbuffers::VOffsetT = 16;
    pub const VT_OPEN_RAID_BOSS_GROUP_ID: ::flatbuffers::VOffsetT = 18;
    pub const VT_ENTER_SCENARIO_KEY: ::flatbuffers::VOffsetT = 20;
    pub const VT_LOBBY_IMG_PATH: ::flatbuffers::VOffsetT = 22;
    pub const VT_LEVEL_IMG_PATH: ::flatbuffers::VOffsetT = 24;
    pub const VT_PLAY_TIP: ::flatbuffers::VOffsetT = 26;

    #[inline]
    pub fn season_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MultiFloorRaidSeasonManageExcel::VT_SEASON_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn lobby_enter_scenario(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MultiFloorRaidSeasonManageExcel::VT_LOBBY_ENTER_SCENARIO,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn show_lobby_banner(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(
                    MultiFloorRaidSeasonManageExcel::VT_SHOW_LOBBY_BANNER,
                    Some(false),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn season_start_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_SEASON_START_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn end_note_label_start_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_END_NOTE_LABEL_START_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn season_end_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_SEASON_END_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn settlement_end_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_SETTLEMENT_END_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn open_raid_boss_group_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_OPEN_RAID_BOSS_GROUP_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn enter_scenario_key(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(
                    MultiFloorRaidSeasonManageExcel::VT_ENTER_SCENARIO_KEY,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn lobby_img_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_LOBBY_IMG_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn level_img_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_LEVEL_IMG_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn play_tip(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MultiFloorRaidSeasonManageExcel::VT_PLAY_TIP,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for MultiFloorRaidSeasonManageExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("season_id", Self::VT_SEASON_ID, false)?
            .visit_field::<u32>("lobby_enter_scenario", Self::VT_LOBBY_ENTER_SCENARIO, false)?
            .visit_field::<bool>("show_lobby_banner", Self::VT_SHOW_LOBBY_BANNER, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "season_start_date",
                Self::VT_SEASON_START_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "end_note_label_start_date",
                Self::VT_END_NOTE_LABEL_START_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "season_end_date",
                Self::VT_SEASON_END_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "settlement_end_date",
                Self::VT_SETTLEMENT_END_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "open_raid_boss_group_id",
                Self::VT_OPEN_RAID_BOSS_GROUP_ID,
                false,
            )?
            .visit_field::<u32>("enter_scenario_key", Self::VT_ENTER_SCENARIO_KEY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "lobby_img_path",
                Self::VT_LOBBY_IMG_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "level_img_path",
                Self::VT_LEVEL_IMG_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "play_tip",
                Self::VT_PLAY_TIP,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MultiFloorRaidSeasonManageExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MultiFloorRaidSeasonManageExcel", 12)?;
        s.serialize_field("season_id", &self.season_id())?;
        s.serialize_field("lobby_enter_scenario", &self.lobby_enter_scenario())?;
        s.serialize_field("show_lobby_banner", &self.show_lobby_banner())?;
        if let Some(f) = self.season_start_date() {
            s.serialize_field("season_start_date", &f)?;
        } else {
            s.skip_field("season_start_date")?;
        }
        if let Some(f) = self.end_note_label_start_date() {
            s.serialize_field("end_note_label_start_date", &f)?;
        } else {
            s.skip_field("end_note_label_start_date")?;
        }
        if let Some(f) = self.season_end_date() {
            s.serialize_field("season_end_date", &f)?;
        } else {
            s.skip_field("season_end_date")?;
        }
        if let Some(f) = self.settlement_end_date() {
            s.serialize_field("settlement_end_date", &f)?;
        } else {
            s.skip_field("settlement_end_date")?;
        }
        if let Some(f) = self.open_raid_boss_group_id() {
            s.serialize_field("open_raid_boss_group_id", &f)?;
        } else {
            s.skip_field("open_raid_boss_group_id")?;
        }
        s.serialize_field("enter_scenario_key", &self.enter_scenario_key())?;
        if let Some(f) = self.lobby_img_path() {
            s.serialize_field("lobby_img_path", &f)?;
        } else {
            s.skip_field("lobby_img_path")?;
        }
        if let Some(f) = self.level_img_path() {
            s.serialize_field("level_img_path", &f)?;
        } else {
            s.skip_field("level_img_path")?;
        }
        if let Some(f) = self.play_tip() {
            s.serialize_field("play_tip", &f)?;
        } else {
            s.skip_field("play_tip")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MultiFloorRaidSeasonManageExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MultiFloorRaidSeasonManageExcel");
        ds.field("season_id", &self.season_id());
        ds.field("lobby_enter_scenario", &self.lobby_enter_scenario());
        ds.field("show_lobby_banner", &self.show_lobby_banner());
        ds.field("season_start_date", &self.season_start_date());
        ds.field(
            "end_note_label_start_date",
            &self.end_note_label_start_date(),
        );
        ds.field("season_end_date", &self.season_end_date());
        ds.field("settlement_end_date", &self.settlement_end_date());
        ds.field("open_raid_boss_group_id", &self.open_raid_boss_group_id());
        ds.field("enter_scenario_key", &self.enter_scenario_key());
        ds.field("lobby_img_path", &self.lobby_img_path());
        ds.field("level_img_path", &self.level_img_path());
        ds.field("play_tip", &self.play_tip());
        ds.finish()
    }
}
