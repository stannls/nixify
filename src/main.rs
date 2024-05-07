use clap::{arg, command, value_parser};
mod parser;
use parser::SupportedFormats;


fn main() {
    let matches = command!()
        .name("nixify")
        .version("0.1.0")
        .about("A CLI tool to turn existing configurations into nix syntax.")
        .arg(
            arg!(<FILE>)
                .required(true)
                .id("file")
                .help("The file to convert."),
        )
        .arg(arg!(--"format" <FORMAT>) 
                .short('f')
                .long("format")
                .required(true)
                .id("format")
                .help("The format of the file to convert.")
                .value_parser(value_parser!(SupportedFormats))
        )
        .arg(arg!(--"name" <NAME>)
                .short('n')
                .long("name")
                .required(true)
                .id("name")
                .help("The name of the program in the nix expression."),
        ).get_matches();

}
