extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::Rarity;

#[derive(Copy, Clone, PartialEq)]
pub struct IdCardBackgroundExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for IdCardBackgroundExcel<'a> {
    type Inner = IdCardBackgroundExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> IdCardBackgroundExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_RARITY: ::flatbuffers::VOffsetT = 6;
    pub const VT_DISPLAY_ORDER: ::flatbuffers::VOffsetT = 8;
    pub const VT_COLLECTION_VISIBLE: ::flatbuffers::VOffsetT = 10;
    pub const VT_IS_DEFAULT: ::flatbuffers::VOffsetT = 12;
    pub const VT_BG_PATH: ::flatbuffers::VOffsetT = 14;
    pub const VT_LOCALIZE_ETC_ID: ::flatbuffers::VOffsetT = 16;
    pub const VT_ICON: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(IdCardBackgroundExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn rarity(&self) -> Rarity {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Rarity>(IdCardBackgroundExcel::VT_RARITY, Some(Rarity::N))
                .unwrap()
        }
    }
    #[inline]
    pub fn display_order(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(IdCardBackgroundExcel::VT_DISPLAY_ORDER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn collection_visible(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(IdCardBackgroundExcel::VT_COLLECTION_VISIBLE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn is_default(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(IdCardBackgroundExcel::VT_IS_DEFAULT, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn bg_path(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                IdCardBackgroundExcel::VT_BG_PATH,
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
                .get::<u32>(IdCardBackgroundExcel::VT_LOCALIZE_ETC_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn icon(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(IdCardBackgroundExcel::VT_ICON, None)
        }
    }
}

impl ::flatbuffers::Verifiable for IdCardBackgroundExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<Rarity>("rarity", Self::VT_RARITY, false)?
            .visit_field::<i64>("display_order", Self::VT_DISPLAY_ORDER, false)?
            .visit_field::<bool>("collection_visible", Self::VT_COLLECTION_VISIBLE, false)?
            .visit_field::<bool>("is_default", Self::VT_IS_DEFAULT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "bg_path",
                Self::VT_BG_PATH,
                false,
            )?
            .visit_field::<u32>("localize_etc_id", Self::VT_LOCALIZE_ETC_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("icon", Self::VT_ICON, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for IdCardBackgroundExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("IdCardBackgroundExcel", 8)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("rarity", &self.rarity())?;
        s.serialize_field("display_order", &self.display_order())?;
        s.serialize_field("collection_visible", &self.collection_visible())?;
        s.serialize_field("is_default", &self.is_default())?;
        if let Some(f) = self.bg_path() {
            s.serialize_field("bg_path", &f)?;
        } else {
            s.skip_field("bg_path")?;
        }
        s.serialize_field("localize_etc_id", &self.localize_etc_id())?;
        if let Some(f) = self.icon() {
            s.serialize_field("icon", &f)?;
        } else {
            s.skip_field("icon")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for IdCardBackgroundExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("IdCardBackgroundExcel");
        ds.field("id", &self.id());
        ds.field("rarity", &self.rarity());
        ds.field("display_order", &self.display_order());
        ds.field("collection_visible", &self.collection_visible());
        ds.field("is_default", &self.is_default());
        ds.field("bg_path", &self.bg_path());
        ds.field("localize_etc_id", &self.localize_etc_id());
        ds.field("icon", &self.icon());
        ds.finish()
    }
}
