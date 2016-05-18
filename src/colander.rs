//! Module: czmq
/// Wrapper around Box<T> that deliberately leaks its memory instead
/// of destroying it. This is useful for reading borrowed void ptrs
/// whose memory is managed by C.

use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct Colander<T> {
    inner: Option<Box<T>>,
}

impl<T> Colander<T> {
    pub unsafe fn from_raw(raw: *mut T) -> Colander<T> {
        Colander {
            inner: Some(Box::from_raw(raw)),
        }
    }

    pub fn into_raw(mut self) -> *mut T {
        if let Some(b) = self.inner.take() {
            Box::into_raw(b)
        } else {
            ptr::null_mut()
        }
    }

    pub fn into_inner(mut self) -> Option<Box<T>> {
        self.inner.take()
    }
}

impl<T> AsRef<T> for Colander<T> {
    fn as_ref(&self) -> &T {
        self.inner.as_ref().unwrap().as_ref()
    }
}

impl<T> AsMut<T> for Colander<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().as_mut()
    }
}

impl<T> Borrow<T> for Colander<T> {
    fn borrow(&self) -> &T {
        self.inner.as_ref().unwrap().borrow()
    }
}

impl<T> BorrowMut<T> for Colander<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().borrow_mut()
    }
}

impl<T> Debug for Colander<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.as_ref().unwrap().fmt(f)
    }
}

impl<T> Deref for Colander<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.as_ref().unwrap().deref()
    }
}

impl<T> DerefMut for Colander<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap().deref_mut()
    }
}

impl<T> Drop for Colander<T> {
    fn drop(&mut self) {
        if let Some(b) = self.inner.take() {
            Box::into_raw(b);
        }
    }
}
