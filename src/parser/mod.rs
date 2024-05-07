use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum SupportedFormats {
    yaml,
    toml,
    json
}
