use core::{
    alloc::Layout,
    ops::Deref,
    ptr::{self, NonNull},
};

use crate::error::Error;

use super::ffi;

/// A pointer type for values on the garbage-collected heap.
pub struct Gc<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Gc<T> {}

impl<T> Gc<T> {
    /// Allocate memory on the heap managed by the MicroPython garbage collector
    /// and then place `v` into it.
    ///
    /// `flags` can be an int value built out of constants in the ffi module.
    /// The current MicroPython only supports GC_ALLOC_FLAG_HAS_FINALISER, which
    /// will cause the __del__ method to be called when the object is
    /// garbage collected.
    ///
    /// SAFETY:
    /// Flag GC_ALLOC_FLAG_HAS_FINALISER can only be used with Python objects
    /// that have a base as their first element
    unsafe fn alloc(v: T, flags: u32) -> Result<Self, Error> {
        let layout = Layout::for_value(&v);
        // TODO: Assert that `layout.align()` is the same as the GC alignment.
        // SAFETY:
        //  - Unfortunately we cannot respect `layout.align()` as MicroPython GC does
        //    not support custom alignment.
        //  - `ptr` is guaranteed to stay valid as long as it's reachable from the stack
        //    or the MicroPython heap.
        // EXCEPTION: Returns null instead of raising.
        unsafe {
            let raw = ffi::gc_alloc(layout.size(), flags);
            if raw.is_null() {
                return Err(Error::AllocationFailed);
            }
            let typed = raw.cast();
            ptr::write(typed, v);
            Ok(Self::from_raw(typed))
        }
    }

    /// Allocate memory on the heap managed by the MicroPython garbage collector
    /// and then place `v` into it. `v` will _not_ get its destructor called.
    pub fn new(v: T) -> Result<Self, Error> {
        unsafe {
            // SAFETY: No flag is used
            Self::alloc(v, 0)
        }
    }

    /// Allocate memory on the heap managed by the MicroPython garbage
    /// collector, place `v` into it, and register for finalisation.
    ///
    /// `v` will **not** get its destructor called automatically! However, if
    /// `v` is a Python-style object (has a base as its first field), and
    /// has a `__del__` method, it will be called when the object is garbage
    /// collected. You can use this to implement custom finalisation, in
    /// which you can, e.g., invoke the Drop implementation.
    /// SAFETY:
    /// Can only be used with Python objects that have a base as their
    /// first element
    pub unsafe fn new_with_custom_finaliser(v: T) -> Result<Self, Error> {
        unsafe { Self::alloc(v, ffi::GC_ALLOC_FLAG_HAS_FINALISER) }
    }
}

impl<T: Default> Gc<[T]> {
    /// Allocate slice on the heap managed by the MicroPython garbage collector
    /// and fill with default values.
    pub fn new_slice(len: usize) -> Result<Self, Error> {
        let layout = Layout::array::<T>(len).unwrap();
        // TODO: Assert that `layout.align()` is the same as the GC alignment.
        // SAFETY:
        //  - Unfortunately we cannot respect `layout.align()` as MicroPython GC does
        //    not support custom alignment.
        //  - `ptr` is guaranteed to stay valid as long as it's reachable from the stack
        //    or the MicroPython heap.
        // EXCEPTION: Returns null instead of raising.
        unsafe {
            let raw = ffi::gc_alloc(layout.size(), 0);
            if raw.is_null() {
                return Err(Error::AllocationFailed);
            }
            let typed: *mut T = raw.cast();
            for i in 0..len {
                ptr::write(typed.add(i), T::default());
            }
            let array_ptr = ptr::slice_from_raw_parts_mut(typed, len);
            Ok(Self::from_raw(array_ptr as _))
        }
    }
}

impl<T: ?Sized> Gc<T> {
    /// Construct a `Gc` from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller has to guarantee that `ptr`
    /// is pointing to a memory understood by the MicroPython GC, that is:
    /// - previously allocated through `Gc::new()` or `gc_alloc()`, or
    /// - through the MicroPython interpreter, or
    /// - one of the GC roots (sys.argv, sys.modules, etc.).
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        // SAFETY: The caller must guarantee that `ptr` is something the MicroPython GC
        // can reason about.
        unsafe { Self(NonNull::new_unchecked(ptr)) }
    }

    /// Convert `this` into a raw pointer. This will _not_ drop the contained
    /// value.
    pub fn into_raw(this: Self) -> *mut T {
        this.0.as_ptr()
    }

    /// Return a mutable reference to the value.
    ///
    /// # Safety
    ///
    /// `Gc` values can originate in the MicroPython interpreter, and these can
    /// be both shared and mutable. Before calling this function, you have to
    /// ensure that `this` is unique for the whole lifetime of the
    /// returned mutable reference.
    pub unsafe fn as_mut(this: &mut Self) -> &mut T {
        // SAFETY: The caller must guarantee that `this` meets all the requirements for
        // a mutable reference.
        unsafe { this.0.as_mut() }
    }

    /// Return a immutable reference to the value.
    ///
    /// # Safety
    ///
    /// `Gc` values can originate in the MicroPython interpreter, and these can
    /// be both shared and mutable. Before calling this function, you have to
    /// ensure that `this` does not get externally mutated and nobody
    /// holds a mutable reference.
    pub unsafe fn as_ref(this: &Self) -> &T {
        // SAFETY: The caller must guarantee that `this` meets all the requirements for
        // a immutable reference.
        unsafe { this.0.as_ref() }
    }
}

impl<T: ?Sized> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
