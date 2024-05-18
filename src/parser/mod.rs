use std::path::PathBuf;
pub mod json;
pub mod toml;
pub mod yaml;

use clap::ValueEnum;
use indexmap::IndexMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum SupportedFormats {
    yaml,
    toml,
    json,
}

pub trait Parser {
    fn parse(&self, content: &str) -> Option<Vec<NixVariable>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct NixVariable {
    pub name: String,
    pub value: NixVariableValue,
}

impl NixVariable {
    pub fn to_string(&self) -> String {
        format!("{} = {};\n", self.name, self.value.to_string())
    }
    pub fn new(name: &str, value: &NixVariableValue) -> NixVariable {
        NixVariable {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum NixVariableValue {
    String(String),
    Number(f64),
    Path(Box<PathBuf>),
    Boolean(bool),
    Null,
    List(Vec<NixVariableValue>),
    AttributeSet(IndexMap<String, NixVariableValue>),
}

impl NixVariableValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::String(s) => format!("\"{}\"", s),
            Self::Path(p) => format!("{}", p.to_str().expect("Error parsing file.")),
            Self::Boolean(b) => format!("{}", b),
            Self::Null => "null".to_string(),
            Self::AttributeSet(a) => format!(
                "{{\n{}}}",
                a.into_iter()
                    .map(|(key, value)| NixVariable::new(key, value).to_string())
                    .reduce(|acc, e| acc + &e)
                    .expect("Error parsing file")
            ),
            Self::List(l) => format!(
                "[\n{}\n]",
                l.into_iter()
                    .map(|f| f.to_string())
                    .reduce(|acc, e| acc + "\n" + &e)
                    .unwrap()
            ),
        }
    }
}

pub struct ExpressionParser {
    parsers: IndexMap<SupportedFormats, Box<dyn 'static + Parser>>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            parsers: IndexMap::new(),
        }
    }

    pub fn add_parser(
        mut self,
        format: SupportedFormats,
        parser: Box<dyn Parser>,
    ) -> Option<ExpressionParser> {
        if self.parsers.contains_key(&format) {
            None
        } else {
            self.parsers.insert(format, parser);
            Some(self)
        }
    }

    pub fn parse(&self, content: &str, format: SupportedFormats) -> Option<Vec<NixVariable>> {
        if !self.parsers.contains_key(&format) {
            None
        } else {
            self.parsers[&format].parse(content)
        }
    }
}

pub struct ExpressionGenerator {
    formatting: bool,
}

impl ExpressionGenerator {
    pub fn new() -> ExpressionGenerator {
        ExpressionGenerator { formatting: false }
    }
    pub fn with_formatting(mut self) -> ExpressionGenerator {
        self.formatting = true;
        self
    }
    pub fn generate_nix_expression(&self, name: &str, values: &Vec<NixVariable>) -> Option<String> {
        vec![
            "{ config, pkgs, ... }:\n".to_string(),
            "{\n".to_string(),
            format!("programs.{}.enable = true;\n", name),
        ]
        .into_iter()
        .chain(values.into_iter().map(|v| v.to_string()))
        .chain(vec!["};\n".to_string(), "}".to_string()])
        .reduce(|acc, e| format!("{acc}{e}"))
        .map(|expression| {
            if self.formatting {
                nixpkgs_fmt::reformat_string(&expression)
            } else {
                expression
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpressionGenerator, NixVariable, NixVariableValue};
    use indexmap::IndexMap;
    use std::path::Path;

    #[test]
    fn test_expression_generator() {
        let expression_generator = ExpressionGenerator::new();
        let expression = vec![
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
        let expected = "{ config, pkgs, ... }:\n{\nprograms.test.enable = true;\nfoo = {\nbar = {\na = 1;\nb = \"test\";\n};\n};\nthis = {\nis = {\na = {\nfloat = 0.1;\n};\n};\n};\n};\n}";
        let generated = expression_generator.generate_nix_expression("test", &expression);
        assert!(generated.is_some());
        assert_eq!(generated.unwrap(), expected);
    }

    #[test]
    fn test_expression_generator_formatted() {
        let expression_generator = ExpressionGenerator::new().with_formatting();
        let expression = vec![
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
        let expected = "{ config, pkgs, ... }:\n{\n  programs.test.enable = true;\n  foo = {\n    bar = {\n      a = 1;\n      b = \"test\";\n    };\n  };\n  this = {\n    is = {\n      a = {\n        float = 0.1;\n      };\n    };\n  };\n};\n}\n";
        let generated = expression_generator.generate_nix_expression("test", &expression);
        assert!(generated.is_some());
        assert_eq!(generated.unwrap(), expected);
    }

    #[test]
    fn test_variable_conversion() {
        let number = NixVariable::new("number", &NixVariableValue::Number(4.2));
        let string = NixVariable::new("string", &NixVariableValue::String("foobar".to_string()));
        let path = NixVariable::new(
            "path",
            &NixVariableValue::Path(Box::new(Path::new("/tmp/foo").to_path_buf())),
        );
        let bool = NixVariable::new("bool", &NixVariableValue::Boolean(true));
        let null = NixVariable::new("null", &NixVariableValue::Null);
        let list = NixVariable::new(
            "list",
            &NixVariableValue::List(vec![
                NixVariableValue::Number(4.2),
                NixVariableValue::Number(6.9),
            ]),
        );
        let attrset = NixVariable::new(
            "attrset",
            &NixVariableValue::AttributeSet(IndexMap::from([(
                "foo".to_string(),
                NixVariableValue::String("bar".to_string()),
            )])),
        );

        assert_eq!(number.to_string(), "number = 4.2;\n");
        assert_eq!(string.to_string(), "string = \"foobar\";\n");
        assert_eq!(path.to_string(), "path = /tmp/foo;\n");
        assert_eq!(bool.to_string(), "bool = true;\n");
        assert_eq!(null.to_string(), "null = null;\n");
        assert_eq!(list.to_string(), "list = [\n4.2\n6.9\n];\n");
        assert_eq!(attrset.to_string(), "attrset = {\nfoo = \"bar\";\n};\n");
    }
}
