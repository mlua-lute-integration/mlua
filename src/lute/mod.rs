use crate::error::{Error, Result};
use crate::Lua;
use crate::util::check_stack;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// Flags describing the set of lute standard libraries to load.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LuteStdLib(u32);

impl LuteStdLib {
    pub const CRYPTO : LuteStdLib = LuteStdLib(1);
    pub const FS : LuteStdLib = LuteStdLib(1 << 1);
    pub const LUAU : LuteStdLib = LuteStdLib(1 << 2);
    pub const NET : LuteStdLib = LuteStdLib(1 << 3);
    pub const PROCESS : LuteStdLib = LuteStdLib(1 << 4);
    pub const TASK : LuteStdLib = LuteStdLib(1 << 5);
    pub const VM : LuteStdLib = LuteStdLib(1 << 6);
    pub const SYSTEM : LuteStdLib = LuteStdLib(1 << 7);
    pub const TIME : LuteStdLib = LuteStdLib(1 << 8);

    /// No libraries
    pub const NONE: LuteStdLib = LuteStdLib(0);
    /// (**unsafe**) All standard libraries
    pub const ALL: LuteStdLib = LuteStdLib(u32::MAX);

    pub fn contains(self, lib: Self) -> bool {
        (self & lib).0 != 0
    }
}

impl BitAnd for LuteStdLib {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        LuteStdLib(self.0 & rhs.0)
    }
}

impl BitAndAssign for LuteStdLib {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = LuteStdLib(self.0 & rhs.0)
    }
}

impl BitOr for LuteStdLib {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        LuteStdLib(self.0 | rhs.0)
    }
}

impl BitOrAssign for LuteStdLib {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = LuteStdLib(self.0 | rhs.0)
    }
}

impl BitXor for LuteStdLib {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        LuteStdLib(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for LuteStdLib {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = LuteStdLib(self.0 ^ rhs.0)
    }
}

impl Lua {
    /// Returns if a lute runtime is loaded into the client or not
    pub fn is_lute_loaded(&self) -> Result<bool> {
        let lua = self.lock();
        let mut is_loaded = false;
        unsafe {
            let state = lua.main_state();
            check_stack(state, 1)?;
            protect_lua!(state, 0, 0, |state| {
                if ffi::lutec_isruntimeloaded(state) == 1 {
                    is_loaded = true
                }
            })?;
        };

        Ok(is_loaded)
    }

    /// Sets up a lute runtime on the current Lua state. This internally creates a second auxillary VM
    /// to be created to act as the data VM
    pub fn setup_lute_runtime(&self) -> Result<()> {
        let lua = self.lock();

        unsafe {
            let state = lua.main_state();
            protect_lua!(state, 0, 0, |state| {
                ffi::lutec_setup_runtime(state);
            })?;
        };

        Ok(())
    }

    /// Destroys the lute runtime on the current Lua state. This internally destroys the auxillary VM
    /// created to act as the data VM
    pub fn destroy_lute_runtime(&self) -> Result<()> {
        let lua = self.lock();

        unsafe {
            let state = lua.main_state();
            protect_lua!(state, 0, 0, |state| {
                ffi::lutec_destroy_runtime(state);
            })?;
        };

        Ok(())
    }

    /// Loads the specified lute standard libraries into the current Lua state.
    /// This errors if the runtime is not loaded.
    pub fn load_lute_LuteStdLib(&self, libs: LuteStdLib) -> Result<()> {
        let lua = self.lock();

        if !self.is_lute_loaded()? {
            return Err(Error::external(
                "Lute runtime is not loaded. Please call setup_lute_runtime first.",
            ));
        }

        unsafe {
            let state = lua.main_state();
            check_stack(state, 1)?;
            protect_lua!(state, 0, 0, |state| {
                if libs.contains(LuteStdLib::CRYPTO) {
                    ffi::lutec_opencrypto(state);
                    ffi::lua_setglobal(state, c"crypto".as_ptr());
                    
                }
                if libs.contains(LuteStdLib::FS) {
                    ffi::lutec_openfs(state);
                    ffi::lua_setglobal(state, c"fs".as_ptr());
                }
                if libs.contains(LuteStdLib::LUAU) {
                    ffi::lutec_openluau(state);
                    ffi::lua_setglobal(state, c"luau".as_ptr());
                }
                if libs.contains(LuteStdLib::NET) {
                    ffi::lutec_opennet(state);
                    ffi::lua_setglobal(state, c"net".as_ptr());
                }
                if libs.contains(LuteStdLib::PROCESS) {
                    ffi::lutec_openprocess(state);
                    ffi::lua_setglobal(state, c"process".as_ptr());
                }
                if libs.contains(LuteStdLib::TASK) {
                    ffi::lutec_opentask(state);
                    ffi::lua_setglobal(state, c"task".as_ptr());
                }
                if libs.contains(LuteStdLib::VM) {
                    ffi::lutec_openvm(state);
                    ffi::lua_setglobal(state, c"vm".as_ptr());
                }
                if libs.contains(LuteStdLib::SYSTEM) {
                    ffi::lutec_opensystem(state);
                    ffi::lua_setglobal(state, c"system".as_ptr());
                }
                if libs.contains(LuteStdLib::TIME) {
                    ffi::lutec_opentime(state);
                    ffi::lua_setglobal(state, c"time".as_ptr());
                }
            })?;
        };

        Ok(())
    }
}

