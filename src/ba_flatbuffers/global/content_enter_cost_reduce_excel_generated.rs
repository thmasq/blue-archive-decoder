extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::{ContentType, ParcelType};

#[derive(Copy, Clone, PartialEq)]
pub struct ContentEnterCostReduceExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ContentEnterCostReduceExcel<'a> {
    type Inner = ContentEnterCostReduceExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ContentEnterCostReduceExcel<'a> {
    pub const VT_ENTER_COST_REDUCE_GROUP_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CONTENT_TYPE: ::flatbuffers::VOffsetT = 6;
    pub const VT_STAGE_ID: ::flatbuffers::VOffsetT = 8;
    pub const VT_REDUCE_ENTER_COST_TYPE: ::flatbuffers::VOffsetT = 10;
    pub const VT_REDUCE_ENTER_COST_ID: ::flatbuffers::VOffsetT = 12;
    pub const VT_REDUCE_AMOUNT: ::flatbuffers::VOffsetT = 14;

    #[inline]
    pub fn enter_cost_reduce_group_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ContentEnterCostReduceExcel::VT_ENTER_COST_REDUCE_GROUP_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn content_type(&self) -> ContentType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ContentType>(
                    ContentEnterCostReduceExcel::VT_CONTENT_TYPE,
                    Some(ContentType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn stage_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentEnterCostReduceExcel::VT_STAGE_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reduce_enter_cost_type(&self) -> ParcelType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ParcelType>(
                    ContentEnterCostReduceExcel::VT_REDUCE_ENTER_COST_TYPE,
                    Some(ParcelType::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reduce_enter_cost_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(
                    ContentEnterCostReduceExcel::VT_REDUCE_ENTER_COST_ID,
                    Some(0),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn reduce_amount(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ContentEnterCostReduceExcel::VT_REDUCE_AMOUNT, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ContentEnterCostReduceExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>(
                "enter_cost_reduce_group_id",
                Self::VT_ENTER_COST_REDUCE_GROUP_ID,
                false,
            )?
            .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
            .visit_field::<i64>("stage_id", Self::VT_STAGE_ID, false)?
            .visit_field::<ParcelType>(
                "reduce_enter_cost_type",
                Self::VT_REDUCE_ENTER_COST_TYPE,
                false,
            )?
            .visit_field::<i64>("reduce_enter_cost_id", Self::VT_REDUCE_ENTER_COST_ID, false)?
            .visit_field::<i64>("reduce_amount", Self::VT_REDUCE_AMOUNT, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ContentEnterCostReduceExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContentEnterCostReduceExcel", 6)?;
        s.serialize_field(
            "enter_cost_reduce_group_id",
            &self.enter_cost_reduce_group_id(),
        )?;
        s.serialize_field("content_type", &self.content_type())?;
        s.serialize_field("stage_id", &self.stage_id())?;
        s.serialize_field("reduce_enter_cost_type", &self.reduce_enter_cost_type())?;
        s.serialize_field("reduce_enter_cost_id", &self.reduce_enter_cost_id())?;
        s.serialize_field("reduce_amount", &self.reduce_amount())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ContentEnterCostReduceExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ContentEnterCostReduceExcel");
        ds.field(
            "enter_cost_reduce_group_id",
            &self.enter_cost_reduce_group_id(),
        );
        ds.field("content_type", &self.content_type());
        ds.field("stage_id", &self.stage_id());
        ds.field("reduce_enter_cost_type", &self.reduce_enter_cost_type());
        ds.field("reduce_enter_cost_id", &self.reduce_enter_cost_id());
        ds.field("reduce_amount", &self.reduce_amount());
        ds.finish()
    }
}
