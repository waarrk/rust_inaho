mod file_controller;
extern crate csv;

use std::process;

fn main() {
    let file_path = match file_controller::args::get_first() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let ndarray = match file_controller::file_io::csv_to_ndarray(
        file_controller::file_io::fopen(&file_path).unwrap(),
    ) {
        Ok(ndarray) => ndarray,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    println!("{:?}", ndarray);
}
