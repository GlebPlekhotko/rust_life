mod display;
//mod field;

use life::field;
use life::field::FieldFenceType::*;

fn main() {
    let new_field = field::Field::create(10,10, FieldFenceTypeCliff);
    println!("{}", new_field.population.len());
    
    let test : &bool = &new_field.population[0][0];
    println!("{}", test);
    
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
