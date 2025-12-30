extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::EchelonType;

#[derive(Copy, Clone, PartialEq)]
pub struct ClanAssistSlotExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ClanAssistSlotExcel<'a> {
    type Inner = ClanAssistSlotExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ClanAssistSlotExcel<'a> {
    pub const VT_SLOT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_ECHELON_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_SLOT_NUMBER: ::flatbuffers::VOffsetT = 8;
    pub const VT_ASSIST_TERM_REWARD_PERIOD_FROM_SEC: ::flatbuffers::VOffsetT = 10;
    pub const VT_ASSIST_REWARD_LIMIT: ::flatbuffers::VOffsetT = 12;
    pub const VT_ASSIST_RENT_REWARD_DAILY_MAX_COUNT: ::flatbuffers::VOffsetT = 14;
    pub const VT_ASSIST_RENTAL_FEE_AMOUNT: ::flatbuffers::VOffsetT = 16;
    pub const VT_ASSIST_RENTAL_FEE_AMOUNT_STRANGER: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn slot_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanAssistSlotExcel::VT_SLOT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn echelon_type(&self) -> EchelonType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<EchelonType>(
                    ClanAssistSlotExcel::VT_ECHELON_TYPE,
                    Some(EchelonType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn slot_number(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanAssistSlotExcel::VT_SLOT_NUMBER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn assist_term_reward_period_from_sec(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ClanAssistSlotExcel::VT_ASSIST_TERM_REWARD_PERIOD_FROM_SEC,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn assist_reward_limit(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanAssistSlotExcel::VT_ASSIST_REWARD_LIMIT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn assist_rent_reward_daily_max_count(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ClanAssistSlotExcel::VT_ASSIST_RENT_REWARD_DAILY_MAX_COUNT,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn assist_rental_fee_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ClanAssistSlotExcel::VT_ASSIST_RENTAL_FEE_AMOUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn assist_rental_fee_amount_stranger(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ClanAssistSlotExcel::VT_ASSIST_RENTAL_FEE_AMOUNT_STRANGER,
                    Some(0),
                )
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ClanAssistSlotExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("slot_id", Self::VT_SLOT_ID, false)?
            .visit_field::<EchelonType>("echelon_type", Self::VT_ECHELON_TYPE, false)?
            .visit_field::<i64>("slot_number", Self::VT_SLOT_NUMBER, false)?
            .visit_field::<i64>(
                "assist_term_reward_period_from_sec",
                Self::VT_ASSIST_TERM_REWARD_PERIOD_FROM_SEC,
                false,
            )?
            .visit_field::<i64>("assist_reward_limit", Self::VT_ASSIST_REWARD_LIMIT, false)?
            .visit_field::<i64>(
                "assist_rent_reward_daily_max_count",
                Self::VT_ASSIST_RENT_REWARD_DAILY_MAX_COUNT,
                false,
            )?
            .visit_field::<i64>(
                "assist_rental_fee_amount",
                Self::VT_ASSIST_RENTAL_FEE_AMOUNT,
                false,
            )?
            .visit_field::<i64>(
                "assist_rental_fee_amount_stranger",
                Self::VT_ASSIST_RENTAL_FEE_AMOUNT_STRANGER,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for ClanAssistSlotExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ClanAssistSlotExcel", 8)?;
        s.serialize_field("slot_id", &self.slot_id())?;
        s.serialize_field("echelon_type", &self.echelon_type())?;
        s.serialize_field("slot_number", &self.slot_number())?;
        s.serialize_field(
            "assist_term_reward_period_from_sec",
            &self.assist_term_reward_period_from_sec(),
        )?;
        s.serialize_field("assist_reward_limit", &self.assist_reward_limit())?;
        s.serialize_field(
            "assist_rent_reward_daily_max_count",
            &self.assist_rent_reward_daily_max_count(),
        )?;
        s.serialize_field("assist_rental_fee_amount", &self.assist_rental_fee_amount())?;
        s.serialize_field(
            "assist_rental_fee_amount_stranger",
            &self.assist_rental_fee_amount_stranger(),
        )?;
        s.end()
    }
}

impl ::core::fmt::Debug for ClanAssistSlotExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ClanAssistSlotExcel");
        ds.field("slot_id", &self.slot_id());
        ds.field("echelon_type", &self.echelon_type());
        ds.field("slot_number", &self.slot_number());
        ds.field(
            "assist_term_reward_period_from_sec",
            &self.assist_term_reward_period_from_sec(),
        );
        ds.field("assist_reward_limit", &self.assist_reward_limit());
        ds.field(
            "assist_rent_reward_daily_max_count",
            &self.assist_rent_reward_daily_max_count(),
        );
        ds.field("assist_rental_fee_amount", &self.assist_rental_fee_amount());
        ds.field(
            "assist_rental_fee_amount_stranger",
            &self.assist_rental_fee_amount_stranger(),
        );
        ds.finish()
    }
}
