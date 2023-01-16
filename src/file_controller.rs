#[allow(dead_code)]
pub mod args {
    use std::env;
    use std::error::Error;
    use std::ffi::OsString;

    pub fn get(num: usize) -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(num) {
            None => Err(From::from("expected argument, but got none")),
            Some(file_path) => Ok(file_path),
        }
    }

    pub fn get_all() -> Result<Vec<OsString>, Box<dyn Error>> {
        let mut args = env::args_os();
        args.next();
        let mut file_paths = Vec::new();
        for arg in args {
            file_paths.push(arg);
        }
        Ok(file_paths)
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod file_io {
    use ndarray::{arr2, Array, Array2};
    use std::error::Error;
    use std::ffi::OsString;
    use std::fs::File;

    pub fn fopen(path: &OsString) -> Result<File, Box<dyn Error>> {
        let file = File::open(path)?;
        Ok(file)
    }

    pub fn csv_to_ndarray(file: File) -> Result<Array2<f64>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        let mut data = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let row: Vec<f64> = record.iter().map(|s| s.parse().unwrap()).collect();
            data.push(row);
        }

        let ndarray = Array::from_shape_vec(
            (data.len(), data[0].len()),
            data.into_iter().flatten().collect(),
        )?;
        Ok(ndarray)
    }

    #[test]
    fn csv_to_ndarray_test() {
        let file = File::open("datas/confirmation/k2-input1.csv").unwrap();
        let expected_output: Array2<f64> =
            arr2(&[[2.0, 8.0, 4.0], [3.0, 2.0, -1.0], [7.0, -1.0, 3.0]]);
        let result = csv_to_ndarray(file);
        let result = result.unwrap();
        assert_eq!(result, expected_output);
    }
}
