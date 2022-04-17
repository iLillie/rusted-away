mod utilities;
mod data_parser;
mod raw_data_parser;
mod details_parser;
mod reddit_parser;

use clap::Parser;
use crate::data_parser::DataParser;
use crate::reddit_parser::RedditDataParser;

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

    match reddit_parser::RedditDataParser::from_file(args.path) {
        Ok(parsed_data) => {
            for data in parsed_data.iter() {
                println!("{:?}", data);
            }
        }
        Err(e) => {

            println!("{:?}", e);
        }
    }

}
