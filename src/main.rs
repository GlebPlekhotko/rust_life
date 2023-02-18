mod display;
mod field;

fn main() {
    let mut test_input = field::create(20, 40);
    field::random_populate(&mut test_input, 0.1);

    let console = display::Display::create(display::Id::CONSOLE, 20, 40);
	
	test_input[0][0].alive = true;

    console.draw(&test_input);

    field::update(&mut test_input);
    console.draw(&test_input);

    field::update(&mut test_input);
    console.draw(&test_input);
}
