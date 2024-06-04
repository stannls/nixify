use std::{fs, path::PathBuf};

use clap::{arg, command, value_parser, ArgMatches};
use nixify::parser::{
    json::JsonParser, toml::TomlParser, yaml::YamlParser, ExpressionGenerator, ExpressionParser,
    SupportedFormats,
};

fn main() {
    // Disable verbose panic for release mode and send error to stderr
    #[cfg(not(debug_assertions))]
    use std::panic;
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("{s:?}");
        } else {
            eprintln!("An error ocurred.");
        }
    }));

    let matches = command!()
        .name("nixify")
        .version("0.1.1")
        .about("A CLI tool to turn existing configurations into nix syntax.")
        .arg(
            arg!(<FILE>)
                .required(true)
                .id("file")
                .help("The file to convert.")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(
            arg!(--"format" <FORMAT>)
                .short('f')
                .long("format")
                .required(true)
                .id("format")
                .help("The format of the file to convert.")
                .value_parser(value_parser!(SupportedFormats)),
        )
        .arg(
            arg!(--"name" <NAME>)
                .short('n')
                .long("name")
                .required(true)
                .id("name")
                .help("The name of the program in the nix expression."),
        )
        .get_matches();
    handle_matches(matches);
}

fn handle_matches(matches: ArgMatches) {
    // Build a new ExpressionParser
    let expression_parser = ExpressionParser::new()
        .add_parser(SupportedFormats::toml, Box::new(TomlParser::new()))
        .unwrap()
        .add_parser(SupportedFormats::yaml, Box::new(YamlParser::new()))
        .unwrap()
        .add_parser(SupportedFormats::json, Box::new(JsonParser::new()))
        .unwrap();
    let expression_generator = ExpressionGenerator::new().with_formatting();

    // Get arguments from clap
    let filepath: &PathBuf = matches.get_one("file").unwrap();
    let format: SupportedFormats = *matches.get_one("format").unwrap();
    let name: &String = matches.get_one("name").unwrap();

    // Parse the file
    let content = fs::read_to_string(filepath).expect("Error reading given file");
    let parsed = expression_parser
        .parse(&content, Some(format))
        .expect("Failed parsing the given file");
    let expression = expression_generator
        .generate_nix_expression(name, &parsed)
        .unwrap();
    println!("{}", expression);
}
