/// Hint that the `Deserialize` type is expecting an `i8` value.
fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>;

/// Hint that the `Deserialize` type is expecting an `i16` value.
fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>;

/// Hint that the `Deserialize` type is expecting an `i32` value.
fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>;

/// Hint that the `Deserialize` type is expecting an `i64` value.
fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>;

/// Hint that the `Deserialize` type is expecting an `i128` value.
///
/// The default behavior unconditionally returns an error.
fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>,
{
    let _ = visitor;
    Err(Error::custom("i128 is not supported"))
}
