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
    let arg_string: Vec<String> = env::args().collect();

    if arg_string.len() <= 1 {
        arguments::help();
        return;
    }

    let mut args = arguments::parse(arg_string);

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

    let display = Display::create(args.x_size, args.y_size);
    let mut field = Field::create(args.x_size, args.y_size, args.fence_type);

    match args.input_file {
        Some(ref file) => {
            if let Err(error) = file::load(&mut field.population, &file) {
                println!("Error: Failed to load, code {}", error as i32)
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

                    if let Err(error) = file::save(&field.population, &generation_file) {
                        println!("Error: Failed to save, code {}", error as i32)
                    }
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
                if let Err(error) = file::save(&field.population, file) {
                    println!("Error: Failed to save, code {}", error as i32)
                }
            },
            None => {
                display.draw(&field.population);
            }
        };
    }
}
