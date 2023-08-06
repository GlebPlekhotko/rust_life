use life::errors::ErrorCode;
use life::file::plaintext::*;

#[test]
fn field_x_size_small() {
    let content = "..".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field[0].push(true);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::FieldTooSmall, result.err().unwrap());
}


#[test]
fn field_y_size_small() {
    let content = ".\n\
                   .".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field[0].push(true);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::FieldTooSmall, result.err().unwrap());
}

#[test]
fn header_not_expected() {
    let content = "O.\n\
                   !\n\
                   .O".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::HeaderNotExpected, result.err().unwrap());
}

#[test]
fn unrecognized_character() {
    let content = "!\n\
                   O.\n\
                   .0".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::UnrecognizedCharacter, result.err().unwrap());
}

#[test]
fn exact_load() {
    let content = "!\n\
                   O..\n\
                   .O.\n\
                   ..O".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(true, field[0][0]); assert_eq!(false, field[0][1]); assert_eq!(false, field[0][2]);
    assert_eq!(false, field[1][0]); assert_eq!(true, field[1][1]); assert_eq!(false, field[1][2]);
    assert_eq!(false, field[2][0]); assert_eq!(false, field[2][1]); assert_eq!(true, field[2][2]);
}

#[test]
fn oversize_load() {
    let content = "!\n\
                   O..\n\
                   .O.\n\
                   ..O".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(true);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(true,  field[0][0]); assert_eq!(false, field[0][1]); assert_eq!(false, field[0][2]);
    assert_eq!(false, field[1][0]); assert_eq!(true,  field[1][1]); assert_eq!(false, field[1][2]);
    assert_eq!(false, field[2][0]); assert_eq!(false, field[2][1]); assert_eq!(true,  field[2][2]);
    assert_eq!(false, field[3][0]); assert_eq!(false, field[3][1]); assert_eq!(false,  field[3][2]);
}
