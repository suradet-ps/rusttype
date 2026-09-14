impl<E> UnitDeserializer<E> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        UnitDeserializer {
            marker: PhantomData,
        }
    }
}

impl<'de, E> de::Deserializer<'de> for UnitDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_none()
    }
}
