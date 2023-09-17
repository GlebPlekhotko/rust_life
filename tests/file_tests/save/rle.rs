use life::file::rle::*;

#[test]
fn empty_field() {
    let field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    let result = save(&field, &mut content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!("x = 0, y = 0\r\n".to_string(), content);
}

#[test]
fn all_dead() {
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
            field[row].push(false);
        }
    }

    let result = save(&field, &mut content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!("x = 5, y = 4\r\n\
                5b$\r\n5b$\r\n5b$\r\n5b!".to_string(), content);
}

#[test]
fn one_alive_in_the_first_cell() {
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
            field[row].push(false);
        }
    }
    field[0][0] = true;

    let result = save(&field, &mut content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!("x = 5, y = 4\r\n\
                o$\r\n\
                5b$\r\n\
                5b$\r\n\
                5b!".to_string(), content);
}

#[test]
fn one_alive_in_the_last_cell() {
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
            field[row].push(false);
        }
    }
    field[4][3] = true;

    let result = save(&field, &mut content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!("x = 5, y = 4\r\n\
                5b$\r\n\
                5b$\r\n\
                5b$\r\n\
                4bo!".to_string(), content);
}

#[test]
fn glider() {
    let mut field : Vec<Vec<bool>> = Vec::new();
    let mut content = String::new();

    for row in 0..5 {
        field.push(Vec::new());

        for _cell in 0..4 {
            field[row].push(false);
        }
    }
    field[1][0] = true;
    field[2][1] = true;
    field[0][2] = true; field[1][2] = true; field[2][2] = true;


    let result = save(&field, &mut content);

    assert!(if let Ok(()) = result {true} else {false});
    assert_eq!("x = 5, y = 4\r\n\
                bo$\r\n\
                2bo$\r\n\
                3o$\r\n\
                5b!".to_string(), content);
}
