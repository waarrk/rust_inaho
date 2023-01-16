extern crate ndarray;
extern crate rust_inaho;

use rust_inaho::file_controller::args;
use rust_inaho::file_controller::file_io;

use std::process;

fn main() {
    let file_path = match args::get_first() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let ndarray = match file_io::csv_to_ndarray(file_io::fopen(&file_path).unwrap()) {
        Ok(ndarray) => ndarray,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    println!("{:?}", ndarray);
}
