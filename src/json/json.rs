use serde::Serialize;
use serde_json;
use serde::ser::{SerializeMap, Serializer};


struct GenericWrapper<T> {
    field_name: String,
    value: T,
}

impl<T> GenericWrapper<T> {
    fn new(field_name: &str, value: T) -> Self {
        Self {
            field_name: field_name.to_string(),
            value,
        }
    }
}

impl<T> Serialize for GenericWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field_name, &self.value)?;
        map.end()
    }
}


pub fn transform_in_json<T>(field_name: &str, value: T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    let wrapper = GenericWrapper::new(field_name, value);
    serde_json::to_string_pretty(&wrapper)
}

