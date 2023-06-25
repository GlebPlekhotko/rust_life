use life::file_formats::*;

#[test]
fn plaintext_simple_path() {
    let format = deduce("test.cells".to_string());

    assert!(if let FileFormats::PlainText = format { true } else { false });
}

#[test]
fn plaintext_complex_path() {
    let format = deduce("dir1/dir2/test.cells".to_string());

    assert!(if let FileFormats::PlainText = format { true } else { false });
}

#[test]
fn rle_simple_path() {
    let format = deduce("test.rle".to_string());

    assert!(if let FileFormats::Rle = format { true } else { false });
}

#[test]
fn rle_complex_path() {
    let format = deduce("dir1/dir2/test.rle".to_string());

    assert!(if let FileFormats::Rle = format { true } else { false });
}

#[test]
fn unknown_simple_path() {
    let format = deduce("test.bla".to_string());

    assert!(if let FileFormats::Unknown = format { true } else { false });
}

#[test]
fn unknown_complex_path() {
    let format = deduce("dir1/dir2/test.bla".to_string());

    assert!(if let FileFormats::Unknown = format { true } else { false });
}

#[test]
fn unknown_no_extenstion_simple_path() {
    let format = deduce("test_cells".to_string());

    assert!(if let FileFormats::Unknown = format { true } else { false });
}

#[test]
fn unknown_no_extenstion_complex_path() {
    let format = deduce("dir1/dir2/test_cells".to_string());

    assert!(if let FileFormats::Unknown = format { true } else { false });
}
