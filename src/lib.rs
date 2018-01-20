#![feature(optin_builtin_traits, alloc, unique)]

#![no_std]

extern crate alloc;

extern {
    fn critical_section_enter();
    fn critical_section_exit();
}

/// While this is alive no interruption nor context switch may occur.
/// Using a droppable
pub struct CriticalSection {
}

impl CriticalSection {
    pub fn new() -> CriticalSection {
        unsafe { critical_section_enter() };
        CriticalSection { }
    }
}
impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe { critical_section_exit() };
    }
}
impl !Sync for CriticalSection {}
impl !Send for CriticalSection {}

