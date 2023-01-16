extern crate ndarray;
extern crate rust_inaho;

use rust_inaho::file_controller::args;
use rust_inaho::file_controller::file_io;

use std::ffi::OsString;
use std::process;

fn matrix_product(
    matrix1: ndarray::Array2<f64>,
    matrix2: ndarray::Array2<f64>,
) -> ndarray::Array2<f64> {
    matrix1.dot(&matrix2)
}

fn main() {
    let mut file_path: [OsString; 2] = [OsString::from(""), OsString::from("")];
    let mut matrix: [ndarray::Array2<f64>; 2] = [
        ndarray::Array2::zeros((0, 0)),
        ndarray::Array2::zeros((0, 0)),
    ];

    args::get_all()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, arg)| {
            file_path[i] = arg.clone();
        });

    println!("Input file paths:");
    println!("{:?}", file_path);

    println!("Input matrixes:");
    for i in 0..2 {
        matrix[i] = match file_io::csv_to_ndarray(file_io::fopen(&file_path[i]).unwrap()) {
            Ok(matrix) => matrix,
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };
    }

    for i in 0..2 {
        println!("{}", matrix[i]);
    }

    println!("Output:");
    let answer = matrix_product(matrix[0].clone(), matrix[1].clone());
    println!("{}", answer);
}
