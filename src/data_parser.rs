use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub trait DataParser {
    type Data;

    fn from_file(file_path: String) -> Result<Vec<Self::Data>, io::Error> {
        let file = File::open(&file_path).map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;
        let buff_reader = BufReader::new(file);
        let mut data_vec = Vec::new();
        for reader_line in buff_reader.lines() {
            let line = reader_line?;
            data_vec.push(Self::from_line(line)?);
        }
        Ok(data_vec)
    }

    fn from_line(line: String) -> Result<Self::Data, io::Error>;
}