//! Class Factory for buttre Text Service

use crate::platforms::windows::tsf::com::{dll_add_ref, dll_release};
use crate::platforms::windows::tsf::text_service::TextService;
use std::sync::atomic::AtomicU32;
use windows::core::*;
use windows::Win32::System::Com::*;

/// Class Factory for creating TextService instances
#[implement(IClassFactory)]
pub struct ClassFactory {
    _ref_count: AtomicU32,
}

impl Default for ClassFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassFactory {
    pub fn new() -> Self {
        dll_add_ref();
        Self {
            _ref_count: AtomicU32::new(1),
        }
    }
}

impl IClassFactory_Impl for ClassFactory_Impl {
    fn CreateInstance(
        &self,
        _punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        // Create TextService instance using windows-rs 0.62 pattern
        let text_service: IUnknown = TextService::new()
            .into_object()
            .into_interface();
        
        // SAFETY:
        // 1. text_service is a valid IUnknown COM interface created above
        // 2. riid points to valid GUID provided by COM runtime
        // 3. ppvobject is a valid output pointer provided by COM runtime
        // 4. query() is a COM method that safely handles interface query
        // 5. If successful, writes interface pointer to ppvobject and increments refcount
        // 6. ok()? converts Result to Result<()> with proper error handling
        unsafe {
            text_service.query(riid, ppvobject).ok()?;
        }
        
        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        if flock.as_bool() {
            dll_add_ref();
        } else {
            dll_release();
        }
        Ok(())
    }
}

impl Drop for ClassFactory {
    fn drop(&mut self) {
        dll_release();
    }
}
