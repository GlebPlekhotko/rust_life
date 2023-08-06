mod plaintext;
mod rle;

use std::fs::*;
use life::errors::ErrorCode;
use life::file::*;

#[test]
fn unrecognized_format() {

    let file_path = "tests/file_tests/unrecognized.abc".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    let result = save(&mut field, &file_path);

    if let Err(ErrorCode::UnrecognizedFileFormat) = result {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn plaintext_file() {

    let file_path = "tests/file_tests/test.cells".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for cell in 0..4 {
            field[row].push(false);
        }
    }
    field[1][0] = true;
    field[2][1] = true;
    field[0][2] = true; field[1][2] = true; field[2][2] = true;
        
    let result = save(&mut field, &file_path);

    if let Ok(_) = result { assert!(true) } else { assert!(false) };
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), 
               ".O...\r\n\
                ..O..\r\n\
                OOO..\r\n\
                .....\r\n".to_string());

    std::fs::remove_file(&file_path);
}

#[test]
fn rle_file() {

    let file_path = "tests/file_tests/test.rle".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for cell in 0..4 {
            field[row].push(false);
        }
    }
    field[1][0] = true;
    field[2][1] = true;
    field[0][2] = true; field[1][2] = true; field[2][2] = true;
        
    let result = save(&mut field, &file_path);

    if let Ok(_) = result { assert!(true) } else { assert!(false) };
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), 
               "x = 5, y = 4\r\n\
                bo$\r\n\
                2bo$\r\n\
                3o$\r\n\
                5b!".to_string(),);

    std::fs::remove_file(&file_path);
}
