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

#[cfg(test)]
mod tests {
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
