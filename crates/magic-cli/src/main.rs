use magic_parser::{Parser, Parsers};

use clap::Parser as ClapParser;
use std::fs;

#[derive(Debug, ClapParser)]
#[clap(name = "Magic Language Compiler")]
struct Args {
    #[clap(short, long)]
    program_file: String,
}

fn main() {
    let args: Args = Args::parse();

    let program_text = fs::read_to_string(&args.program_file).expect("Unable to read program file");

    /*let program = magic_parser::ProgramParser::new()
    .parse(&program_text)
    .expect("Unable to parse the program file");*/

    let parser = Parsers::default();
    let program = parser
        .parse(&program_text)
        .map_err(|err| println!("{}", err.with_code(&program_text, &args.program_file)));

    println!("{:?}", program);
}
