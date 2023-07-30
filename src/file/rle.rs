use crate::errors::ErrorCode;

/// Returns the dimensions of the field in the RLE encoded file

pub fn dimensions(content : &String) -> Result<(usize, usize), ErrorCode>
{
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut header_expected = true;

    'content_loop: 
    for line in content.lines() {
        match line.chars().next() {
            Some(first_char) => match first_char {
                                    '#' =>  {
                                        if header_expected == false {
                                            return Err(ErrorCode::HeaderNotExpected)
                                        }
                                    },
                                    'x' | 'X' => {
                                        let mut x_expected = true;                                                                                

                                        for token in line.split(",") {

                                            match token.trim().chars().next().unwrap() {
                                                'x' | 'X' => if x_expected == false { return Err(ErrorCode::WrongRleHeader) },
                                                'y' | 'Y' => if x_expected == true { return Err(ErrorCode::WrongRleHeader) },
                                                _ => return Err(ErrorCode::WrongRleHeader)
                                            }

                                            let mut token_iter = token.split("=");

                                            token_iter.next();
                                            match token_iter.next() {
                                                Some(size_str) => {
                                                    let value : usize = match size_str.trim().parse() {
                                                        Ok(val) => val,
                                                        Err(_) => return Err(ErrorCode::WrongRleHeader)
                                                    };

                                                    if x_expected == true {
                                                        x = value;
                                                        x_expected = false;
                                                    } else {
                                                        y = value;
                                                        break 'content_loop;
                                                    }
                                                }
                                                None => return Err(ErrorCode::WrongRleHeader)
                                            }
                                        }
                                    }
                                    _ => return Err(ErrorCode::WrongRleHeader)
                                }
            _ => panic!("There is a line, but now characters in it\n")
        }
    }

    Ok((x, y))
}

/// Loads (initializes) the field using the file's content

pub fn load(field : &mut Vec<Vec<bool>>, content : &String) -> Result<(), ErrorCode>
{
    let x_field = field.len();
    let y_field = field[0].len();
    let (x_content, y_content) = dimensions(content)?;
    
    if (x_field < x_content) || (y_field < y_content) {
        return Err(ErrorCode::FieldTooSmall);
    }

    let mut header_passed = false;
    let mut x : usize = 0;
    let mut y : usize = 0;

    'line_loop: for line in content.lines() {
        let mut run_length = 0;

        for character in line.chars() {
            match character {
                '#' => {
                    if header_passed == true {
                        return Err(ErrorCode::HeaderNotExpected)
                    } else {
                        continue 'line_loop;
                    }
                },
                'x' | 'X' => {
                    header_passed = true;
                    continue 'line_loop;
                },
                'b' => {
                    // Dead cell

                    if x >= x_field {
                        return Err(ErrorCode::RleVolationXSize)     
                    }

                    if run_length == 0 {
                        run_length = 1;
                    }

                    for cell in 0..run_length {
                        field[x][y] = false;
                        x += 1;
                    }

                    run_length = 0;
                },
                'o' => {
                    // Alive cell
                    if x >= x_field {
                        return Err(ErrorCode::RleVolationXSize)
                    }

                    if run_length == 0 {
                        run_length = 1;
                    }

                    for cell in 0..run_length {
                        field[x][y] = true;
                        x += 1;
                    }

                    run_length = 0;
                },
                '$' => {
                    // End of sequence (line)

                    if y >= y_field {
                        return Err(ErrorCode::RleVolationYSize)
                    }

                    if x != x_field {                       
                        for cell in 0..(x_field - x) {
                            field[x][y] = false;
                        }
                    }

                    x = 0;
                    y += 1;
                },
                '!' => {
                    if y == y_field {
                        break 'line_loop;
                    }

                    if x == x_field {
                        if y == y_field - 1 {
                            break 'line_loop;
                        } else {
                            x = 0;
                            y += 1;
                        }
                    } else {
                        for cell in 0..(x_field - x) {
                            field[x][y] = false;
                            x += 1;
                        }
                        y += 1;
                    }

                    for row in y..=(y_field - y) {
                        for cell in 0..x_field {
                            field[cell][row] = false;
                        }
                    }

                    break 'line_loop;
                },
                ' ' => {
                    // Just a space
                }
                other => {
                    if other.is_digit(10) == true {
                        run_length *= 10;
                        run_length += other.to_digit(10).unwrap();
                    } else {
                        return Err(ErrorCode::UnrecognizedCharacter)
                    }
                }
            }
        }
    }

    Ok(())
}

/// Saves content of the field to the given string

pub fn save(field : & Vec<Vec<bool>>, content : &mut String) -> Result<(), ErrorCode>
{
    let alive_cell = "o";
    let dead_cell = "b";
    let x_size : usize;
    let y_size : usize;
    let mut cell_char : &str = alive_cell;
    let mut run_length = 0;

    x_size = field.len();
    if x_size > 0 {
        y_size = field[0].len();
    } else {
        y_size = 0;
    }

    content.clear();
    
    content.push_str(&format!("x = {}, y = {}\r\n", x_size, y_size));
    if x_size == 0 || y_size == 0 {
        return Ok(());
    }    


    let mut first_row = true;

    for y in 0..y_size {
        let mut smth_printed = false;
        let mut new_row = true;
        let mut previous_cell = false;

        run_length = 0;

        for x in 0..x_size {
            if x == 0 {
                previous_cell = field[x][y];

                if y > 0 {
                    content.push_str("$\r\n");
                }
            }

            if field[x][y] == previous_cell {
                run_length += 1;   
                continue;             
            }

            if previous_cell == true {
                cell_char = alive_cell;
            } else {
                cell_char = dead_cell;
            }

            if run_length > 1 {
                content.push_str(&format!("{}", run_length));
            }
            content.push_str(cell_char);

            smth_printed = true;
            previous_cell = field[x][y];
            run_length = 1;
        }

        if previous_cell == true {
            if run_length > 1 {
                content.push_str(&format!("{}", run_length));
            }
            content.push_str(alive_cell);
        } else {
            if smth_printed == false {
                if run_length > 1 {
                    content.push_str(&format!("{}", run_length));
                }
                content.push_str(dead_cell);
            }
        }

        first_row = false;
    }

    content.push_str("!");

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
    }

    mod load {
        use super::*;

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
    }

    mod save {
        use super::*;

        #[test]
        fn empty_field() {
            let mut field : Vec<Vec<bool>> = Vec::new();
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

                for cell in 0..4 {
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

                for cell in 0..4 {
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

                for cell in 0..4 {
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

                for cell in 0..4 {
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
    }
}
