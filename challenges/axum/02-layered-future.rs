pin_project! {
    /// The response future for [`Layered`](super::Layered).
    pub struct LayeredFuture<S>
    where
        S: Service<Request>,
    {
        #[pin]
        inner: Map<Oneshot<S, Request>, fn(Result<S::Response, S::Error>) -> Response>,
    }
}

impl<S> LayeredFuture<S>
where
    S: Service<Request>,
{
    pub(super) fn new(
        inner: Map<Oneshot<S, Request>, fn(Result<S::Response, S::Error>) -> Response>,
    ) -> Self {
        Self { inner }
    }
}

impl<S> Future for LayeredFuture<S>
where
    S: Service<Request>,
{
    type Output = Response;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}
