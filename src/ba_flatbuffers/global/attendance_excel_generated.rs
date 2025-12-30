extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{
    AccountState, AttendanceCountRule, AttendanceResetType, AttendanceType, DialogCategory,
    MailType,
};

#[derive(Copy, Clone, PartialEq)]

pub struct AttendanceExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for AttendanceExcel<'a> {
    type Inner = AttendanceExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> AttendanceExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_TYPE_: ::flatbuffers::VOffsetT = 6;
    pub const VT_COUNTDOWN_PREFAB: ::flatbuffers::VOffsetT = 8;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 10;
    pub const VT_ACCOUNT_TYPE: ::flatbuffers::VOffsetT = 12;
    pub const VT_ACCOUNT_LEVEL_LIMIT: ::flatbuffers::VOffsetT = 14;
    pub const VT_TITLE: ::flatbuffers::VOffsetT = 16;
    pub const VT_INFOMATION_LOCALIZE_CODE: ::flatbuffers::VOffsetT = 18;
    pub const VT_COUNT_RULE: ::flatbuffers::VOffsetT = 20;
    pub const VT_COUNT_RESET: ::flatbuffers::VOffsetT = 22;
    pub const VT_BOOK_SIZE: ::flatbuffers::VOffsetT = 24;
    pub const VT_START_DATE: ::flatbuffers::VOffsetT = 26;
    pub const VT_STARTABLE_END_DATE: ::flatbuffers::VOffsetT = 28;
    pub const VT_END_DATE: ::flatbuffers::VOffsetT = 30;
    pub const VT_EXPIRY_DATE: ::flatbuffers::VOffsetT = 32;
    pub const VT_MAIL_TYPE: ::flatbuffers::VOffsetT = 34;
    pub const VT_DIALOG_CATEGORY: ::flatbuffers::VOffsetT = 36;
    pub const VT_TITLE_IMAGE_PATH: ::flatbuffers::VOffsetT = 38;
    pub const VT_DECORATION_IMAGE_PATH: ::flatbuffers::VOffsetT = 40;
    pub const VT_DECORATION_GARLAND_IMAGE_PATH: ::flatbuffers::VOffsetT = 42;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn type_(&self) -> AttendanceType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<AttendanceType>(AttendanceExcel::VT_TYPE_, Some(AttendanceType::Basic))
                .unwrap()
        }
    }
    #[inline]
    pub fn countdown_prefab(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_COUNTDOWN_PREFAB,
                None,
            )
        }
    }
    #[inline]
    pub fn display_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn account_type(&self) -> AccountState {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<AccountState>(
                    AttendanceExcel::VT_ACCOUNT_TYPE,
                    Some(AccountState::WaitingSignIn),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn account_level_limit(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceExcel::VT_ACCOUNT_LEVEL_LIMIT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn title(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(AttendanceExcel::VT_TITLE, None)
        }
    }
    #[inline]
    pub fn infomation_localize_code(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_INFOMATION_LOCALIZE_CODE,
                None,
            )
        }
    }
    #[inline]
    pub fn count_rule(&self) -> AttendanceCountRule {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<AttendanceCountRule>(
                    AttendanceExcel::VT_COUNT_RULE,
                    Some(AttendanceCountRule::Accumulation),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn count_reset(&self) -> AttendanceResetType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<AttendanceResetType>(
                    AttendanceExcel::VT_COUNT_RESET,
                    Some(AttendanceResetType::User),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn book_size(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceExcel::VT_BOOK_SIZE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn start_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(AttendanceExcel::VT_START_DATE, None)
        }
    }
    #[inline]
    pub fn startable_end_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_STARTABLE_END_DATE,
                None,
            )
        }
    }
    #[inline]
    pub fn end_date(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(AttendanceExcel::VT_END_DATE, None)
        }
    }
    #[inline]
    pub fn expiry_date(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(AttendanceExcel::VT_EXPIRY_DATE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn mail_type(&self) -> MailType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<MailType>(AttendanceExcel::VT_MAIL_TYPE, Some(MailType::System))
                .unwrap()
        }
    }
    #[inline]
    pub fn dialog_category(&self) -> DialogCategory {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DialogCategory>(
                    AttendanceExcel::VT_DIALOG_CATEGORY,
                    Some(DialogCategory::Cafe),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn title_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_TITLE_IMAGE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn decoration_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_DECORATION_IMAGE_PATH,
                None,
            )
        }
    }
    #[inline]
    pub fn decoration_garland_image_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                AttendanceExcel::VT_DECORATION_GARLAND_IMAGE_PATH,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for AttendanceExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<AttendanceType>("type_", Self::VT_TYPE_, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "countdown_prefab",
                Self::VT_COUNTDOWN_PREFAB,
                false,
            )?
            .visit_field::<i64>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<AccountState>("account_type", Self::VT_ACCOUNT_TYPE, false)?
            .visit_field::<i64>("account_level_limit", Self::VT_ACCOUNT_LEVEL_LIMIT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("title", Self::VT_TITLE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "infomation_localize_code",
                Self::VT_INFOMATION_LOCALIZE_CODE,
                false,
            )?
            .visit_field::<AttendanceCountRule>("count_rule", Self::VT_COUNT_RULE, false)?
            .visit_field::<AttendanceResetType>("count_reset", Self::VT_COUNT_RESET, false)?
            .visit_field::<i64>("book_size", Self::VT_BOOK_SIZE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "start_date",
                Self::VT_START_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "startable_end_date",
                Self::VT_STARTABLE_END_DATE,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "end_date",
                Self::VT_END_DATE,
                false,
            )?
            .visit_field::<i64>("expiry_date", Self::VT_EXPIRY_DATE, false)?
            .visit_field::<MailType>("mail_type", Self::VT_MAIL_TYPE, false)?
            .visit_field::<DialogCategory>("dialog_category", Self::VT_DIALOG_CATEGORY, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "title_image_path",
                Self::VT_TITLE_IMAGE_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "decoration_image_path",
                Self::VT_DECORATION_IMAGE_PATH,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "decoration_garland_image_path",
                Self::VT_DECORATION_GARLAND_IMAGE_PATH,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for AttendanceExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AttendanceExcel", 20)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("type_", &self.type_())?;
        if let Some(f) = self.countdown_prefab() {
            s.serialize_field("countdown_prefab", &f)?;
        } else {
            s.skip_field("countdown_prefab")?;
        }
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("account_type", &self.account_type())?;
        s.serialize_field("account_level_limit", &self.account_level_limit())?;
        if let Some(f) = self.title() {
            s.serialize_field("title", &f)?;
        } else {
            s.skip_field("title")?;
        }
        if let Some(f) = self.infomation_localize_code() {
            s.serialize_field("infomation_localize_code", &f)?;
        } else {
            s.skip_field("infomation_localize_code")?;
        }
        s.serialize_field("count_rule", &self.count_rule())?;
        s.serialize_field("count_reset", &self.count_reset())?;
        s.serialize_field("book_size", &self.book_size())?;
        if let Some(f) = self.start_date() {
            s.serialize_field("start_date", &f)?;
        } else {
            s.skip_field("start_date")?;
        }
        if let Some(f) = self.startable_end_date() {
            s.serialize_field("startable_end_date", &f)?;
        } else {
            s.skip_field("startable_end_date")?;
        }
        if let Some(f) = self.end_date() {
            s.serialize_field("end_date", &f)?;
        } else {
            s.skip_field("end_date")?;
        }
        s.serialize_field("expiry_date", &self.expiry_date())?;
        s.serialize_field("mail_type", &self.mail_type())?;
        s.serialize_field("dialog_category", &self.dialog_category())?;
        if let Some(f) = self.title_image_path() {
            s.serialize_field("title_image_path", &f)?;
        } else {
            s.skip_field("title_image_path")?;
        }
        if let Some(f) = self.decoration_image_path() {
            s.serialize_field("decoration_image_path", &f)?;
        } else {
            s.skip_field("decoration_image_path")?;
        }
        if let Some(f) = self.decoration_garland_image_path() {
            s.serialize_field("decoration_garland_image_path", &f)?;
        } else {
            s.skip_field("decoration_garland_image_path")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for AttendanceExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("AttendanceExcel");
        ds.field("id", &self.id());
        ds.field("type_", &self.type_());
        ds.field("countdown_prefab", &self.countdown_prefab());
        ds.field("display_order", &self.display_order());
        ds.field("account_type", &self.account_type());
        ds.field("account_level_limit", &self.account_level_limit());
        ds.field("title", &self.title());
        ds.field("infomation_localize_code", &self.infomation_localize_code());
        ds.field("count_rule", &self.count_rule());
        ds.field("count_reset", &self.count_reset());
        ds.field("book_size", &self.book_size());
        ds.field("start_date", &self.start_date());
        ds.field("startable_end_date", &self.startable_end_date());
        ds.field("end_date", &self.end_date());
        ds.field("expiry_date", &self.expiry_date());
        ds.field("mail_type", &self.mail_type());
        ds.field("dialog_category", &self.dialog_category());
        ds.field("title_image_path", &self.title_image_path());
        ds.field("decoration_image_path", &self.decoration_image_path());
        ds.field(
            "decoration_garland_image_path",
            &self.decoration_garland_image_path(),
        );
        ds.finish()
    }
}
