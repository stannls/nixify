use super::{NixVariable, NixVariableValue, Parser};
use toml::{Table, Value};

#[derive(Debug, Clone)]
pub struct TomlParser {}

impl Default for TomlParser {
    fn default() -> Self {
        Self::new()
    }
}

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
                NixVariableValue::List(a.iter().map(TomlParser::parse_value).collect())
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

#[cfg(test)]
mod test {
    use crate::parser::{toml::TomlParser, NixVariable, NixVariableValue, Parser};
    use indexmap::IndexMap;
    use lazy_static::lazy_static;

    #[test]
    fn test_toml() {
        let parser = TomlParser::new();
        let parsed = parser.parse(TOML);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap(), *EXPECTED)
    }
    const TOML: &str = "
[foo.bar]
a = 1
b = \"test\"
[this.is.a]
float = 0.1
# A comment
            ";
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
}
