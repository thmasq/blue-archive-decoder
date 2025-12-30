extern crate alloc;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub struct MemoryLobby_GlobalExcel<'a> {
    pub _tab: ::flatbuffers::Table<'a>,
}

impl<'a> ::flatbuffers::Follow<'a> for MemoryLobby_GlobalExcel<'a> {
    type Inner = MemoryLobby_GlobalExcel<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: unsafe { ::flatbuffers::Table::new(buf, loc) },
        }
    }
}

impl<'a> MemoryLobby_GlobalExcel<'a> {
    pub const VT_ID: ::flatbuffers::VOffsetT = 4;
    pub const VT_CHARACTER_ID: ::flatbuffers::VOffsetT = 6;
    pub const VT_PREFAB_NAME_KR: ::flatbuffers::VOffsetT = 8;
    pub const VT_PREFAB_NAME_TW: ::flatbuffers::VOffsetT = 10;
    pub const VT_PREFAB_NAME_ASIA: ::flatbuffers::VOffsetT = 12;
    pub const VT_PREFAB_NAME_NA: ::flatbuffers::VOffsetT = 14;
    pub const VT_PREFAB_NAME_GLOBAL: ::flatbuffers::VOffsetT = 16;
    pub const VT_PREFAB_NAME_TEEN: ::flatbuffers::VOffsetT = 18;

    #[inline]
    pub fn id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MemoryLobby_GlobalExcel::VT_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn character_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(MemoryLobby_GlobalExcel::VT_CHARACTER_ID, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn prefab_name_kr(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_KR,
                None,
            )
        }
    }
    #[inline]
    pub fn prefab_name_tw(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_TW,
                None,
            )
        }
    }
    #[inline]
    pub fn prefab_name_asia(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_ASIA,
                None,
            )
        }
    }
    #[inline]
    pub fn prefab_name_na(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_NA,
                None,
            )
        }
    }
    #[inline]
    pub fn prefab_name_global(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_GLOBAL,
                None,
            )
        }
    }
    #[inline]
    pub fn prefab_name_teen(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<::flatbuffers::ForwardsUOffset<&str>>(
                MemoryLobby_GlobalExcel::VT_PREFAB_NAME_TEEN,
                None,
            )
        }
    }
}

impl ::flatbuffers::Verifiable for MemoryLobby_GlobalExcel<'_> {
    #[inline]
    fn run_verifier(
        v: &mut ::flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<i64>("id", Self::VT_ID, false)?
            .visit_field::<i64>("character_id", Self::VT_CHARACTER_ID, false)?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_kr",
                Self::VT_PREFAB_NAME_KR,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_tw",
                Self::VT_PREFAB_NAME_TW,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_asia",
                Self::VT_PREFAB_NAME_ASIA,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_na",
                Self::VT_PREFAB_NAME_NA,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_global",
                Self::VT_PREFAB_NAME_GLOBAL,
                false,
            )?
            .visit_field::<::flatbuffers::ForwardsUOffset<&str>>(
                "prefab_name_teen",
                Self::VT_PREFAB_NAME_TEEN,
                false,
            )?
            .finish();
        Ok(())
    }
}

impl Serialize for MemoryLobby_GlobalExcel<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MemoryLobby_GlobalExcel", 8)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("character_id", &self.character_id())?;
        if let Some(f) = self.prefab_name_kr() {
            s.serialize_field("prefab_name_kr", &f)?;
        } else {
            s.skip_field("prefab_name_kr")?;
        }
        if let Some(f) = self.prefab_name_tw() {
            s.serialize_field("prefab_name_tw", &f)?;
        } else {
            s.skip_field("prefab_name_tw")?;
        }
        if let Some(f) = self.prefab_name_asia() {
            s.serialize_field("prefab_name_asia", &f)?;
        } else {
            s.skip_field("prefab_name_asia")?;
        }
        if let Some(f) = self.prefab_name_na() {
            s.serialize_field("prefab_name_na", &f)?;
        } else {
            s.skip_field("prefab_name_na")?;
        }
        if let Some(f) = self.prefab_name_global() {
            s.serialize_field("prefab_name_global", &f)?;
        } else {
            s.skip_field("prefab_name_global")?;
        }
        if let Some(f) = self.prefab_name_teen() {
            s.serialize_field("prefab_name_teen", &f)?;
        } else {
            s.skip_field("prefab_name_teen")?;
        }
        s.end()
    }
}

impl ::core::fmt::Debug for MemoryLobby_GlobalExcel<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut ds = f.debug_struct("MemoryLobby_GlobalExcel");
        ds.field("id", &self.id());
        ds.field("character_id", &self.character_id());
        ds.field("prefab_name_kr", &self.prefab_name_kr());
        ds.field("prefab_name_tw", &self.prefab_name_tw());
        ds.field("prefab_name_asia", &self.prefab_name_asia());
        ds.field("prefab_name_na", &self.prefab_name_na());
        ds.field("prefab_name_global", &self.prefab_name_global());
        ds.field("prefab_name_teen", &self.prefab_name_teen());
        ds.finish()
    }
}
