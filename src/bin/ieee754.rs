extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[test]
fn ieee745形式() {
    let file = File::open("datas/confirmation/k1-input.csv").unwrap();
    let expected_output = 0b0100000001001010101100000000000000000000000000000000000000000000;
    let result = csv_parse(file);
    let result = result.unwrap();
    let num: f64 = result.parse().unwrap();
    let bits = num.to_bits();
    assert_eq!(bits, expected_output);
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn fopen(path: &OsString) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(file)
}

fn csv_parse(file: File) -> Result<String, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let first_line = rdr.records().next();
    match first_line {
        Some(Ok(record)) => Ok(record[0].to_owned()),
        Some(Err(err)) => Err(From::from(err)),
        None => Err(From::from("No data found in CSV file")),
    }
}

fn main() {
    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let data = match csv_parse(fopen(&file_path).unwrap()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let num: f64 = data.parse().unwrap();
    let bits = num.to_bits();
    let bytes = bits.to_le_bytes();

    println!("{}", num);
    println!("sEEEEEEE EEEEdddd dddddddd...");

    for byte in (bytes.iter()).rev() {
        print!("{:08b} ", byte);
    }
}
