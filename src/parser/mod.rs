use std::{collections::HashMap, path::Path};
pub mod toml;
pub mod yaml;

use clap::ValueEnum;

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
    pub fn new(name: String, value: NixVariableValue) -> NixVariable {
        NixVariable { name, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum NixVariableValue {
    String(String),
    Number(f64),
    Path(Box<Path>),
    Boolean(bool),
    Null,
    List(Vec<NixVariableValue>),
    AttributeSet(HashMap<String, NixVariableValue>),
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
                    .map(
                        |(key, value)| NixVariable::new(key.to_owned(), value.to_owned())
                            .to_string()
                    )
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
    parsers: HashMap<SupportedFormats, Box<dyn 'static + Parser>>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            parsers: HashMap::new(),
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

pub struct ExpressionGenerator {}

impl ExpressionGenerator {
    pub fn new() -> ExpressionGenerator {
        ExpressionGenerator {}
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
    }
}
