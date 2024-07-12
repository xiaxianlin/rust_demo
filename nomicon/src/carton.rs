use std::{
    cmp::max,
    mem::*,
    ops::{Deref, DerefMut},
    ptr,
};

pub struct Carton<T>(ptr::NonNull<T>);

impl<T> Carton<T> {
    pub fn new(value: T) -> Self {
        assert_ne!(size_of::<T>(), 0);

        let mut mem_ptr: *mut T = ptr::null_mut();
        unsafe {
            let ret = libc::posix_memalign(
                (&mut mem_ptr as *mut *mut T).cast(),
                max(align_of::<T>(), size_of::<usize>()),
                size_of::<T>(),
            );
            assert_eq!(ret, 0, "Failed to allocate or invalid alignment");
        };

        let ptr = {
            ptr::NonNull::new(mem_ptr.cast::<T>())
                .expect("Guaranteed non-null if posix_memalign returns 0")
        };

        unsafe {
            ptr.as_ptr().write(value);
        }

        Self(ptr)
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr().cast());
        }
    }
}

// unsafe impl<T> Send for Carton<T> where T: Send {}
unsafe impl<T> Sync for Carton<T> where T: Sync {}
unsafe impl<T> Send for Carton<T> where Box<T>: Send {}
