//! Small bounded serde value decoder. Reject aliases before YAML expansion.
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
#[derive(Clone, Copy)]
pub(super) enum Format {
    Json,
    Toml,
    Yaml,
}
#[derive(Debug, Clone, Copy)]
pub(super) enum Failure {
    Malformed,
    Limit,
    Unsupported,
}
struct Budget {
    remaining: usize,
    limited: bool,
}
struct Seed<'a> {
    budget: &'a mut Budget,
    depth: usize,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        if self.depth > 64 || self.budget.remaining == 0 {
            self.budget.limited = true;
            return Err(de::Error::custom("dependency value limit"));
        }
        self.budget.remaining -= 1;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("bounded dependency value")
    }
    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(Value::Number(v.into()))
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(Value::Number(v.into()))
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite dependency value"))
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.into()))
    }
    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            budget: self.budget,
            depth: self.depth + 1,
        })? {
            out.push(v);
        }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(de::Error::custom("duplicate dependency key"));
            }
            let v = a.next_value_seed(Seed {
                budget: self.budget,
                depth: self.depth + 1,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
fn preflight(text: &str, format: Format) -> Result<(), Failure> {
    if text.len() > 2 * 1024 * 1024 {
        return Err(Failure::Limit);
    }
    let mut quote = None;
    let mut escape = false;
    let mut depth = 0usize;
    for line in text.lines() {
        if matches!(format, Format::Yaml) && line.len() - line.trim_start().len() > 256 {
            return Err(Failure::Limit);
        }
        let mut dots = 0;
        for c in line.chars() {
            if let Some(q) = quote {
                if escape {
                    escape = false;
                } else if c == '\\' && q == '"' {
                    escape = true;
                } else if c == q {
                    quote = None;
                }
                continue;
            }
            if c == '#' {
                break;
            }
            if c == '\'' || c == '"' {
                quote = Some(c);
                continue;
            }
            if matches!(format, Format::Yaml) && matches!(c, '&' | '*' | '!') {
                return Err(Failure::Unsupported);
            }
            if c == '[' || c == '{' {
                depth += 1;
                if depth > 64 {
                    return Err(Failure::Limit);
                }
            } else if c == ']' || c == '}' {
                depth = depth.saturating_sub(1);
            }
            if matches!(format, Format::Toml) {
                if c == '.' {
                    dots += 1;
                    if dots > 64 {
                        return Err(Failure::Limit);
                    }
                }
                if c == '=' {
                    dots = 0;
                }
            }
        }
    }
    Ok(())
}
pub(super) fn decode(text: &str, format: Format) -> Result<Value, Failure> {
    preflight(text, format)?;
    let mut budget = Budget {
        remaining: 50000,
        limited: false,
    };
    let seed = Seed {
        budget: &mut budget,
        depth: 0,
    };
    let result = match format {
        Format::Json => {
            let mut de = serde_json::Deserializer::from_str(text);
            seed.deserialize(&mut de)
                .and_then(|v| {
                    de.end()?;
                    Ok(v)
                })
                .map_err(|_| ())
        }
        Format::Toml => seed
            .deserialize(toml::de::Deserializer::new(text))
            .map_err(|_| ()),
        Format::Yaml => {
            let mut docs = serde_yaml::Deserializer::from_str(text);
            let first = docs.next().ok_or(Failure::Malformed)?;
            let v = seed.deserialize(first).map_err(|_| ());
            if docs.next().is_some() {
                return Err(Failure::Unsupported);
            }
            v
        }
    };
    result.map_err(|_| {
        if budget.limited {
            Failure::Limit
        } else {
            Failure::Malformed
        }
    })
}
