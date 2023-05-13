mod display;
//mod field;

use life::field;
use life::field::FenceType::*;

fn main() {
    let console = display::Display::create(display::Id::CONSOLE, 6, 6);
    let mut glider = field::Field::create(6, 6, Warp);
    let mut expected = field::Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][1] = true;
    expected.population[1][2] = true;
    expected.population[2][0] = true;
    expected.population[2][1] = true;
    expected.population[2][2] = true; 

    console.draw(&glider.population);
    
    glider.update(12);
    
    console.draw(&glider.population);

    expected.population[4][3] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][4] = true;

//    console.draw(&expected.population);
    
    // let new_field = field::Field::create(10,10, Cliff);
    // println!("{}", new_field.population.len());
    
    // let test : &bool = &new_field.population[0][0];
    // println!("{}", test);
    
    //new_field.cell[0][0].alive = Some(&new_field.population[0][0]);
    //println!("{}", test);
    
    //let mut test_field = field::create(20, 10);
    //field::random_populate(&mut test_field.cell, 0.1);
    
    //let console = display::Display::create(display::Id::CONSOLE, 20, 10);
    //console.draw(&test_field.cell);
	
	//test_input[0][0].alive = true;

    //console.draw(&test_input);

    //field::update(&mut test_input);
    //console.draw(&test_input);

    //field::update(&mut test_input);
    //console.draw(&test_input);
}
