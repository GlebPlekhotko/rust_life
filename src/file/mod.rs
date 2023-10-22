pub mod gif;
pub mod plaintext;
pub mod rle;

use crate::errors::ErrorCode;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

enum Formats {
    PlainText,
    Rle,
    Gif,
    Unknown,
}

/// Appends a number to the and of the file path

/**
    Given function is useful in the cases when multiple consequetive generations to be saved to the distinct files. 
    For instance, output_0.cells, output_1.cells, output_3.cells and so on.
*/
pub fn append_number(old_path : &String, number : u32) -> Result<String, ErrorCode> {
    let mut new_path : String;
    let path : &str;
    let file : &str;
    let name : &str;
    let extension : &str;

    if old_path == "" {
        return Err(ErrorCode::EmptyPath);
    }

    match old_path.rsplit_once('/') {
        Some((left, right)) => {
            path = left;
            file = right;
        },
        None => {
            path = "";
            file = old_path.as_str();
        }
    }

    match file.rsplit_once('.') {
        Some((left, right)) => {
            name = left;
            extension = right;
            if extension == "" {
                return Err(ErrorCode::NoFileExtension);    
            }
        },
        None => {
            return Err(ErrorCode::NoFileExtension);
        }
    }

    new_path = String::from(path);
    if new_path != "" {
        new_path += "/";
    }
    new_path += name;
    new_path += "_";
    new_path += &number.to_string();
    new_path += ".";
    new_path += extension;

    return Ok(new_path);
}

/// Returns a tuple containing dimensions of the filed in the given file

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
            "gif" => Formats::Gif,
            _ => Formats::Unknown
        };

        break;
    }

    format
}


/// Attempts to load (setup) the population using the given file

pub fn load(population : &mut Vec<Vec<bool>>, path : &String) -> Result<(), ErrorCode>
{
    match File::open(&path) {
        Ok(mut file) => {
            let mut content = String::new();

            if let Err(_) = file.read_to_string(&mut content) {
                return Err(ErrorCode::FailedToReadFile)
            }
        
            match deduce(path) {
                Formats::PlainText => plaintext::load(population, &content),
                Formats::Rle => rle::load(population, &content),
                _ => return Err(ErrorCode::UnrecognizedFileFormat)
            }
        },
        Err(_) => Err(ErrorCode::FailedToOpenFile)
    }
}

/// Saves the given population to the destination file

pub fn save(population : &Vec<Vec<bool>>, path : &String) -> Result<(), ErrorCode>
{
    let mut content = String::new();

    if let Some((dir, _)) = path.rsplit_once("/") {
        if let Err(_) = fs::create_dir_all(dir) {
            return Err(ErrorCode::FailedToCreateDirectory);
        }
    }

    let mut file = match File::create(path) {
        Ok(handle) => handle,
        _ => return Err(ErrorCode::FailedToCreateFile),
    };

    match deduce(path) {
        Formats::PlainText => {
            plaintext::save(population, &mut content)?;
            if let Err(_) = file.write(content.as_bytes()) {
                return Err(ErrorCode::FailedToWriteFile);
            }
        },
        Formats::Rle => {
            rle::save(population, &mut content)?;
            if let Err(_) = file.write(content.as_bytes()) {
                return Err(ErrorCode::FailedToWriteFile);
            }
        },
        Formats::Gif => {
            gif::save(&file, population)?;
        },
        _ => {
            return Err(ErrorCode::UnrecognizedFileFormat);
        }
    };
    

    Ok(())
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
    fn gif_simple_path() {
        let format = deduce(&"test.gif".to_string());

        assert!(if let Formats::Gif = format { true } else { false });
    }

    #[test]
    fn gif_complex_path() {
        let format = deduce(&"dir1/dir2/test.gif".to_string());

        assert!(if let Formats::Gif = format { true } else { false });
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
