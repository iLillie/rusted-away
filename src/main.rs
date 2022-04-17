mod reddit_data;
mod utilities;
mod data_parser;
mod raw_data_parser;

use clap::Parser;
use crate::data_parser::DataParser;

// Program to handle the datasets from 2022 r/place event.
#[derive(Parser)]
struct Cli {
    //pattern: String,
    //#[clap(parse(from_os_str))]
    path: String,
}


fn main() {
    let args = Cli::parse();
    println!("{}", args.path);
    match raw_data_parser::RawDataParser::parse_file(args.path) {
        Ok(_) => {
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
