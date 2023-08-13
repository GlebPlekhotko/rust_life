mod arguments;
mod display;
mod errors;
mod field;
mod file;

use display::Display;
use errors::ErrorCode;
use field::Field;
use std::env;

fn main() {
    let arg_strings: Vec<String> = env::args().collect();   
    let mut args = arguments::parse(arg_strings);
    let mut load_file = false;

    match args.input_file {
        Some(ref file) => {
            if args.x_size != 0 || args.y_size != 0 {
                println!("Error: Both dimensions and load file specified, code {}", ErrorCode::BothFileAndDimensionsSpecified as i32);
                return;
            }

            match file::dimensions(&file) {
                Ok(x_y) => (args.x_size, args.y_size) = x_y,
                Err(error) => println!("Error: Failed to obtain dimensions from the file, code {}", error as i32)
            }
        },
        None => {
            ()
        }
    }

    let display = Display::create(display::Id::CONSOLE, args.x_size, args.y_size);
    let mut field = Field::create(args.x_size, args.y_size, args.fence_type);

    match args.input_file {
        Some(ref file) => {
            match file::load(&mut field.population, &file) {
                Ok(()) => (),
                Err(error) => println!("Error: Failed to obtain dimensions from the file, code {}", error as i32)
            }
        },
        None => {
            field.populate_randomly(args.density);
        }
    }

    if args.output_each_generation == true {
        for generation in 0..args.generations {
            field.update(1);

            match args.output_file {
                Some(ref file) => {
                    let generation_file = file.clone() + "_" + &generation.to_string();
                    file::save(&field.population, &generation_file);
                },
                None => {
                    display.draw(&field.population);
                }
            };
        }
    } else {
        field.update(args.generations);
        match args.output_file {
            Some(ref file) => {
                file::save(&field.population, file);
            },
            None => {
                display.draw(&field.population);
            }
        };
    }
}
