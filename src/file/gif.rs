use crate::errors::ErrorCode;
use gif;
use std::borrow::Cow;
use std::fs::File;

enum Color {
	BLACK,
	WHITE,
	LAST
}


/// Takes given field of cells and moves it to the GIF's canvas

fn populate_canvas(canvas : &mut Vec<u8>, field : &Vec<Vec<bool>>, scale : u16) -> Result<(), ErrorCode>
{
    let field_width = field.len();

    if scale == 0 {
        return Err(ErrorCode::ZeroScale);
    }

    let mut x = 0;
    for row in field {
        let mut y = 0;

        for cell in row {
            for x_offset in 0..scale {
                for y_offset in 0..scale {
                    let x_scaled = (x * scale + x_offset) as usize;
                    let y_scaled = (y * scale + y_offset) as usize;

                    let pixel = coordinates_to_index(x_scaled, y_scaled, field_width * scale as usize)?;

                    if *cell == true {
                        canvas[pixel] = Color::BLACK as u8;
                    } else {
                        canvas[pixel] = Color::WHITE as u8;
                    }
                }
            }            

            y += 1;
        }

        x += 1;
    }

    Ok(())
}

/// Returns the dimensions of the field in the plaintext encoded file

pub fn dimensions(content : &String) -> Result<(usize, usize), ErrorCode>
{
    Err(ErrorCode::NotSupported)
}

/// Takes (x, y) coordinates of the cell and translated them to the index

fn coordinates_to_index(x : usize, y : usize, width : usize) -> Result<usize, ErrorCode> {
	if x < width && width > 0 {
        Ok(x + y * width)
    } else {
        Err(ErrorCode::CoordinatesToIndexConversionFailure)
    }
}


/// Loads (initializes) the field using the file's content

pub fn load(field : &mut Vec<Vec<bool>>, content : &String) -> Result<(), ErrorCode>
{
    Err(ErrorCode::NotSupported)
}

/// Saves content of the field to the gif file

