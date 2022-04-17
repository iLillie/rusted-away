use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub trait DataParser {
    type Data;

    fn from_file(file_path: String) -> Result<Vec<Self::Data>, io::Error> {
        let file = File::open(&file_path).map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;
        let buff_reader = BufReader::new(file);
        let mut data_vec: Vec<Self::Data> = Vec::new();
        for reader_line in buff_reader.lines() {
            let line = reader_line?;
            let value_to_push: Self::Data = match Self::from_line(line) {
                Ok(data_file ) => data_file,
                Err(e) => {
                    continue
                }
            };
            data_vec.push(value_to_push);
        }
        Ok(data_vec)
    }

    fn from_line(line: String) -> Result<Self::Data, io::Error>;
}