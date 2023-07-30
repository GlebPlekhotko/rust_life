use crate::errors::ErrorCode;


/// Returns the dimensions of the field in the plaintext encoded file

pub fn dimensions(content : &String) -> Result<(usize, usize), ErrorCode>
{
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut header_expected = true;
    let mut x_obtained = false;

    for line in content.lines() {
        match line.chars().next() {
            Some(first_char) => match first_char {
                                    '!' =>  {
                                        if header_expected == false {
                                            return Err(ErrorCode::HeaderNotExpected)
                                        }
                                    },
                                    '.' | 'O' => {
                                        if x_obtained == false {
                                            x = line.len();
                                            x_obtained = true;
                                        }
                                        y = y + 1;
                                        header_expected = false;
                                    }
                                    _ => return Err(ErrorCode::UnrecognizedCharacter)
                                }
            _ => panic!("There is a line, but now characters in it\n")
        }
    }

    Ok((x, y))
}


/// Loads (initializes) the field using the file's content

pub fn load(field : &mut Vec<Vec<bool>>, content : &String) -> Result<(), ErrorCode>
{
    let x_size = field.len();
    let y_size = field[0].len();
    let (x_content, y_content) = dimensions(content)?;
    
    if (x_size < x_content) || (y_size < y_content) {
        return Err(ErrorCode::FieldTooSmall);
    }

    let mut header_passed = false;
    let mut x : usize = 0;
    let mut y : usize = 0;

    'line_loop: for line in content.lines() {
        for character in line.chars() {
            match character {
                '!' => {
                    if header_passed == true {
                        return Err(ErrorCode::HeaderNotExpected);
                    } else {
                        continue 'line_loop;
                    }
                },
                '.' => {
                    header_passed = true;
                    field[x][y] = false;
                },
                'O' => {
                    header_passed = true;
                    field[x][y] = true;
                },
                _ => {
                    return Err(ErrorCode::UnrecognizedCharacter);
                }
            }

            x += 1;
        }

        while x < x_size {
            field[x][y] = false;
            x += 1;
        }

        x = 0;
        y += 1;
    }

    while y < y_size {
        for x in 0..x_size {
            field[x][y] = false;
        }

        y += 1;
    }

    Ok(())
}

/// Saves content of the field to the given string

pub fn save(field : & Vec<Vec<bool>>, content : &mut String) -> Result<(), ErrorCode>
{
    let x_size : usize;
    let y_size : usize;

    x_size = field.len();
    if x_size > 0 {
        y_size = field[0].len();
    } else {
        y_size = 0;
    }

    content.clear();

    for y in 0..y_size {
        for x in 0..x_size {
            if field[x][y] == true {
                content.push_str("O");
            } else {
                content.push_str(".");
            }
        }
        content.push_str("\r\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dimensions {
        use super::*;

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
    }

    mod load {
        use super::*;

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
    }

    mod save {
        use super::*;

        #[test]
        fn empty_field() {
            let mut field : Vec<Vec<bool>> = Vec::new();
            let mut content = String::new();

            let result = save(&field, &mut content);

            assert!(if let Ok(()) = result {true} else {false});
            assert_eq!("".to_string(), content);
        }

        #[test]
        fn all_dead() {
            let mut field : Vec<Vec<bool>> = Vec::new();
            let mut content = String::new();

            for row in 0..5 {
                field.push(Vec::new());

                for cell in 0..4 {
                    field[row].push(false);
                }
            }

            let result = save(&field, &mut content);

            assert!(if let Ok(()) = result {true} else {false});
            assert_eq!(".....\r\n\
                        .....\r\n\
                        .....\r\n\
                        .....\r\n".to_string(), content);
        }

        #[test]
        fn one_alive_in_the_first_cell() {
            let mut field : Vec<Vec<bool>> = Vec::new();
            let mut content = String::new();

            for row in 0..5 {
                field.push(Vec::new());

                for cell in 0..4 {
                    field[row].push(false);
                }
            }
            field[0][0] = true;

            let result = save(&field, &mut content);

            assert!(if let Ok(()) = result {true} else {false});
            assert_eq!("O....\r\n\
                        .....\r\n\
                        .....\r\n\
                        .....\r\n".to_string(), content);
        }

        #[test]
        fn one_alive_in_the_last_cell() {
            let mut field : Vec<Vec<bool>> = Vec::new();
            let mut content = String::new();

            for row in 0..5 {
                field.push(Vec::new());

                for cell in 0..4 {
                    field[row].push(false);
                }
            }
            field[4][3] = true;

            let result = save(&field, &mut content);

            assert!(if let Ok(()) = result {true} else {false});
            assert_eq!(".....\r\n\
                        .....\r\n\
                        .....\r\n\
                        ....O\r\n".to_string(), content);
        }

        #[test]
        fn glider() {
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


            let result = save(&field, &mut content);

            assert!(if let Ok(()) = result {true} else {false});
            assert_eq!(".O...\r\n\
                        ..O..\r\n\
                        OOO..\r\n\
                        .....\r\n".to_string(), content);
        }
    }
}
