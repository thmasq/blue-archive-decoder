extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use super::ScenarioBGScroll;

#[derive(Copy, Clone, PartialEq)]
pub struct ScenarioBGEffectExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for ScenarioBGEffectExcel<'a> {
    type Inner = ScenarioBGEffectExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> ScenarioBGEffectExcel<'a> {
    pub const VT_NAME: ::flatbuffers::VOffsetT = 4;
    pub const VT_EFFECT: ::flatbuffers::VOffsetT = 6;
    pub const VT_EFFECT2: ::flatbuffers::VOffsetT = 8;
    pub const VT_SCROLL: ::flatbuffers::VOffsetT = 10;
    pub const VT_SCROLL_TIME: ::flatbuffers::VOffsetT = 12;
    pub const VT_SCROLL_FROM: ::flatbuffers::VOffsetT = 14;
    pub const VT_SCROLL_TO: ::flatbuffers::VOffsetT = 16;

    #[inline]
    pub fn name(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(ScenarioBGEffectExcel::VT_NAME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn effect(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<::flatbuffers::ForwardsUOffset<&str>>(ScenarioBGEffectExcel::VT_EFFECT, None)
        }
    }
    #[inline]
    pub fn effect2(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                ScenarioBGEffectExcel::VT_EFFECT2,
                None,
            )
        }
    }
    #[inline]
    pub fn scroll(&self) -> ScenarioBGScroll {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ScenarioBGScroll>(
                    ScenarioBGEffectExcel::VT_SCROLL,
                    Some(ScenarioBGScroll::None),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn scroll_time(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioBGEffectExcel::VT_SCROLL_TIME, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scroll_from(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioBGEffectExcel::VT_SCROLL_FROM, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn scroll_to(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(ScenarioBGEffectExcel::VT_SCROLL_TO, Some(0))
                .unwrap()
        }
    }
}

impl ::flatbuffers::Verifiable for ScenarioBGEffectExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("name", Self::VT_NAME, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>("effect", Self::VT_EFFECT, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "effect2",
                Self::VT_EFFECT2,
                false,
            )?
            .visit_field::<ScenarioBGScroll>("scroll", Self::VT_SCROLL, false)?
            .visit_field::<i64>("scroll_time", Self::VT_SCROLL_TIME, false)?
            .visit_field::<i64>("scroll_from", Self::VT_SCROLL_FROM, false)?
            .visit_field::<i64>("scroll_to", Self::VT_SCROLL_TO, false)?
            .finish();
        Ok(())
    }
}

impl Serialize for ScenarioBGEffectExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ScenarioBGEffectExcel", 7)?;
        s.serialize_field("name", &self.name())?;
        if let Some(f) = self.effect() {
            s.serialize_field("effect", &f)?;
        } else {
            s.skip_field("effect")?;
        }
        if let Some(f) = self.effect2() {
            s.serialize_field("effect2", &f)?;
        } else {
            s.skip_field("effect2")?;
        }
        s.serialize_field("scroll", &self.scroll())?;
        s.serialize_field("scroll_time", &self.scroll_time())?;
        s.serialize_field("scroll_from", &self.scroll_from())?;
        s.serialize_field("scroll_to", &self.scroll_to())?;
        s.end()
    }
}

impl ::core::fmt::Debug for ScenarioBGEffectExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("ScenarioBGEffectExcel");
        ds.field("name", &self.name());
        ds.field("effect", &self.effect());
        ds.field("effect2", &self.effect2());
        ds.field("scroll", &self.scroll());
        ds.field("scroll_time", &self.scroll_time());
        ds.field("scroll_from", &self.scroll_from());
        ds.field("scroll_to", &self.scroll_to());
        ds.finish()
    }
}
