#[diagnostic::on_unimplemented(
    note = "Function argument is not a valid axum extractor. \nSee `https://docs.rs/axum/0.8/axum/extract/index.html` for details"
)]
pub trait FromRequestParts<S>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

/// Types that can be created from requests.
///
/// Extractors that implement `FromRequest` can consume the request body and can thus only be run
/// once for handlers.
///
/// If your extractor doesn't need to consume the request body then you should implement
/// [`FromRequestParts`] and not [`FromRequest`].
///
/// See [`axum::extract`] for more general docs about extractors.
///
/// [`axum::extract`]: https://docs.rs/axum/0.8/axum/extract/index.html
#[diagnostic::on_unimplemented(
    note = "Function argument is not a valid axum extractor. \nSee `https://docs.rs/axum/0.8/axum/extract/index.html` for details"
)]
pub trait FromRequest<S, M = private::ViaRequest>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}
