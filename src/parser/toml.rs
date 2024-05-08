use toml::{Table, Value};

use super::{NixVariable, NixVariableValue, Parser};

#[derive(Debug, Clone)]
pub struct TomlParser {}

impl TomlParser {
    pub fn new() -> TomlParser {
        TomlParser {}
    }

    fn parse_value(value: &Value) -> NixVariableValue {
        match value {
            Value::String(s) => NixVariableValue::String(s.to_owned()),
            Value::Integer(i) => NixVariableValue::Number(*i as f64),
            Value::Float(f) => NixVariableValue::Number(*f),
            Value::Boolean(b) => NixVariableValue::Boolean(*b),
            Value::Datetime(d) => NixVariableValue::String(d.to_string()),
            Value::Array(a) => {
                NixVariableValue::List(a.into_iter().map(|f| TomlParser::parse_value(f)).collect())
            }
            Value::Table(m) => NixVariableValue::AttributeSet(
                m.into_iter()
                    .map(|(key, value)| (key.to_owned(), TomlParser::parse_value(value)))
                    .collect(),
            ),
        }
    }
}
impl Parser for TomlParser {
    fn parse(&self, content: &str) -> Option<Vec<super::NixVariable>> {
        Some(
            content
                .parse::<Table>()
                .ok()?
                .iter()
                .map(|(name, value)| NixVariable {
                    name: name.to_owned(),
                    value: TomlParser::parse_value(value),
                })
                .collect(),
        )
    }
}
