//! Bounded, duplicate-aware JSON ingestion shared by the saved-only readers.
use super::{Error, Result};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::{fmt, fs::File, io::Read, path::Path};

pub(crate) const MAX_DEPTH: usize = 16;
pub(crate) const MAX_FIELDS: usize = 64;
pub(crate) const MAX_STRING: usize = 1024;
pub(crate) const MAX_ARRAY: usize = 65536;

pub(crate) fn read(path: &Path, limit: usize) -> Result<(Vec<u8>, File)> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let mut file = options.open(path).map_err(|_| Error::InputUnavailable)?;
    let metadata = file.metadata().map_err(|_| Error::InputUnavailable)?;
    if !metadata.is_file() {
        return Err(Error::InputUnavailable);
    }
    if metadata.len() > limit as u64 {
        return Err(Error::ResourceLimit);
    }
    let mut bytes = Vec::new();
    (&mut file)
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::InputUnavailable)?;
    if bytes.len() > limit {
        return Err(Error::ResourceLimit);
    }
    Ok((bytes, file))
}

pub(crate) fn parse(bytes: &[u8], limit: usize) -> Result<Value> {
    if bytes.len() > limit {
        return Err(Error::ResourceLimit);
    }
    // serde_json rejects BOM, malformed UTF-8, trailing input and nonfinite numbers.
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = Seed(0).deserialize(&mut deserializer).map_err(|e| {
        if e.to_string().starts_with("resource_limit") {
            Error::ResourceLimit
        } else {
            Error::InvalidInput
        }
    })?;
    deserializer.end().map_err(|_| Error::InvalidInput)?;
    Ok(value)
}

struct Seed(usize);
pub(crate) fn deserialize_value<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Value, D::Error> {
    Seed(0).deserialize(deserializer)
}
impl<'de> DeserializeSeed<'de> for Seed {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        de: D,
    ) -> std::result::Result<Value, D::Error> {
        de.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> std::result::Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Value, E> {
        Ok(Value::Number(v.into()))
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Value, E> {
        Ok(Value::Number(v.into()))
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<Value, E> {
        Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("invalid_input"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Value, E> {
        if v.len() > MAX_STRING {
            return Err(E::custom("resource_limit"));
        }
        Ok(Value::String(v.into()))
    }
    fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<Value, A::Error> {
        use serde::de::Error;
        if self.0 >= MAX_DEPTH {
            return Err(A::Error::custom("resource_limit"));
        }
        let mut values = Vec::new();
        while let Some(value) = seq.next_element_seed(Seed(self.0 + 1))? {
            if values.len() == MAX_ARRAY {
                return Err(A::Error::custom("resource_limit"));
            }
            values.push(value);
        }
        Ok(Value::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<Value, A::Error> {
        use serde::de::Error;
        if self.0 >= MAX_DEPTH {
            return Err(A::Error::custom("resource_limit"));
        }
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if key.len() > MAX_STRING || values.len() == MAX_FIELDS {
                return Err(A::Error::custom("resource_limit"));
            }
            if values.contains_key(&key) {
                return Err(A::Error::custom("invalid_input"));
            }
            values.insert(key, map.next_value_seed(Seed(self.0 + 1))?);
        }
        Ok(Value::Object(values))
    }
}

pub(crate) fn collection_timestamp(value: &str) -> Result<i64> {
    use chrono::{Datelike, Timelike};
    let date = chrono::DateTime::parse_from_rfc3339(value).map_err(|_| Error::InvalidInput)?;
    if !(1970..=9999).contains(&date.year()) || date.nanosecond() >= 1_000_000_000 {
        return Err(Error::InvalidInput);
    }
    let utc = date.with_timezone(&chrono::Utc);
    if !(1970..=9999).contains(&utc.year()) {
        return Err(Error::InvalidInput);
    }
    // Existing collectors emit subsecond collection times; retain the original
    // string and round upward only for comparison to a whole-second as_of.
    Ok(utc.timestamp() + i64::from(utc.nanosecond() != 0))
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
}

pub(crate) fn vocabulary(value: &str, allowed: &[&str]) -> String {
    if allowed.contains(&value) {
        value
    } else {
        "unknown"
    }
    .into()
}
