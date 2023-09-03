mod arguments;
mod display;
mod errors;
mod field;
mod file;

use display::Display;
use errors::ErrorCode;
use field::Field;
use std::env;
use std::io;


/// A smaller wrapper to print the field to the standard output

fn display_field(display : &Display, field : &Field, gen : u32) {
    println!("Generation {}", gen);
    display.draw(&field.population);
    println!("");
}

fn main() {
    let arg_string: Vec<String> = env::args().collect();
    let mut args = arguments::parse(arg_string);

    if args.help == true {
        arguments::help();
        return;
    }

    if let Some(ref file) = args.input_file {
        match file::dimensions(&file) {
            Ok(x_y) => (args.x_size, args.y_size) = x_y,
            Err(error) => println!("Error: Failed to obtain dimensions from the file, code {}", 
                                   error as i32)
        }
    }

    let display = Display::create(args.x_size, args.y_size);
    let mut field = Field::create(args.x_size, args.y_size, args.fence_type);

    match args.input_file {
        Some(ref file) => {
            if let Err(error) = file::load(&mut field.population, &file) {
                println!("Error: Failed to load, code {}", 
                         error as i32)
            }
        },
        None => {
            field.populate_randomly(args.density);
        }
    }

    if let None = args.output_file {
        display_field(&display, &field, 0);
    }

    if args.output_each_generation == true {
        for generation in 0..args.generations {
            field.update(1);

            match args.output_file {
                Some(ref file) => {
                    let generation_file = match file::append_number(file, generation + 1) {
                        Ok(new_file) => new_file,
                        Err(error) => {
                            println!("Error: Failed to append generation number, code {}", 
                                     error as i32);
                            return;
                        }
                    };

                    if let Err(error) = file::save(&field.population, &generation_file) {
                        println!("Error: Failed to save, code {}", 
                                 error as i32);
                    }
                },
                None => {
                    display_field(&display, &field, generation + 1);

                    let mut _new_line = String::new();
                    if let io::Result::Err(_) = io::stdin().read_line(&mut _new_line) {
                        println!("Error: Failed to get a newline character, code {}", 
                                 ErrorCode::NewlineFailed as i32);
                    }
                }
            };
        }
    } else {
        field.update(args.generations);
        match args.output_file {
            Some(ref file) => {
                if let Err(error) = file::save(&field.population, file) {
                    println!("Error: Failed to save, code {}", 
                             error as i32);
                }
            },
            None => {
                display_field(&display, &field, args.generations);
            }
        }
    }
}
