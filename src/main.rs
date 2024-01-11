mod blambda;

#[cfg(test)]
mod tests;

use std::io::Read;
use std::result::Result;

use crate::blambda::parse::Rule;
use crate::blambda::{error::BlambdaError, eval::evaluate_program};
use blambda::parse::BlambdaParser;
use clap::{Arg, Command};
use pest::Parser;
use serde_yaml;

type BlambdaResult<T> = Result<T, BlambdaError>;

fn cli() -> BlambdaResult<()> {
    let matches = Command::new("blambda")
        .bin_name("blambda")
        .version("1.0.0")
        .author("David Sillman")
        .about("Blambda boolean lambda calculus engine, written in Rust.")
        .subcommand_required(true)
        .subcommand(
            Command::new("parse")
                .about("Parse a blambda program, returning the AST")
                // Add any additional arguments or options for the parse command here
                .arg(
                    Arg::new("stdin")
                        .short('s')
                        .help("Whether the input should be parsed from stdin")
                        .required(false)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("input or filepath")
                        .help("The input or file to parse into an AST")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("eval")
                .about("Evaluate a blambda program, returning the result")
                .arg(
                    Arg::new("stdin")
                        .short('s')
                        .help("Whether the input should be parsed from stdin")
                        .required(false)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("input or filepath")
                        .help("The input or file to parse into an AST")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    use crate::blambda::parse::*;
    use crate::blambda::syntax::*;
    use std::fs::File;

    match matches.subcommand() {
        Some(("parse", submatches)) => {
            let from_stdin: bool = *submatches.get_one::<bool>("stdin").unwrap();
            let input: &str = submatches.get_one::<String>("input or filepath").unwrap();

            if !from_stdin {
                let mut file = File::open(input).unwrap();
                let mut contents: String = String::from("");
                file.read_to_string(&mut contents)
                    .map_err::<BlambdaError, _>(|e: std::io::Error| e.into())?;
                let pairs = fallible_parse(Rule::program, &contents)?;
                let program: Program = parse_program(pairs);
                println!("{}", serde_yaml::to_string(&program).unwrap());
                Ok(())
            } else {
                let pairs = fallible_parse(Rule::program, input)?;
                let program: Program = parse_program(pairs);
                println!("{}", serde_yaml::to_string(&program).unwrap());
                Ok(())
            }
        }
        Some(("eval", submatches)) => {
            let from_stdin: bool = *submatches.get_one::<bool>("stdin").unwrap();
            let input: &str = submatches.get_one::<String>("input or filepath").unwrap();

            if !from_stdin {
                let mut file = File::open(input).unwrap();
                let mut contents: String = String::from("");
                file.read_to_string(&mut contents)
                    .map_err::<BlambdaError, _>(|e: std::io::Error| e.into())?;
                let pairs = fallible_parse(Rule::program, &contents)?;
                let program: Program = parse_program(pairs);
                if let Some(value) = evaluate_program(program) {
                    println!("{}", value);
                } else {
                    println!("Error: program could not be evaluated");
                }
                Ok(())
            } else {
                let pairs = fallible_parse(Rule::program, input)?;
                let program: Program = parse_program(pairs);
                if let Some(value) = evaluate_program(program) {
                    println!("{}", value);
                } else {
                    println!("Error: program could not be evaluated");
                }
                Ok(())
            }
        }
        Some((_, _)) => unreachable!(),
        None => unreachable!(),
    }
}

fn fallible_parse(rule: Rule, input: &str) -> Result<pest::iterators::Pairs<Rule>, BlambdaError> {
    BlambdaParser::parse(rule, &input).map_err(|e| e.into())
}

fn main() {
    match cli() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
