use life::errors::ErrorCode;
use life::file::rle::*;

#[test]
fn field_x_size_small() {
    let content = "x = 2, y = 1\n\
                   oo".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field[0].push(true);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::FieldTooSmall, result.err().unwrap());
}

#[test]
fn field_y_size_small() {
    let content = "x = 1, y = 2\n\
                   oo".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field[0].push(true);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::FieldTooSmall, result.err().unwrap());
}

#[test]
fn header_not_expected() {
    let content = "x = 2, y = 2\n\
                   oo$\n\
                   # Some Comment\n\
                   bb!".to_string();
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
    let content = "x = 2, y = 2\n\
                   oo$\n\
                   # Some Comment\n\
                   bb!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false);

    let result = load(&mut field, &content);

    assert_eq!(ErrorCode::HeaderNotExpected, result.err().unwrap());
}

#[test]
fn exact_load() {
    let content = "x = 3, y = 3\n\
                   obb$\n\
                   bob$\n\
                   bbo$!".to_string();
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
    let content = "x = 3, y = 3\n\
                   obb$\n\
                   bob$\n\
                   bbo$!".to_string();
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
    assert_eq!(true, field[0][0]); assert_eq!(false, field[0][1]); assert_eq!(false, field[0][2]);
    assert_eq!(false, field[1][0]); assert_eq!(true, field[1][1]); assert_eq!(false, field[1][2]);
    assert_eq!(false, field[2][0]); assert_eq!(false, field[2][1]); assert_eq!(true, field[2][2]);
    assert_eq!(false, field[3][0]); assert_eq!(false, field[3][1]); assert_eq!(false, field[3][2]);
}

#[test]
fn no_end_of_line_before_termination() {
    let content = "x = 3, y = 3\n\
                   obb$\n\
                   bob$\n\
                   bbo!".to_string();
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
fn one_digit_run() {
    let content = "x = 5, y = 3\n\
                   b3ob$\n\
                   o3bo$\n\
                   5o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(false); field[4].push(false);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(true,  field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(true,  field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(true,  field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(true,  field[3][2]); assert_eq!(true,  field[4][2]);
}

#[test]
fn one_digit_run_with_line_termination() {
    let content = "x = 5, y = 3\n\
                   b3ob$\n\
                   o3bo$\n\
                   5o$!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(false); field[4].push(false);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(true,  field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(true,  field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(true,  field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(true,  field[3][2]); assert_eq!(true,  field[4][2]);
}

#[test]
fn one_digit_run_first_line_not_complete() {
    let content = "x = 5, y = 3\n\
                   b3o$\n\
                   o3bo$\n\
                   3o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(true);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(true); field[4].push(true);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(true,  field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(true,  field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(true,  field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(false,  field[3][2]); assert_eq!(false,  field[4][2]);
}

#[test]
fn one_digit_run_intermediate_line_not_complete() {
    let content = "x = 5, y = 3\n\
                   b3o$\n\
                   o3b$\n\
                   3o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(true);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(true); field[4].push(true);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(true,  field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(true,  field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(false,  field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(false,  field[3][2]); assert_eq!(false,  field[4][2]);
}

#[test]
fn one_digit_run_last_line_not_complete() {
    let content = "x = 5, y = 3\n\
                   b3ob$\n\
                   o3bo$\n\
                   3o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(true); field[4].push(true);

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(false, field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(true,  field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(true,  field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(true,  field[4][1]);
    assert_eq!(true,  field[0][2]); assert_eq!(true,  field[1][2]); assert_eq!(true,  field[2][2]); assert_eq!(false,  field[3][2]); assert_eq!(false,  field[4][2]);
}

#[test]
fn one_digit_run_just_first_line() {
    let content = "x = 5, y = 3\n\
                   3o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(true); field[4].push(false);
    

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(true,  field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(false, field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(false, field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(false, field[4][1]);
    assert_eq!(false, field[0][2]); assert_eq!(false, field[1][2]); assert_eq!(false, field[2][2]); assert_eq!(false, field[3][2]); assert_eq!(false, field[4][2]);
}

#[test]
fn one_digit_run_just_first_line_with_termination() {
    let content = "x = 5, y = 3\n\
                   3o$!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field.push(Vec::new());
    field[0].push(false); field[0].push(false); field[0].push(false);
    field[1].push(false); field[1].push(false); field[1].push(false);
    field[2].push(false); field[2].push(false); field[2].push(false);
    field[3].push(false); field[3].push(false); field[3].push(false);
    field[4].push(false); field[4].push(true); field[4].push(false);
    

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!(true,  field[0][0]); assert_eq!(true,  field[1][0]); assert_eq!(true,  field[2][0]); assert_eq!(false, field[3][0]); assert_eq!(false, field[4][0]);
    assert_eq!(false, field[0][1]); assert_eq!(false, field[1][1]); assert_eq!(false, field[2][1]); assert_eq!(false, field[3][1]); assert_eq!(false, field[4][1]);
    assert_eq!(false, field[0][2]); assert_eq!(false, field[1][2]); assert_eq!(false, field[2][2]); assert_eq!(false, field[3][2]); assert_eq!(false, field[4][2]);
}

#[test]
fn two_digits_run_just_first_line() {
    let content = "x = 10, y = 2\n\
                   10o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    for x in 0..10 {
        field.push(Vec::new());
        for y in 0..2 {
            field[x].push(false);
        }
    }

    let result = load(&mut field, &content);

    assert!(if let Ok(()) = result {true} else {false});
    for x in 0..10 {
        for y in 0..2 {
            if y == 0 {
                assert_eq!(true,  field[x][y]);
            } else {
                assert_eq!(false, field[x][y]);
            }
        }
    }
}
