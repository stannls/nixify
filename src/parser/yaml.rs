use super::{NixVariable, NixVariableValue, Parser};
use indexmap::IndexMap;
use yaml_rust2::{Yaml, YamlLoader};

pub struct YamlParser {}
impl Default for YamlParser {
    fn default() -> Self {
        Self::new()
    }
}

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
                a.iter()
                    .map(YamlParser::parse_variable)
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
                    .collect::<Option<IndexMap<String, NixVariableValue>>>()
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

#[cfg(test)]
mod tests {
    use crate::parser::{yaml::YamlParser, NixVariable, NixVariableValue, Parser};
    use indexmap::IndexMap;
    use lazy_static::lazy_static;

    #[test]
    fn test_yaml() {
        let parser = YamlParser::new();

        let parsed = parser.parse(YAML);
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
    const YAML: &str = "
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
}
