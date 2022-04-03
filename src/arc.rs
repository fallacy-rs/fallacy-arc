//! A thread-safe reference-counting pointer.

use crate::Weak;
use fallacy_alloc::AllocError;
use std::alloc::Layout;
use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc as StdArc;

/// A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically
/// Reference Counted'.
///
/// The type `Arc<T>` provides shared ownership cod
/// of a value of type `T`,
/// allocated in the heap. Invoking `clone` on `Arc` produces
/// a new `Arc` instance, which points to the same allocation on the heap as the
/// source `Arc`, while increasing a reference count. When the last `Arc`
/// pointer to a given allocation is destroyed, the value stored in that allocation (often
/// referred to as "inner value") is also dropped.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Arc<T: ?Sized>(StdArc<T>);

impl<T> Arc<T> {
    /// Constructs a new `Arc<T>`, returning an error if allocation fails.
    #[inline]
    pub fn try_new(data: T) -> Result<Arc<T>, AllocError> {
        Ok(Arc(
            StdArc::try_new(data).map_err(|_| AllocError::new(Layout::new::<T>()))?
        ))
    }
}

impl<T: ?Sized> Arc<T> {
    #[inline]
    pub fn into_std(self) -> StdArc<T> {
        self.0
    }

    #[inline]
    pub fn from_std(a: StdArc<T>) -> Self {
        Arc(a)
    }

    /// Creates a new [`Weak`] pointer to this allocation.
    #[must_use = "this returns a new `Weak` pointer, \
                  without modifying the original `Arc`"]
    #[inline]
    pub fn downgrade(this: &Self) -> Weak<T> {
        Weak::from_std(StdArc::downgrade(&this.0))
    }

    /// Gets the number of [`Weak`] pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the weak count at any time,
    /// including potentially between calling this method and acting on the result.
    #[must_use]
    #[inline]
    pub fn weak_count(this: &Self) -> usize {
        StdArc::weak_count(&this.0)
    }

    /// Gets the number of strong (`Arc`) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    #[must_use]
    #[inline]
    pub fn strong_count(this: &Self) -> usize {
        StdArc::strong_count(&this.0)
    }

    /// Returns `true` if the two `Arc`s point to the same allocation
    /// (in a vein similar to [`std::ptr::eq`]).
    #[must_use]
    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        StdArc::ptr_eq(&this.0, &other.0)
    }
}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}
