mod plaintext;
mod rle;

use life::errors::ErrorCode;
use life::file::*;

#[test]
fn file_not_found() {
    let file_path = "not_found.cells".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    let result = load(&mut field, &file_path);

    if let Err(ErrorCode::FailedToOpenFile) = result {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn unrecognized_format() {

    let file_path = "tests/file_tests/unrecognized.abc".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    let result = load(&mut field, &file_path);

    if let Err(ErrorCode::UnrecognizedFileFormat) = result {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn plaintext_five_by_four() {
    let file_path = "tests/file_tests/plaintext.cells".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    let (x, y) = dimensions(&file_path).unwrap();
    for row in 0..x {
        field.push(Vec::new());
        for _column in 0..y {
            field[row].push(false);
        }
    }

    let result = load(&mut field, &file_path);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(false, field[2][0]); assert_eq!(false, field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(false, field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(true,  field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(false, field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(false, field[3][2]); assert_eq!(false, field[4][2]);
    assert_eq!(false, field[0][3]); assert_eq!(false, field[1][3]); assert_eq!(false, field[2][3]); assert_eq!(false, field[3][3]); assert_eq!(false, field[4][3]);
}

#[test]
fn rle_five_by_four() {
    let file_path = "tests/file_tests/run_length_encoded.rle".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    let (x, y) = dimensions(&file_path).unwrap();
    for row in 0..x {
        field.push(Vec::new());
        for _column in 0..y {
            field[row].push(false);
        }
    }

    let result = load(&mut field, &file_path);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(false, field[2][0]); assert_eq!(false, field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(false, field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(true,  field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(false, field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(false, field[3][2]); assert_eq!(false, field[4][2]);
    assert_eq!(false, field[0][3]); assert_eq!(false, field[1][3]); assert_eq!(false, field[2][3]); assert_eq!(false, field[3][3]); assert_eq!(false, field[4][3]);
}
