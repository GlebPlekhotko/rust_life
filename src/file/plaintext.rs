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
