/// Default hash builder for the `S` type parameter of
/// [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).
///
/// This only implements `BuildHasher` when the "default-hasher" crate feature
/// is enabled; otherwise it just serves as a placeholder, and a custom `S` type
/// must be used to have a fully functional `HashMap` or `HashSet`.
#[derive(Clone, Debug, Default)]
pub struct DefaultHashBuilder {
    #[cfg(feature = "default-hasher")]
    inner: RandomState,
}

#[cfg(feature = "default-hasher")]
impl BuildHasher for DefaultHashBuilder {
    type Hasher = DefaultHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher {
            inner: self.inner.build_hasher(),
        }
    }
}

/// Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).
#[cfg(feature = "default-hasher")]
#[derive(Clone)]
pub struct DefaultHasher {
    inner: <RandomState as BuildHasher>::Hasher,
}
