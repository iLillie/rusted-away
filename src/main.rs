mod reddit_data;
mod utilities;
mod data_parser;
mod raw_data_parser;
mod details_parser;

use clap::Parser;
use crate::data_parser::DataParser;
use crate::details_parser::DetailDataParser;

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

    match details_parser::DetailDataParser::from_file(args.path) {
        Ok(raw_data) => {
            for r_data in raw_data.iter() {
                println!("{:?}", r_data);
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

}
