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

                                        header_expected = false;
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

                    for _cell in 0..run_length {
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

                    for _cell in 0..run_length {
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
                        for _cell in 0..(x_field - x) {
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
                            y += 1;
                        }
                    } else {
                        for _cell in 0..(x_field - x) {
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
    let mut cell_char;
    let mut run_length;

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

    for y in 0..y_size {
        let mut smth_printed = false;
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
    }

    content.push_str("!");

    Ok(())
}

