//! Pre-seeded module base cache.
//!
//! Callers must invoke `init_module_bases()` before any `syscall!()` or
//! `spoof!()` call.  This eliminates the need for `dinvk::get_module_address()`
//! (which walks PEB→Ldr) at runtime, allowing LTO to strip its code — and the
//! telltale `gs:[0x60]` instruction — from the final binary.

use core::ffi::c_void;
use core::sync::atomic::{AtomicPtr, Ordering};

static NTDLL_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
static KERNEL32_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
static KERNELBASE_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());

/// Seed the module-base cache.  Must be called once before any spoofed
/// call or syscall.  Bases are typically resolved by the loader (via IAT)
/// or received through a bootstrap struct from the parent process.
pub fn init_module_bases(
    ntdll: *mut c_void,
    kernel32: *mut c_void,
    kernelbase: *mut c_void,
) {
    NTDLL_BASE.store(ntdll, Ordering::Release);
    KERNEL32_BASE.store(kernel32, Ordering::Release);
    KERNELBASE_BASE.store(kernelbase, Ordering::Release);
}

#[inline]
pub(crate) fn cached_ntdll() -> *mut c_void {
    NTDLL_BASE.load(Ordering::Acquire)
}

#[inline]
pub(crate) fn cached_kernel32() -> *mut c_void {
    KERNEL32_BASE.load(Ordering::Acquire)
}

#[inline]
pub(crate) fn cached_kernelbase() -> *mut c_void {
    KERNELBASE_BASE.load(Ordering::Acquire)
}
