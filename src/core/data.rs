use super::data_service::{Deserializer, Serializer};

pub trait DataChunk: Sized {
    const ID: &'static str;

    fn serialize(&self, se: &mut Serializer);
    fn deserialize(&self, de: &mut Deserializer) -> Self;
}
#[cfg(test)]
mod test {}
