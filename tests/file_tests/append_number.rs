use life::errors::ErrorCode;
use life::file::*;

#[test]
fn empty_path() {

    let result = append_number(&String::from(""), 0);

    assert!(if let Err(ErrorCode::EmptyPath) = result {true} else {false});
}

#[test]
fn no_extension() {

    let result = append_number(&String::from("some_file"), 0);

    assert!(if let Err(ErrorCode::NoFileExtension) = result {true} else {false});
}

#[test]
fn no_extension_but_with_dot() {

    let result = append_number(&String::from("some_file."), 0);

    assert!(if let Err(ErrorCode::NoFileExtension) = result {true} else {false});
}

#[test]
fn just_file() {

    let result = append_number(&String::from("some_file.ext"), 0);

    assert_eq!("some_file_0.ext", result.unwrap());
}

#[test]
fn just_file_multiple_digits() {

    let result = append_number(&String::from("some_file.ext"), 10);

    assert_eq!("some_file_10.ext", result.unwrap());
}

#[test]
fn complex_path() {

    let result = append_number(&String::from("dir1\\dir2\\some_file.ext"), 0);

    assert_eq!("dir1\\dir2\\some_file_0.ext", result.unwrap());
}

#[test]
fn complex_path_multiple_digits() {

    let result = append_number(&String::from("dir1\\dir2\\some_file.ext"), 10);

    assert_eq!("dir1\\dir2\\some_file_10.ext", result.unwrap());
}
