mod display;
mod arguments;
mod input;
mod file_formats;

use life::field;
use life::field::FenceType::*;
use std::env;

fn main() {
    let format = file_formats::deduce("test.cells".to_string());

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
