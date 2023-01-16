pub mod args {
    use std::env;
    use std::error::Error;
    use std::ffi::OsString;

    pub fn get_first() -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(1) {
            None => Err(From::from("expected 1 argument, but got none")),
            Some(file_path) => Ok(file_path),
        }
    }
}

pub mod file_io {
    use ndarray::{Array, Array2};
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
}
