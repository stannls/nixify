use serde_json::Value;

use crate::parser::NixVariable;

use super::{NixVariableValue, Parser};

pub struct JsonParser {}
impl JsonParser {
    pub fn new() -> JsonParser {
        JsonParser {}
    }
}

impl JsonParser {
    pub fn parse_value(&self, value: Value) -> NixVariableValue {
        if value.is_i64() {
            NixVariableValue::Number(value.as_i64().unwrap() as f64)
        } else if value.is_u64() {
            NixVariableValue::Number(value.as_u64().unwrap() as f64)
        } else if value.is_f64() {
            NixVariableValue::Number(value.as_f64().unwrap())
        } else if value.is_null() {
            NixVariableValue::Null
        } else if value.is_array() {
            NixVariableValue::List(
                value
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|value| self.parse_value(value.to_owned()))
                    .collect(),
            )
        } else if value.is_object() {
            NixVariableValue::AttributeSet(
                value
                    .as_object()
                    .unwrap()
                    .into_iter()
                    .map(|(key, value)| (key.to_owned(), self.parse_value(value.to_owned())))
                    .collect(),
            )
        } else if value.is_string() {
            NixVariableValue::String(value.as_str().unwrap().to_string())
        } else if value.is_boolean() {
            NixVariableValue::Boolean(value.as_bool().unwrap())
        } else {
            NixVariableValue::Null
        }
    }
}

impl Parser for JsonParser {
    fn parse(&self, content: &str) -> Option<Vec<super::NixVariable>> {
        let parsed = serde_json::from_str::<Value>(content).ok()?;
        let parsed_object = parsed.as_object()?;
        Some(
            parsed_object
                .into_iter()
                .map(|(key, value)| {
                    NixVariable::new(key.to_owned(), self.parse_value(value.to_owned()))
                })
                .collect(),
        )
    }
}
