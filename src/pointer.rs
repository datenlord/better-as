pub fn to_address<T: ?Sized>(ptr: *const T) -> usize {
    ptr as *const () as usize
}

pub fn to_address_mut<T: ?Sized>(ptr: *mut T) -> usize {
    ptr as *mut () as usize
}

pub const fn cast<T: ?Sized, U>(ptr: *const T) -> *const U {
    ptr.cast()
}

pub const fn cast_mut<T: ?Sized, U>(ptr: *mut T) -> *mut U {
    ptr.cast()
}
