mod plaintext;
mod rle;

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

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
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

    if let Err(_) = std::fs::remove_file(&file_path) {
        panic!("Failed to delete a file while testing");
    }
}

#[test]
fn plaintext_file_nested_dirs() {

    let file_path = "tests/file_tests/nested/test.cells".to_string();
    let dir_path = "tests/file_tests/nested/";
    let mut field : Vec<Vec<bool>> = Vec::new();


    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
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

    if let Err(_) = std::fs::remove_dir_all(&dir_path) {
        panic!("Failed to remove a directory while testing");
    }
}

#[test]
fn rle_file() {

    let file_path = "tests/file_tests/test.rle".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
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

    if let Err(_) = std::fs::remove_file(&file_path) {
        panic!("Failed to delete a file while testing");
    }
}

#[test]
fn rle_file_nested_dirs() {

    let file_path = "tests/file_tests/nested/test.rle".to_string();
    let dir_path = "tests/file_tests/nested/";
    let mut field : Vec<Vec<bool>> = Vec::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
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

    if let Err(_) = std::fs::remove_dir_all(&dir_path) {
        panic!("Failed to remove a directory while testing");
    }
}
