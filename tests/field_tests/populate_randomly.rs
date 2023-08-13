use life::field::*;
use life::field::FenceType::*;

#[test]
#[should_panic]
fn zero_density() {
    let mut new_field = Field::create(10, 20, Cliff);

    new_field.populate_randomly(0.0);
}

#[test]
#[should_panic]
fn more_than_one_density() {
    let mut new_field = Field::create(10, 20, Cliff);

    new_field.populate_randomly(1.1);
}

#[test]
fn zero_point_five_density() {
    let mut new_field = Field::create(10, 20, Cliff);
    
    let expected_alive_top = 10 * 20 / 2 + 10 * 20 / 20;
    let expected_alive_bot = 10 * 20 / 2 - 10 * 20/ 20;

    new_field.populate_randomly(0.5);
    
    assert_eq!(true, new_field.alive < expected_alive_top);
    assert_eq!(true, new_field.alive > expected_alive_bot);
}
