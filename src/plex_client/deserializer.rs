use serde::Deserialize;
pub fn deserialize_integer_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(string) => Ok(string == "1"),
        Err(_) => Ok(false),
    }
}
