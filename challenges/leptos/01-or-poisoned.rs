/// Unwraps a lock.
pub trait OrPoisoned {
    /// The inner guard type.
    type Inner;

    /// Unwraps the lock.
    ///
    /// ## Panics
    ///
    /// Will panic if the lock is poisoned.
    fn or_poisoned(self) -> Self::Inner;
}

impl<'a, T: ?Sized> OrPoisoned
    for Result<RwLockReadGuard<'a, T>, PoisonError<RwLockReadGuard<'a, T>>>
{
    type Inner = RwLockReadGuard<'a, T>;

    fn or_poisoned(self) -> Self::Inner {
        self.expect("lock poisoned")
    }
}

impl<'a, T: ?Sized> OrPoisoned
    for Result<RwLockWriteGuard<'a, T>, PoisonError<RwLockWriteGuard<'a, T>>>
{
    type Inner = RwLockWriteGuard<'a, T>;

    fn or_poisoned(self) -> Self::Inner {
        self.expect("lock poisoned")
    }
}

impl<'a, T: ?Sized> OrPoisoned for LockResult<MutexGuard<'a, T>> {
    type Inner = MutexGuard<'a, T>;

    fn or_poisoned(self) -> Self::Inner {
        self.expect("lock poisoned")
    }
}
