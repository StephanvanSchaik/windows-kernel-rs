use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use windows_kernel_sys::base::EX_PUSH_LOCK;
use windows_kernel_sys::ntoskrnl::{
    ExInitializePushLock,
    ExAcquirePushLockShared,
    ExReleasePushLockShared,
    ExAcquirePushLockExclusive,
    ExReleasePushLockExclusive,
    KeEnterCriticalRegion,
    KeLeaveCriticalRegion,
};

/// A [`PushLock`] is an efficient implementation of a reader-writer lock that can be stored both
/// in paged and non-paged memory.
///
/// This type of lock allows a number of readers or at most one writer at any point in time. The
/// write portion of this lock typically allows modifications of the underlying data (exclusive
/// access) and the read portion of this lock typically allows for read-only access (shared
/// access).
///
/// In comparison, a [`FastMutex`] does not distinguish between readers or writers that acquire the
/// lock, therefore blocking any threads waiting for the lock to become available. A [`PushLock`]
/// will allow any number of readers to acquire the lock as long as a writer is not holding the
/// lock.
///
/// The priority policy is such that a thread trying to acquire the [`PushLock`] for exclusive
/// access will be prioritized over threads trying to acquire the [`PushLock`] for shared access.
/// More specifically, if a thread cannot lock the [`PushLock`] for exclusive access immediately,
/// it will wait for the thread(s) that currently holds the lock to release the lock. If another
/// thread tries to acquire the [`PushLock`] for shared access while a thread is waiting to acquire
/// the lock for exclusive access, it will yield to the thread(s) trying to acquire the
/// [`PushLock`] for exclusive access, even in the event that the [`PushLock`] is acquired for
/// shared access.
///
/// [`FastMutex`]: crate::fast_mutex::FastMutex
pub struct PushLock<T: ?Sized> {
    pub(crate) lock: Box<EX_PUSH_LOCK>,
    pub(crate) data: UnsafeCell<T>,
}

unsafe impl<T> Send for PushLock<T> {}
unsafe impl<T> Sync for PushLock<T> {}

impl<T> PushLock<T> {
    /// Creates new instance of [`PushLock<T>`] that is unlocked.
    pub fn new(data: T) -> Self {
        let mut lock: Box<EX_PUSH_LOCK> = Box::new(0);

        unsafe {
            ExInitializePushLock(
                &mut *lock,
            )
        };

        Self {
            lock,
            data: UnsafeCell::new(data),
        }
    }

    /// Consumes this [`PushLock`], returning the underlying data.
    #[inline]
    pub fn into_inner(self) -> T {
        let Self { data, .. } = self;
        data.into_inner()
    }

    /// Locks this [`PushLock`] with shared read access, blocking the current thread until it can
    /// be acquired.
    ///
    /// The calling thread will be blocked until there are no more writers which hold the lock.
    /// There may be other readers currently inside the lock when this method returns.
    ///
    /// This function will yield to threads waiting to acquire the [`PushLock`] for exclusive
    /// access, even in the event that the [`PushLock`] is currently held by one or more threads
    /// for shared access.
    ///
    /// While the underlying function does allow for recursion, this atomically increments a shared
    /// reader counter. Since dropping the RAII guard releases the lock by atomically decrementing
    /// this shared counter, it will eventually reach zero once all RAII guards have been dropped. 
    #[inline]
    pub fn read(&mut self) -> Option<PushLockReadGuard<T>> {
        unsafe {
            KeEnterCriticalRegion()
        };

        unsafe {
            ExAcquirePushLockShared(
                &mut *self.lock,
            )
        };

        Some(PushLockReadGuard {
            lock: &mut self.lock,
            data: unsafe { &mut *self.data.get() },
        })
    }

    /// Locks this [`PushLock`] with exclusive write access, blocking the current thread until it can
    /// be acquired.
    ///
    /// This function will not return while other writers or other readers currently have access to
    /// the lock.
    ///
    /// Returns an RAII guard which will drop the write access of this [`PushLock`] when dropped.
    ///
    /// This thread will take priority over any threads that are trying to acquire the lock for
    /// shared access but that do not currently hold the lock for shared access.
    ///
    /// The underlying function does not allow for recursion, which ensures correct behavior. 
    #[inline]
    pub fn write(&mut self) -> Option<PushLockWriteGuard<T>> {
        unsafe {
            KeEnterCriticalRegion()
        };

        unsafe {
            ExAcquirePushLockExclusive(
                &mut *self.lock,
            )
        };

        Some(PushLockWriteGuard {
            lock: &mut self.lock,
            data: unsafe { &mut *self.data.get() },
        })
    }
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] and [`try_read`] methods on [`PushLock`]
///
/// [`read`]: PushLock::read
/// [`try_read`]: PushLock::try_read
pub struct PushLockReadGuard<'a, T: 'a + ?Sized> {
    pub(crate) lock: &'a mut EX_PUSH_LOCK,
    pub(crate) data: &'a T,
}

impl<'a, T: ?Sized> Drop for PushLockReadGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ExReleasePushLockShared(
                &mut *self.lock,
            )
        };

        unsafe {
            KeLeaveCriticalRegion()
        };
    }
}

impl<'a, T: ?Sized> Deref for PushLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] and [`try_write`] methods on [`PushLock`]
///
/// [`write`]: PushLock::write
/// [`try_write`]: PushLock::try_write
pub struct PushLockWriteGuard<'a, T: 'a + ?Sized> {
    pub(crate) lock: &'a mut EX_PUSH_LOCK,
    pub(crate) data: &'a mut T,
}

impl<'a, T: ?Sized> Drop for PushLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ExReleasePushLockExclusive(
                &mut *self.lock,
            )
        };

        unsafe {
            KeLeaveCriticalRegion()
        };
    }
}

impl<'a, T: ?Sized> Deref for PushLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for PushLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}
