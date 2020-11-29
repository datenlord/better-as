pub fn to_address<T: ?Sized>(p: *const T) -> usize {
    p as *const () as usize
}

pub fn to_address_mut<T: ?Sized>(p: *mut T) -> usize {
    p as *mut () as usize
}
