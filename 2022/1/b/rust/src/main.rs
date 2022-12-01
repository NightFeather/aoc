use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Error {
    MissingArgument,
    FailedToOpenFile(io::Error),
    IOError(io::Error),
    ParseError(std::num::ParseIntError),
}


fn main() -> Result<(), Error> {
    let arg = env::args().skip(1).next();
    if let Some(filename) = arg {
        let f = File::open(filename).map_err(|err: io::Error| Error::FailedToOpenFile(err))?;
        let reader = BufReader::new(f);
        let mut result : Vec<u32> = vec![0];
        for line in reader.lines() {
            let data = line.map_err(|err : io::Error| Error::IOError(err))?;
            if data.is_empty() {
                result.push(0);
            } else {
                if let Some(v) = result.last_mut() {
                    *v += data.parse::<u32>().map_err(|err| Error::ParseError(err))?;
                }
            }
        }
        result.sort();
        println!("{:?}", result.iter().rev().take(3).sum::<u32>());
        Ok(())
    } else {
        Err(Error::MissingArgument)
    }
}
