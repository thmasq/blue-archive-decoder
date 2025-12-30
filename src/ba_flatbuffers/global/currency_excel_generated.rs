extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{
    CurrencyAdditionalChargeType, CurrencyOverChargeType, CurrencyTypes, DailyRefillType,
    ParcelType, Rarity,
};

#[derive(Copy, Clone, PartialEq)]
pub struct CurrencyExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for CurrencyExcel<'a> {
    type Inner = CurrencyExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> CurrencyExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_CURRENCY_TYPE: ::flatbuffers::VOffsetT = 8;
    pub const VT_CURRENCY_NAME: ::flatbuffers::VOffsetT = 10;
    pub const VT_ICON: ::flatbuffers::VOffsetT = 12;
    pub const VT_RARITY: ::flatbuffers::VOffsetT = 14;
    pub const VT_AUTO_CHARGE_MSC: ::flatbuffers::VOffsetT = 16;
    pub const VT_AUTO_CHARGE_AMOUNT: ::flatbuffers::VOffsetT = 18;
    pub const VT_CURRENCY_OVER_CHARGE_TYPE: ::flatbuffers::VOffsetT = 20;
    pub const VT_CURRENCY_ADDITIONAL_CHARGE_TYPE: ::flatbuffers::VOffsetT = 22;
    pub const VT_CHARGE_LIMIT: ::flatbuffers::VOffsetT = 24;
    pub const VT_OVER_CHARGE_LIMIT: ::flatbuffers::VOffsetT = 26;
    pub const VT_SPRITE_NAME: ::flatbuffers::VOffsetT = 28;
    pub const VT_DAILY_REFILL_TYPE: ::flatbuffers::VOffsetT = 30;
    pub const VT_DAILY_REFILL_AMOUNT: ::flatbuffers::VOffsetT = 32;
    pub const VT_DAILY_REFILL_TIME: ::flatbuffers::VOffsetT = 34;
    pub const VT_EXPIRATION_DATE_TIME: ::flatbuffers::VOffsetT = 36;
    pub const VT_EXPIRATION_NOTIFY_DATE_IN: ::flatbuffers::VOffsetT = 38;
    pub const VT_EXPIRY_CHANGE_PARCEL_TYPE: ::flatbuffers::VOffsetT = 40;
    pub const VT_EXPIRY_CHANGE_ID: ::flatbuffers::VOffsetT = 42;
    pub const VT_EXPIRY_CHANGE_AMOUNT: ::flatbuffers::VOffsetT = 44;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(CurrencyExcel::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn localize_etc_id(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(CurrencyExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn currency_type(&self) -> CurrencyTypes {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CurrencyTypes>(
                    CurrencyExcel::VT_CURRENCY_TYPE,
                    Some(CurrencyTypes::Invalid),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn currency_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(CurrencyExcel::VT_CURRENCY_NAME, None)
        }
    }
    #[inline]
    pub fn icon(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(CurrencyExcel::VT_ICON, None)
        }
    }
    #[inline]
    pub fn rarity(&self) -> Rarity {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Rarity>(CurrencyExcel::VT_RARITY, Some(Rarity::N))
                .unwrap()
        }
    }
    #[inline]
    pub fn auto_charge_msc(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CurrencyExcel::VT_AUTO_CHARGE_MSC, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn auto_charge_amount(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CurrencyExcel::VT_AUTO_CHARGE_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn currency_over_charge_type(&self) -> CurrencyOverChargeType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CurrencyOverChargeType>(
                    CurrencyExcel::VT_CURRENCY_OVER_CHARGE_TYPE,
                    Some(CurrencyOverChargeType::CanNotCharge),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn currency_additional_charge_type(&self) -> CurrencyAdditionalChargeType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<CurrencyAdditionalChargeType>(
                    CurrencyExcel::VT_CURRENCY_ADDITIONAL_CHARGE_TYPE,
                    Some(CurrencyAdditionalChargeType::EnableAutoChargeOverLimit),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn charge_limit(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CurrencyExcel::VT_CHARGE_LIMIT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn over_charge_limit(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CurrencyExcel::VT_OVER_CHARGE_LIMIT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn sprite_name(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(CurrencyExcel::VT_SPRITE_NAME, None)
        }
    }
    #[inline]
    pub fn daily_refill_type(&self) -> DailyRefillType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<DailyRefillType>(
                    CurrencyExcel::VT_DAILY_REFILL_TYPE,
                    Some(DailyRefillType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn daily_refill_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CurrencyExcel::VT_DAILY_REFILL_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn daily_refill_time(&self) -> Option<::flatbuffers::Vector<'a, i64>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'a, i64>>>(
                    CurrencyExcel::VT_DAILY_REFILL_TIME,
                    None,
                )
        }
    }
    #[inline]
    pub fn expiration_date_time(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                CurrencyExcel::VT_EXPIRATION_DATE_TIME,
                None,
            )
        }
    }
    #[inline]
    pub fn expiration_notify_date_in(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(CurrencyExcel::VT_EXPIRATION_NOTIFY_DATE_IN, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn expiry_change_parcel_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    CurrencyExcel::VT_EXPIRY_CHANGE_PARCEL_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn expiry_change_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CurrencyExcel::VT_EXPIRY_CHANGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn expiry_change_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(CurrencyExcel::VT_EXPIRY_CHANGE_AMOUNT, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for CurrencyExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<CurrencyTypes>("currency_type", Self::VT_CURRENCY_TYPE, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "currency_name",
                Self::VT_CURRENCY_NAME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("icon", Self::VT_ICON, false)?
            .visit_field::<Rarity>("rarity", Self::VT_RARITY, false)?
            .visit_field::<i32>("auto_charge_msc", Self::VT_AUTO_CHARGE_MSC, false)?
            .visit_field::<i32>("auto_charge_amount", Self::VT_AUTO_CHARGE_AMOUNT, false)?
            .visit_field::<CurrencyOverChargeType>(
                "currency_over_charge_type",
                Self::VT_CURRENCY_OVER_CHARGE_TYPE,
                false,
            )?
            .visit_field::<CurrencyAdditionalChargeType>(
                "currency_additional_charge_type",
                Self::VT_CURRENCY_ADDITIONAL_CHARGE_TYPE,
                false,
            )?
            .visit_field::<i64>("charge_limit", Self::VT_CHARGE_LIMIT, false)?
            .visit_field::<i64>("over_charge_limit", Self::VT_OVER_CHARGE_LIMIT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "sprite_name",
                Self::VT_SPRITE_NAME,
                false,
            )?
            .visit_field::<DailyRefillType>("daily_refill_type", Self::VT_DAILY_REFILL_TYPE, false)?
            .visit_field::<i64>("daily_refill_amount", Self::VT_DAILY_REFILL_AMOUNT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<::flatbuffers::Vector<'_, i64>>>(
                "daily_refill_time",
                Self::VT_DAILY_REFILL_TIME,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "expiration_date_time",
                Self::VT_EXPIRATION_DATE_TIME,
                false,
            )?
            .visit_field::<i32>(
                "expiration_notify_date_in",
                Self::VT_EXPIRATION_NOTIFY_DATE_IN,
                false,
            )?
            .visit_field::<ParcelType>(
                "expiry_change_parcel_type",
                Self::VT_EXPIRY_CHANGE_PARCEL_TYPE,
                false,
            )?
            .visit_field::<i64>("expiry_change_id", Self::VT_EXPIRY_CHANGE_ID, false)?
            .visit_field::<i64>("expiry_change_amount", Self::VT_EXPIRY_CHANGE_AMOUNT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for CurrencyExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CurrencyExcel", 21)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        s.serialize_field("currency_type", &self.currency_type())?;
        if let Some(f) = self.currency_name() {
            s.serialize_field("currency_name", &f)?;
        } else {
            s.skip_field("currency_name")?;
        }
        if let Some(f) = self.icon() {
            s.serialize_field("icon", &f)?;
        } else {
            s.skip_field("icon")?;
        }
        s.serialize_field("rarity", &self.rarity())?;
        s.serialize_field("auto_charge_msc", &self.auto_charge_msc())?;
        s.serialize_field("auto_charge_amount", &self.auto_charge_amount())?;
        s.serialize_field(
            "currency_over_charge_type",
            &self.currency_over_charge_type(),
        )?;
        s.serialize_field(
            "currency_additional_charge_type",
            &self.currency_additional_charge_type(),
        )?;
        s.serialize_field("charge_limit", &self.charge_limit())?;
        s.serialize_field("over_charge_limit", &self.over_charge_limit())?;
        if let Some(f) = self.sprite_name() {
            s.serialize_field("sprite_name", &f)?;
        } else {
            s.skip_field("sprite_name")?;
        }
        s.serialize_field("daily_refill_type", &self.daily_refill_type())?;
        s.serialize_field("daily_refill_amount", &self.daily_refill_amount())?;
        if let Some(f) = self.daily_refill_time() {
            s.serialize_field("daily_refill_time", &f)?;
        } else {
            s.skip_field("daily_refill_time")?;
        }
        if let Some(f) = self.expiration_date_time() {
            s.serialize_field("expiration_date_time", &f)?;
        } else {
            s.skip_field("expiration_date_time")?;
        }
        s.serialize_field(
            "expiration_notify_date_in",
            &self.expiration_notify_date_in(),
        )?;
        s.serialize_field(
            "expiry_change_parcel_type",
            &self.expiry_change_parcel_type(),
        )?;
        s.serialize_field("expiry_change_id", &self.expiry_change_id())?;
        s.serialize_field("expiry_change_amount", &self.expiry_change_amount())?;
        s.end()
    }
}

impl ::core::fmt::Debug for CurrencyExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("CurrencyExcel");
        ds.field("id", &self.id());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("currency_type", &self.currency_type());
        ds.field("currency_name", &self.currency_name());
        ds.field("icon", &self.icon());
        ds.field("rarity", &self.rarity());
        ds.field("auto_charge_msc", &self.auto_charge_msc());
        ds.field("auto_charge_amount", &self.auto_charge_amount());
        ds.field(
            "currency_over_charge_type",
            &self.currency_over_charge_type(),
        );
        ds.field(
            "currency_additional_charge_type",
            &self.currency_additional_charge_type(),
        );
        ds.field("charge_limit", &self.charge_limit());
        ds.field("over_charge_limit", &self.over_charge_limit());
        ds.field("sprite_name", &self.sprite_name());
        ds.field("daily_refill_type", &self.daily_refill_type());
        ds.field("daily_refill_amount", &self.daily_refill_amount());
        ds.field("daily_refill_time", &self.daily_refill_time());
        ds.field("expiration_date_time", &self.expiration_date_time());
        ds.field(
            "expiration_notify_date_in",
            &self.expiration_notify_date_in(),
        );
        ds.field(
            "expiry_change_parcel_type",
            &self.expiry_change_parcel_type(),
        );
        ds.field("expiry_change_id", &self.expiry_change_id());
        ds.field("expiry_change_amount", &self.expiry_change_amount());
        ds.finish()
    }
}
