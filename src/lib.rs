//! Rust interface for Objective-C's `@throw` and `@try`/`@catch` statements.

#![no_std]
#![feature(c_unwind)]
#![feature(core_intrinsics)]

#[cfg(test)]
extern crate alloc;

use core::ffi::c_void;
use core::intrinsics;
use core::mem::ManuallyDrop;

#[link(name = "objc", kind = "dylib")]
extern "C-unwind" {
    // Header marks this with _Nonnull, but LLVM output shows otherwise
    fn objc_exception_throw(exception: *mut c_void) -> !;
    // fn objc_exception_rethrow();
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    // Header marks this with _Nonnull, but LLVM output shows otherwise
    fn objc_begin_catch(exc_buf: *mut c_void) -> *mut c_void;
    fn objc_end_catch();

    // Doesn't belong in this library, but is required to return the exception
    fn objc_retain(value: *mut c_void) -> *mut c_void;
}

/// An opaque type representing any Objective-C object thrown as an exception.
pub enum Exception { }

/// Throws an Objective-C exception.
/// The argument must be a pointer to an Objective-C object.
///
/// Unsafe because this unwinds from Objective-C.
#[inline]
pub unsafe fn throw(exception: *mut Exception) -> ! {
    objc_exception_throw(exception as *mut _)
}

/// Tries to execute the given closure and catches an Objective-C exception
/// if one is thrown.
///
/// Returns a `Result` that is either `Ok` if the closure succeeded without an
/// exception being thrown, or an `Err` with a pointer to an exception if one
/// was thrown. The exception is retained and so must be released.
///
/// Unsafe because this encourages unwinding through the closure from
/// Objective-C, which is not safe.
pub unsafe fn r#try<F: FnOnce() -> R, R>(f: F) -> Result<R, *mut Exception> {
    // Our implementation is just a copy of `std::panicking::r#try`:
    // https://github.com/rust-lang/rust/blob/1.52.1/library/std/src/panicking.rs#L299-L408
    union Data<F, R> {
        f: ManuallyDrop<F>,
        r: ManuallyDrop<R>,
        p: ManuallyDrop<*mut Exception>,
    }

    let mut data = Data { f: ManuallyDrop::new(f) };

    let data_ptr = &mut data as *mut _ as *mut u8;

    return if intrinsics::r#try(do_call::<F, R>, data_ptr, do_catch::<F, R>) == 0 {
        Ok(ManuallyDrop::into_inner(data.r))
    } else {
        Err(ManuallyDrop::into_inner(data.p))
    };

    /// Only function that we've changed
    #[cold]
    unsafe fn cleanup(payload: *mut u8) -> *mut Exception {
        // We let Objective-C process the unwind payload, and hand us the
        // exception object. Everything between this and `objc_end_catch` is
        // treated as a `@catch` block.
        let obj = objc_begin_catch(payload as *mut c_void);
        // We retain the exception since it might have been autoreleased.
        // This cannot unwind, so we don't need extra guards here.
        let obj = objc_retain(obj) as *mut Exception;
        // End the `@catch` block.
        objc_end_catch();
        obj
    }

    #[inline]
    fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let f = ManuallyDrop::take(&mut data.f);
            data.r = ManuallyDrop::new(f());
        }
    }

    #[inline]
    fn do_catch<F: FnOnce() -> R, R>(data: *mut u8, payload: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let obj = cleanup(payload);
            data.p = ManuallyDrop::new(obj);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::ptr;

    use super::{r#try, throw};

    #[test]
    fn test_try() {
        unsafe {
            let s = "Hello".to_string();
            let result = r#try(move || {
                if s.len() > 0 {
                    throw(ptr::null_mut());
                }
                s.len()
            });
            assert!(result.unwrap_err() == ptr::null_mut());

            let mut s = "Hello".to_string();
            let result = r#try(move || {
                s.push_str(", World!");
                s
            });
            assert!(result.unwrap() == "Hello, World!");
        }
    }
}
