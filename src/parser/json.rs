use super::{NixVariableValue, Parser};
use crate::parser::NixVariable;
use serde_json::Value;

pub struct JsonParser {}
impl Default for JsonParser {
    fn default() -> Self {
        Self::new()
    }
}

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
                    .iter()
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
                .map(|(key, value)| NixVariable::new(key, &self.parse_value(value.to_owned())))
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{json::JsonParser, NixVariable, NixVariableValue, Parser};
    use indexmap::IndexMap;
    use lazy_static::lazy_static;

    #[test]
    fn test_json() {
        let parser = JsonParser::new();

        let parsed = parser.parse(JSON);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap(), *EXPECTED)
    }
    lazy_static! {
        pub static ref EXPECTED: Vec<NixVariable> = vec![
            NixVariable::new(
                "foo",
                &NixVariableValue::AttributeSet(IndexMap::from([(
                    "bar".to_string(),
                    NixVariableValue::AttributeSet(IndexMap::from([
                        ("a".to_string(), NixVariableValue::Number(1.0)),
                        (
                            "b".to_string(),
                            NixVariableValue::String("test".to_string()),
                        ),
                    ])),
                )])),
            ),
            NixVariable::new(
                "this",
                &NixVariableValue::AttributeSet(IndexMap::from([(
                    "is".to_string(),
                    NixVariableValue::AttributeSet(IndexMap::from([(
                        "a".to_string(),
                        NixVariableValue::AttributeSet(IndexMap::from([(
                            "float".to_string(),
                            NixVariableValue::Number(0.1),
                        )])),
                    )])),
                )])),
            ),
        ];
    }
    const JSON: &str = "
{
    \"foo\": {
        \"bar\": {
            \"a\": 1,
            \"b\": \"test\"
        }
    },
    \"this\": {
        \"is\": {
            \"a\" : {
                \"float\": 0.1
            }
        }
    }
}
";
}
