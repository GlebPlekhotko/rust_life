#[derive(PartialEq)]
pub struct Arguments {
    pub x_size : u32,
    pub y_size : u32,

    pub generations : u32,
    pub output_each_generation : bool,

    pub input_file : Option<String>,
    pub output_file : Option<String>
}

enum Switches {
    XSize,
    YSize,
    Generations,
    OutputEachGeneration,
    InputFile,
    OutputFile
}


// Parse the command line arguments and populate structure with values

pub fn parse(input : Vec<String>) -> Arguments {
    let mut arguments = Arguments {
        x_size : 0,
        y_size : 0,
        generations : 0,
        output_each_generation : false,
        input_file : None,
        output_file : None
    };

    if input.len() > 1 {
        let mut input_iter = input.iter();

        // Gulp the very first argument, it's always a program's name and path
        input_iter.next();

        let mut switch : Switches = Switches::XSize;
        let mut switch_expected = true;

        for switch_str in input_iter {
            if switch_expected == true {
                match fetch_switch(switch_str) {
                    Some(value) => switch = value,
                    _ => panic!("Unrecognized switch /n")
                }

                if switch_needs_value(&switch) == true {
                    switch_expected = false;
                    continue;
                }
            }

            match switch {
                Switches::XSize => arguments.x_size = get_unsigned_integer(switch_str),
                Switches::YSize => arguments.y_size = get_unsigned_integer(switch_str),
                Switches::Generations => arguments.generations = get_unsigned_integer(switch_str),
                Switches::OutputEachGeneration => arguments.output_each_generation = true,
                Switches::InputFile => arguments.input_file = get_string(switch_str),
                Switches::OutputFile => arguments.output_file = get_string(switch_str),
                _ => panic!("Something horrible has happened!\n")
            }

            switch_expected = true;
        }
    }

    arguments
}


/// Analyzes a given string for a valid argument presence

fn fetch_switch(switch_str : &String) -> Option<Switches> {
    let mut switch : Option <Switches> = None;
    let mut switch_char = switch_str.chars();

    if let Some('-') = switch_char.next() {
        match switch_char.next() {
            Some(character) => {
                match character {
                    'x' => switch = Some(Switches::XSize),
                    'y' => switch = Some(Switches::YSize),
                    'g' => switch = Some(Switches::Generations),
                    'e' => switch = Some(Switches::OutputEachGeneration),
                    'i' => switch = Some(Switches::InputFile),
                    'o' => switch = Some(Switches::OutputFile),
                    _ => ()
                }
            }
            None => ()
        }

        // Only two characters long switches are recognized
        if let Some(_) = switch_char.next() {
            switch = None;
        }
    }

    switch
}


/// Tries to convert given string to an unsigned integer

fn get_unsigned_integer(value_str : &String) -> u32 {
    match value_str.trim().parse() {
        Ok(value) => value,
        Err(_) => panic!("Wrong switch value\n")
    }
}

/// Tries to convert given string to an unsigned integer

fn get_string(value_str : &String) -> Option<String> {
    if value_str.len() > 0 {
        Some(String::from(value_str))
    } else {
        None
    }
}

/// Returns true if a given switch must be followed by a value

fn switch_needs_value(switch : &Switches) -> bool {
    match switch {
        Switches::OutputEachGeneration => false,
        _ => true
    }
}


