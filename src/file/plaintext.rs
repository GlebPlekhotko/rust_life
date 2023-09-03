use crate::errors::ErrorCode;

/// Returns the dimensions of the field in the plaintext encoded file

pub fn dimensions(content : &String) -> Result<(usize, usize), ErrorCode>
{
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut header_expected = true;
    let mut x_obtained = false;
    let mut empty_line_found = false;

    for line in content.lines() {
        match line.chars().next() {
            Some(first_char) => match first_char {
                                    '!' =>  {
                                        if header_expected == false {
                                            return Err(ErrorCode::HeaderNotExpected);
                                        }
                                    },
                                    '.' | 'O' => {
                                        if empty_line_found == true {
                                            return Err(ErrorCode::CellsNotExpected);
                                        }

                                        if x_obtained == false {
                                            x = line.len();
                                            x_obtained = true;
                                        }
                                        y = y + 1;
                                        header_expected = false;
                                    },
                                    _ => {
                                        return Err(ErrorCode::UnrecognizedCharacter);
                                    }
                                }
            _ => {
                empty_line_found = true;
            }
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
        let mut empty_line = true;

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
                    empty_line = false;
                    
                    header_passed = true;
                    field[x][y] = false;                    
                },
                'O' => {
                    empty_line = false;

                    header_passed = true;
                    field[x][y] = true;
                },
                _ => {
                    return Err(ErrorCode::UnrecognizedCharacter);
                }
            }

            x += 1;
        }

        if empty_line == false {
            while x < x_size {
                field[x][y] = false;
                x += 1;
            }

            x = 0;
            y += 1;
        }
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
