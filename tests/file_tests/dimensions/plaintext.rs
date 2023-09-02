use life::errors::ErrorCode;
use life::file::plaintext::*;

#[test]
fn empty_content() {
    let content = "".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((0, 0), dimensions.unwrap());
}

#[test]
fn header_not_expected() {
    let content = ".O\n\
                   !First line\n\
                   O.\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!(ErrorCode::HeaderNotExpected, dimensions.err().unwrap());
}

#[test]
fn unrecognized_character() {
    let content = "!First line\n\
                   !\n\
                   .O\n\
                   o.\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!(ErrorCode::UnrecognizedCharacter, dimensions.err().unwrap());
}

#[test]
fn one_line_header() {
    let content = "!First line\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((0, 0), dimensions.unwrap());
}

#[test]
fn two_lines_header() {
    let content = "!First line\n\
                   !\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((0, 0), dimensions.unwrap());
}

#[test]
fn two_by_two() {
    let content = "!First line\n\
                   !\n\
                   .O\n\
                   O.\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((2, 2), dimensions.unwrap());
}

#[test]
fn three_by_four() {
    let content = "!First line\n\
                   !\n\
                   .O.\n\
                   ..O\n\
                   .O.\n\
                   O..\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((3, 4), dimensions.unwrap());
}

#[test]
fn newline_in_the_middle() {
    let content = "!First line\n\
                   !\n\
                   .O.\n\
                   ..O\n\
                   .O.\n\
                   \n\
                   O..\n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!(ErrorCode::CellsNotExpected, dimensions.err().unwrap());
}

#[test]
fn trailing_newlines_ignored() {
    let content = "!First line\n\
                   !\n\
                   .O.\n\
                   ..O\n\
                   .O.\n\
                   O..\n\
                   \n".to_string();

    let dimensions = dimensions(&content);

    assert_eq!((3, 4), dimensions.unwrap());
}
