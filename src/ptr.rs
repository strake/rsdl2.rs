use core::marker::PhantomData;
use core::ops::*;

pub trait DropPtr: private::Sealed {
    fn drop_ptr(*mut Self);
}

pub(crate) mod private { pub trait Sealed {} }

pub struct Ptr<'a, A: ?Sized + DropPtr>(pub(crate) *mut A, pub(crate) PhantomData<&'a mut ()>);

impl<'a, A: ?Sized + DropPtr> Ptr<'a, A> {
    #[inline]
    pub unsafe fn new(ptr: *mut A) -> Option<Self> {
        if 0 == ptr as *mut () as usize { None } else { Some(Ptr(ptr, PhantomData)) }
    }
}

impl<'a, A: ?Sized + DropPtr> Drop for Ptr<'a, A> {
    #[inline]
    fn drop(&mut self) { A::drop_ptr(self.0) }
}

impl<'a, A: ?Sized + DropPtr> Deref for Ptr<'a, A> {
    type Target = A;

    #[inline]
    fn deref(&self) -> &A { unsafe { &*self.0 } }
}

impl<'a, A: ?Sized + DropPtr> DerefMut for Ptr<'a, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut A { unsafe { &mut *self.0 } }
}
