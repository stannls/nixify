use std::{collections::HashMap, path::Path};
pub mod toml;

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

#[derive(Debug, Clone, PartialEq)]
pub enum NixVariableValue {
    String(String),
    Number(f64),
    Path(Box<Path>),
    Boolean(bool),
    Null,
    List(Vec<NixVariableValue>),
    AttributeSet(HashMap<String, NixVariableValue>),
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
