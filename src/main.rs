mod display;
mod arguments;
mod input;
mod file;
mod errors;

use life::field;
use life::field::FenceType::*;
use std::env;

fn main() {
    let content = "x = 10, y = 2\n\
                   10o!".to_string();
    let mut field : Vec<Vec<bool>> = Vec::new();
    
    for row in 0..10 {
        field.push(Vec::new());
        for cell in 0..2 {
            field[row].push(false);
        }
    }

    let result = file::rle::load(&mut field, &content);

    for y in 0..field[0].len() {
        for x in 0..field.len() {
            print!("{} ", field[x][y]);
        }
        println!("");
    }

     if let Ok(()) = result {
        println!("Result ok");
    } else {
        println!("Result not ok {:?}", result);
    }


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
