    /// Calls the `action` function, and returns a `Future` that resolves when any
    /// [`AsyncDerived`](crate::computed::AsyncDerived) or
    /// or [`ArcAsyncDerived`](crate::computed::ArcAsyncDerived) that is read during the action
    /// has resolved.
    ///
    /// This allows for an inversion of control: the caller does not need to know when all the
    /// resources created inside the `action` will resolve, but can wait for them to notify it.
    pub async fn run<T, U>(action: impl FnOnce() -> T) -> U
    where
        T: Future<Output = U>,
    {
        let (tx, rx) = mpsc::channel();
        let global_transition = global_transition();
        let inner = TransitionInner { tx };
        let prev = Option::replace(
            &mut *global_transition.write().or_poisoned(),
            inner.clone(),
        );
        let value = action().await;
        _ = std::mem::replace(
            &mut *global_transition.write().or_poisoned(),
            prev,
        );
        let mut pending = Vec::new();
        while let Ok(tx) = rx.try_recv() {
            pending.push(tx);
        }
        join_all(pending).await;
        value
    }
