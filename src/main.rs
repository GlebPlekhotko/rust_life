mod arguments;
mod display;
mod errors;
mod field;
mod file;
mod input;

use std::env;

fn main() {
    let arg_strings: Vec<String> = env::args().collect();   
    let mut args = arguments::parse(arg_strings);

    if let Some(_) = args.input_file {
        //(args.x_size, args.y_size) = file::dimensions(&args.input_file.unwrap()).unwrap();
    }


    //let mut field : Vec<Vec<bool>> = Vec::new();
    //let mut content = String::new();

    //for row in 0..5 {
    //    field.push(Vec::new());

    //    for cell in 0..4 {
    //        field[row].push(false);
    //    }
    //}
    //field[0][0] = true;
    //field[4][3] = true;

    //let result = file::rle::save(&field, &mut content);

    //for y in 0..field[0].len() {
    //    for x in 0..field.len() {
    //        print!("{} ", field[x][y]);
    //    }
    //    println!("");
    //}

    // if let Ok(()) = result {
    //    println!("Result ok");
    //} else {
    //    println!("Result not ok {:?}", result);
    //}


    //let console = display::Display::create(display::Id::CONSOLE, 6, 6);
    //let mut glider = field::Field::create(6, 6, Warp);
    //let mut expected = field::Field::create(6, 6, Warp);

    //glider.population[3][4] = true;
    //glider.population[4][5] = true;
    //glider.population[5][3] = true;
    //glider.population[5][4] = true;
    //glider.population[5][5] = true;

    //expected.population[0][1] = true;
    //expected.population[1][2] = true;
    //expected.population[2][0] = true;
    //expected.population[2][1] = true;
    //expected.population[2][2] = true; 

    //console.draw(&glider.population);
    
    //glider.update(12);
    
    //console.draw(&glider.population);

    //expected.population[4][3] = true;
    //expected.population[4][5] = true;
    //expected.population[5][4] = true;
    //expected.population[5][4] = true;

    //let args = arguments::parse(args_string);
}
