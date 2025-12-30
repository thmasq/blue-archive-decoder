extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ServiceActionType;

#[derive(Copy, Clone, PartialEq)]
pub struct ServiceActionExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ServiceActionExcel<'a> {
    type Inner = ServiceActionExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ServiceActionExcel<'a> {
    pub const VT_SERVICE_ACTION_TYPE: ::flatbuffers::VOffsetT = 4;
    pub const VT_IS_LEGACY: ::flatbuffers::VOffsetT = 6;
    pub const VT_GOODS_ID: ::flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn service_action_type(&self) -> ServiceActionType {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ServiceActionType>(
                    ServiceActionExcel::VT_SERVICE_ACTION_TYPE,
                    Some(ServiceActionType::ClanCreate),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn is_legacy(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(ServiceActionExcel::VT_IS_LEGACY, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn goods_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ServiceActionExcel::VT_GOODS_ID, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ServiceActionExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<ServiceActionType>(
                "service_action_type",
                Self::VT_SERVICE_ACTION_TYPE,
                false,
            )?
            .visit_field::<bool>("is_legacy", Self::VT_IS_LEGACY, false)?
            .visit_field::<i64>("goods_id", Self::VT_GOODS_ID, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ServiceActionExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ServiceActionExcel", 3)?;
        s.serialize_field("service_action_type", &self.service_action_type())?;
        s.serialize_field("is_legacy", &self.is_legacy())?;
        s.serialize_field("goods_id", &self.goods_id())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ServiceActionExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ServiceActionExcel");
        ds.field("service_action_type", &self.service_action_type());
        ds.field("is_legacy", &self.is_legacy());
        ds.field("goods_id", &self.goods_id());
        ds.finish()
    }
}
