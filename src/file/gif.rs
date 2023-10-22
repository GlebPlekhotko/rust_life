use crate::errors::ErrorCode;
use gif;
use std::borrow::Cow;
use std::fs::File;

enum Color {
	BLACK,
	WHITE,
	LAST
}

/// Returns the dimensions of the field in the plaintext encoded file

pub fn dimensions(content : &String) -> Result<(usize, usize), ErrorCode>
{
    Err(ErrorCode::NotSupported)
}

/// Takes (x, y) coordinates of the cell and translated them to the index

fn coordinates_to_index(x : u16, y : u16, width : u16) -> Result<usize, ErrorCode> {
	if x < width && width > 0 {
        Ok((x + (y * width)) as usize)
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
}