#[inline]
fn visit_none<E>(self) -> Result<Self::Value, E> {
    Ok(IgnoredAny)
}

#[inline]
fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
where
    D: Deserializer<'de>,
{
    IgnoredAny::deserialize(deserializer)
}

#[inline]
fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
where
    D: Deserializer<'de>,
{
    IgnoredAny::deserialize(deserializer)
}

#[inline]
fn visit_unit<E>(self) -> Result<Self::Value, E> {
    Ok(IgnoredAny)
}
