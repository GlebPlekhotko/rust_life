mod plaintext;
mod rle;

use life::errors::ErrorCode;
use life::file::*;

#[test]
fn file_not_found() {
    let file_path = "not_found.cells".to_string();
    
    let result = dimensions(&file_path);

    if let Err(ErrorCode::FailedToOpenFile) = result {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn unrecognized_format() {
    let file_path = "tests/file_tests/unrecognized.abc".to_string();
    
    let result = dimensions(&file_path);

    if let Err(ErrorCode::UnrecognizedFileFormat) = result {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn plaintext_five_by_four() {
    let file_path = "tests/file_tests/plaintext.cells".to_string();
    let x : usize;
    let y : usize;
    
    let result = dimensions(&file_path);

    (x, y) = result.unwrap();

    assert_eq!(5, x);
    assert_eq!(4, y);
}

#[test]
fn rle_five_by_four() {
    let file_path = "tests/file_tests/run_length_encoded.rle".to_string();
    let x : usize;
    let y : usize;
    
    let result = dimensions(&file_path);

    (x, y) = result.unwrap();

    assert_eq!(5, x);
    assert_eq!(4, y);
}
