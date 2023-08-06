use life::errors::ErrorCode;
use life::file::rle::*;

#[test]
fn empty_content() {
    let content = "".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((0, 0), dimensions.unwrap());
}

#[test]
fn only_header() {
    let content = "#".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((0, 0), dimensions.unwrap());
}

#[test]
fn wrong_header_marker() {
    let content = "!".to_string();

    let dimensions = dimensions(&content);

    match dimensions {
        Ok(_) => assert!(false),
        Err(error_code) => {
            if let ErrorCode::WrongRleHeader = error_code { 
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn wrong_x_size_character() {
    let content = "#\n\
                   z = 1, y = 1".to_string();

    let dimensions = dimensions(&content);

    match dimensions {
        Ok(_) => assert!(false),
        Err(error_code) => {
            if let ErrorCode::WrongRleHeader = error_code { 
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn wrong_y_size_character() {
    let content = "#\n\
                   x = 1, z = 1".to_string();

    let dimensions = dimensions(&content);

    match dimensions {
        Ok(_) => assert!(false),
        Err(error_code) => {
            if let ErrorCode::WrongRleHeader = error_code { 
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn wrong_x_and_y_separator() {
    let content = "#\n\
                   x = 1; z = 1".to_string();

    let dimensions = dimensions(&content);

    match dimensions {
        Ok(_) => assert!(false),
        Err(error_code) => {
            if let ErrorCode::WrongRleHeader = error_code { 
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn two_by_two() {
    let content = "#\n\
                   x = 2, y = 2".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((2, 2), dimensions.unwrap());
}

#[test]
fn three_by_four() {
    let content = "#\n\
                   x = 3, y = 4".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((3, 4), dimensions.unwrap());
}

#[test]
fn three_by_four_with_data() {
    let content = "#\n\
                   x = 3, y = 4\n\
                   bo$\
                   2bo$\
                   o2b$\
                   3o!".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((3, 4), dimensions.unwrap());
}