pub fn save(file: &File, field : & Vec<Vec<bool>>) -> Result<(), ErrorCode>
{
	let width = field.len() as u16;
    let height = field[0].len() as u16;
    //let scale_width : u16 = 512 / width;
    //let scale_height : u16 = 512 / height;
    let scale = 1;

    /*
    if scale_width == 0 || scale_height == 0 {
    	scale = 1;
    } else {
    	if scale_width > scale_width {
    		scale = scale_width;
    	} else {
    		scale = scale_height;
    	}
    }
    */

    let pixels_in_canvas = width * height * scale;
	let mut canvas: Vec<u8> = Vec::with_capacity(pixels_in_canvas as usize);
	for byte in 0..canvas.capacity() {
		canvas.push(0);
	}

	let mut x = 0;
	for row in field {
		let mut y = 0;

		for cell in row {
            let pixel = coordinates_to_index(x, y, width)?;

			if *cell == true {
				canvas[pixel] = Color::BLACK as u8;
			} else {
				canvas[pixel] = Color::WHITE as u8;
			}

			y += 1;
		}

		x += 1;
	}

    let mut palette: Vec<u8> = Vec::with_capacity((Color::LAST as u32 * 3) as usize);
    for color in 0..Color::LAST as u8 {
        if color == Color::BLACK as u8 {
            palette.push(0x00);
            palette.push(0x00);
            palette.push(0x00);
        } else if color == Color::WHITE as u8 {
            palette.push(0xFF);
            palette.push(0xFF);
            palette.push(0xFF);
        } else {
            return Err(ErrorCode::UnrecognizedColor)
        }
    }

	let mut encoder = gif::Encoder::new(file, width, height, &palette).unwrap();

    let frame = gif::Frame {
        delay: 50,
        dispose: gif::DisposalMethod::Any,
        transparent: None,
        needs_user_input: false,
        top: 0,
        left: 0,
        width: width,
        height: height,
        interlaced: false,
        palette: None,
        buffer: Cow::from(&canvas)
    };

    if let Err(_) = encoder.set_repeat(gif::Repeat::Infinite) {
        return Err(ErrorCode::GifRepeatIntervalSetFailed)
    }
    if let Err(_) = encoder.write_frame(&frame) {
        return Err(ErrorCode::GifFrameWriteFailed)
    }
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod coordinates_to_index {
        use super::*;

        #[test]
        fn width_zero_x_zero_y_zero() {

            let result = coordinates_to_index(0, 0, 0);
        
            if let Err(ErrorCode::CoordinatesToIndexConversionFailure) = result {
                assert!(true);
            } else {
                assert!(false);
            }
        }

        #[test]
        fn width_ten_x_zero_y_zero() {

            let pixel = coordinates_to_index(0, 0, 10).unwrap();
        
            assert_eq!(0, pixel);
        }
        
        #[test]
        fn width_ten_x_nine_y_zero() {

            let pixel = coordinates_to_index(9, 0, 10).unwrap();
        
            assert_eq!(9, pixel);
        }

        #[test]
        fn width_ten_x_zero_y_one() {

            let pixel = coordinates_to_index(0, 1, 10).unwrap();
        
            assert_eq!(10, pixel);
        }

        #[test]
        fn width_ten_x_ten_y_zero() {

            let result = coordinates_to_index(10, 0, 10);
        
            if let Err(ErrorCode::CoordinatesToIndexConversionFailure) = result {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }

    mod populate_canvas {
        use super::*;

        fn create_field_and_canvas(scale : u16) -> (Vec<Vec<bool>>, Vec<u8>, Vec<u8>) {
            let mut field : Vec<Vec<bool>> = Vec::new();
            for x in 0..4 {
                field.push(Vec::new());
                for _y in 0..3 {
                    field[x].push(false);
                }
            }

            let mut canvas: Vec<u8> = Vec::with_capacity(field.len() * field[0].len() * (scale * scale) as usize);
            let mut expected_canvas: Vec<u8> = Vec::with_capacity(field.len() * field[0].len() * (scale * scale) as usize);
            for _byte in 0..canvas.capacity() {
                canvas.push(0);
                expected_canvas.push(1);
            }

            (field, canvas, expected_canvas)
        }

        #[test]
        fn scale_zero() {
            let (field, mut canvas, expected) = create_field_and_canvas(1);

            let result = populate_canvas(&mut canvas, &field, 0);
        
            if let Err(ErrorCode::ZeroScale) = result {
                assert!(true);
            }
        }

        #[test]
        fn scale_one_empty_field() {
            let (field, mut canvas, expected) = create_field_and_canvas(1);

            let result = populate_canvas(&mut canvas, &field, 1);
        
            if let Ok(_) = result {
                assert!(true);
            }

            assert_eq!(expected, canvas);
        }

        #[test]
        fn scale_one_not_empty_field() {
            let (mut field, mut canvas, mut expected) = create_field_and_canvas(1);
            for y in 0..field[0].len() {
                field[2][y] = true;
                expected[coordinates_to_index(2, y, field.len()).unwrap()] = 0;
            }

            let result = populate_canvas(&mut canvas, &field, 1);
        
            if let Ok(_) = result {
                assert!(true);
            }
            assert_eq!(expected, canvas);
        }

        #[test]
        fn scale_two_empty_field() {
            let (field, mut canvas, expected) = create_field_and_canvas(2);

            let result = populate_canvas(&mut canvas, &field, 2);
        
            if let Ok(_) = result {
                assert!(true);
            }
            assert_eq!(expected, canvas);
        }

        #[test]
        fn scale_two_not_empty_field() {
            let scale = 2;
            let (mut field, mut canvas, mut expected) = create_field_and_canvas(scale);
            for y in 0..field[0].len() {
                field[2][y] = true;
            }
            for y in 0..field[0].len() * 2 {
                let index = coordinates_to_index(4, y, field.len() * 2).unwrap();
                expected[index] = 0;
                
                let index = coordinates_to_index(5, y, field.len() * 2).unwrap();
                expected[index] = 0;
            }

            let result = populate_canvas(&mut canvas, &field, scale);
        
            if let Ok(_) = result {
                assert!(true);
            }
            assert_eq!(expected, canvas);
        }
    }
}