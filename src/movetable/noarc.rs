use std::ops::Deref;

#[derive(Clone)]
pub struct NoArc<T> {
    ptr: *mut T,
}

impl<T> NoArc<T> {
    pub fn new(val: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(val)),
        }
    }
}

unsafe impl<T> Sync for NoArc<T> {}
unsafe impl<T> Send for NoArc<T> {}

impl<T> Deref for NoArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}
