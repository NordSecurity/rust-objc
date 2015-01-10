use std::ptr;

use runtime::{Class, Object};

/*
 The Sized bound on Message is unfortunate; ideally, objc objects would not be
 treated as Sized. However, rust won't allow casting a dynamically-sized type
 pointer to an Object pointer, because dynamically-sized types can have fat
 pointers (two words) instead of real pointers.
 */
/// Types that may be sent Objective-C messages.
/// For example: objects, classes, and blocks.
pub unsafe trait Message: Sized { }

unsafe impl Message for Object { }

unsafe impl Message for Class { }

/// A trait for converting to a pointer to a type that may be sent Objective-C
/// messages.
pub trait ToMessage<T: Message> {
    fn as_ptr(&self) -> *mut T;

    fn is_nil(&self) -> bool {
        self.as_ptr().is_null()
    }
}

impl<T: Message> ToMessage<T> for *const T {
    fn as_ptr(&self) -> *mut T {
        *self as *mut T
    }
}

impl<T: Message> ToMessage<T> for *mut T {
    fn as_ptr(&self) -> *mut T {
        *self
    }
}

impl<'a, T: Message> ToMessage<T> for &'a T {
    fn as_ptr(&self) -> *mut T {
        *self as *const T as *mut T
    }
}

impl<'a, T: Message> ToMessage<T> for &'a mut T {
    fn as_ptr(&self) -> *mut T {
        *self
    }
}

impl<'a, T: Message> ToMessage<T> for Option<&'a T> {
    fn as_ptr(&self) -> *mut T {
        match *self {
            None => ptr::null_mut(),
            Some(ref obj) => obj.as_ptr(),
        }
    }
}

impl<'a, T: Message> ToMessage<T> for Option<&'a mut T> {
    fn as_ptr(&self) -> *mut T {
        match *self {
            None => ptr::null_mut(),
            Some(ref obj) => obj.as_ptr(),
        }
    }
}

/// Converts to an Object pointer; this function is mainly used by the
/// `msg_send!` macro.
pub fn to_obj_ptr<T: Message, M: ToMessage<T>>(obj_ref: &M) -> *mut Object {
    obj_ref.as_ptr() as *mut Object
}
