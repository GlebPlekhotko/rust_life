pub mod plaintext;
mod rle;

use crate::errors::ErrorCode;
use std::fs::File;
use std::io;
use std::io::Read;

enum Formats {
    PlainText,
    Rle,
    Unknown,
}


/// Returns a tuple containing dimensiont of the filed in the given file

pub fn dimensions(path : &String) -> Result<(usize, usize), ErrorCode>
{
    match File::open(&path) {
        Ok(mut file) => {
            let mut content = String::new();

            if let Err(_) = file.read_to_string(&mut content) {
                return Err(ErrorCode::FailedToReadFile)
            }
        
            match deduce(path) {
                Formats::PlainText => plaintext::dimensions(&content),
                Formats::Rle => rle::dimensions(&content),
                _ => return Err(ErrorCode::UnrecognizedFileFormat)
            }
        },
        Err(_) => Err(ErrorCode::FailedToOpenFile)
    }
}

/// Deduce the type of the file by its extension, ".cells" and .rle" for the plaintext and run length encoded

fn deduce(path : &String) -> Formats {
    let mut format = Formats::Unknown;

    for extension in path.split('.').rev() {
        format = match extension {
            "cells" => Formats::PlainText,
            "rle" => Formats::Rle,
            _ => Formats::Unknown
        };

        break;
    }

    format
}


/// Attempts to load (setup) the population using the given file

pub fn load(path : String, population : Vec<Vec<bool>>) -> Result<i32, ErrorCode>
{
    Result::Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plaintext_simple_path() {
        let format = deduce(&"test.cells".to_string());

        assert!(if let Formats::PlainText = format { true } else { false });
    }

    #[test]
    fn plaintext_complex_path() {
        let format = deduce(&"dir1/dir2/test.cells".to_string());

        assert!(if let Formats::PlainText = format { true } else { false });
    }

    #[test]
    fn rle_simple_path() {
        let format = deduce(&"test.rle".to_string());

        assert!(if let Formats::Rle = format { true } else { false });
    }

    #[test]
    fn rle_complex_path() {
        let format = deduce(&"dir1/dir2/test.rle".to_string());

        assert!(if let Formats::Rle = format { true } else { false });
    }

    #[test]
    fn unknown_simple_path() {
        let format = deduce(&"test.bla".to_string());

        assert!(if let Formats::Unknown = format { true } else { false });
    }

    #[test]
    fn unknown_complex_path() {
        let format = deduce(&"dir1/dir2/test.bla".to_string());

        assert!(if let Formats::Unknown = format { true } else { false });
    }

    #[test]
    fn unknown_no_extenstion_simple_path() {
        let format = deduce(&"test_cells".to_string());

        assert!(if let Formats::Unknown = format { true } else { false });
    }

    #[test]
    fn unknown_no_extenstion_complex_path() {
        let format = deduce(&"dir1/dir2/test_cells".to_string());

        assert!(if let Formats::Unknown = format { true } else { false });
    }
}
