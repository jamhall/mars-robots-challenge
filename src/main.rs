extern crate core;

use std::fs;

use structopt::StructOpt;

use crate::error::ApplicationError;
use crate::houston::Houston;
use crate::parser::parse;

mod bounding_box;
mod coordinate;
mod error;
mod houston;
mod instruction;
mod orientation;
mod parser;
mod rover;

#[derive(Debug, StructOpt)]
#[structopt(name = "rover", about = "Martian rovers developer exercise")]
struct Application {
    /// Path to the input file of rover instructions
    #[structopt(short)]
    file: String,
}

fn main() -> Result<(), ApplicationError> {
    let options = Application::from_args();
    println!("Parsing input file: {}", options.file);
    let contents = fs::read_to_string(options.file)?;
    match parse(&contents) {
        Ok((mut houston, rovers)) => {
            for (mut rover, instructions) in rovers {
                for instruction in instructions {
                    rover.execute_instruction(&mut houston, instruction);
                }
                println!("{}", rover);
            }
        },
        Err(error) => {
            eprintln!("Could not parse input file: {}", error);
        },
    }

    Ok(())
}
