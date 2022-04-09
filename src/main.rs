mod raw_data;

use clap::Parser;

// Program to handle the datasets from 2022 r/place event.
#[derive(Parser)]
struct Cli {
    //pattern: String,
    //#[clap(parse(from_os_str))]
    path: String,
}


fn main() {
    let args = Cli::parse();
    raw_data::read_pixel_history(&args.path);
}
