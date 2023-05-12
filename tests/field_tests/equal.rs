use life::field::*;
use life::field::FenceType::*;

#[test]
fn different_x_size() {
    let first_field = Field::create(1, 1, Cliff);
    let second_field = Field::create(2, 1, Cliff);
    
    assert_eq!(false, equal(&first_field, &second_field));
}

#[test]
fn different_y_size() {
    let first_field = Field::create(1, 1, Cliff);
    let second_field = Field::create(1, 2, Cliff);
    
    assert_eq!(false, equal(&first_field, &second_field));
}

#[test]
fn equal_size_no_changes() {
    let first_field = Field::create(3, 3, Cliff);
    let second_field = Field::create(3, 3, Cliff);
    
    assert_eq!(true, equal(&first_field, &second_field));
}

#[test]
fn equal_size_first_cell_changed() {
    let mut first_field = Field::create(3, 3, Cliff);
    let mut second_field = Field::create(3, 3, Cliff);
    
    first_field.population[0][0] = true;
    second_field.population[0][0] = false;
    
    assert_eq!(false, equal(&first_field, &second_field));
}

#[test]
fn equal_size_last_cell_changed() {
    let mut first_field = Field::create(3, 3, Cliff);
    let mut second_field = Field::create(3, 3, Cliff);
    
    first_field.population[2][2] = true;
    second_field.population[2][2] = false;
    
    assert_eq!(false, equal(&first_field, &second_field));
}