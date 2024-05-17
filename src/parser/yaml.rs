use super::{NixVariable, NixVariableValue, Parser};
use std::collections::HashMap;
use yaml_rust2::{Yaml, YamlLoader};

pub struct YamlParser {}
impl YamlParser {
    pub fn new() -> YamlParser {
        YamlParser {}
    }
    fn parse_node(node: &Yaml) -> Option<Vec<NixVariable>> {
        match node {
            Yaml::Hash(hashmap) => hashmap
                .iter()
                .map(|f| {
                    Some(NixVariable {
                        name: f.0.to_owned().into_string()?,
                        value: YamlParser::parse_variable(f.1),
                    })
                })
                .collect(),
            _ => None,
        }
    }
    fn parse_variable(variable: &Yaml) -> NixVariableValue {
        match variable {
            Yaml::Real(r) => NixVariableValue::Number(r.to_owned().parse::<f64>().unwrap()),
            Yaml::String(s) => NixVariableValue::String(s.to_owned()),
            Yaml::Integer(i) => NixVariableValue::Number(*i as f64),
            Yaml::Boolean(b) => NixVariableValue::Boolean(*b),
            Yaml::Null => NixVariableValue::Null,
            Yaml::Array(a) => NixVariableValue::List(
                a.into_iter()
                    .map(|f| YamlParser::parse_variable(&f))
                    .collect(),
            ),
            Yaml::Hash(h) => NixVariableValue::AttributeSet(
                h.iter()
                    .map(|(key, value)| {
                        Some((
                            key.to_owned().into_string()?,
                            YamlParser::parse_variable(value),
                        ))
                    })
                    .collect::<Option<HashMap<String, NixVariableValue>>>()
                    .unwrap(),
            ),
            _ => NixVariableValue::Null,
        }
    }
}

impl Parser for YamlParser {
    fn parse(&self, content: &str) -> Option<Vec<super::NixVariable>> {
        YamlParser::parse_node(&YamlLoader::load_from_str(content).ok()?[0])
    }
}

mod tests {
    use std::collections::HashMap;

    use crate::parser::{yaml::YamlParser, NixVariable, NixVariableValue, Parser};

    #[test]
    fn test_yaml() {
        let parser = YamlParser::new();
        let yaml = "
foo:
    bar:
        a: 1
        b: 'test'
this:
    is:
        a:
            float: 0.1
# Comment
            ";
        let expected = vec![
            NixVariable::new(
                "foo",
                &NixVariableValue::AttributeSet(HashMap::from([(
                    "bar".to_string(),
                    NixVariableValue::AttributeSet(HashMap::from([
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
                &NixVariableValue::AttributeSet(HashMap::from([(
                    "is".to_string(),
                    NixVariableValue::AttributeSet(HashMap::from([(
                        "a".to_string(),
                        NixVariableValue::AttributeSet(HashMap::from([(
                            "float".to_string(),
                            NixVariableValue::Number(0.1),
                        )])),
                    )])),
                )])),
            ),
        ];
        let parsed = parser.parse(&yaml);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap(), expected)
    }
}
