use life::field::*;
use life::field::FenceType::*;

#[test]
#[should_panic]
fn zero_x_size() {
    let _new_field = Field::create(0, 20, Cliff);
}

#[test]
#[should_panic]
fn zero_y_size() {
    let _new_field = Field::create(0, 20, Cliff);
}

#[test]
fn size_ok() {
    let new_field = Field::create(10, 20, Cliff);
    
    assert_eq!(10, new_field.population.len());
    assert_eq!(20, new_field.population[0].len());
}

#[test]
fn zero_alive() {
    let new_field = Field::create(10, 20, Cliff);
    
    assert_eq!(0, new_field.alive);
}

#[test]
fn empty_upon_creation() {
    let new_field = Field::create(10, 20, Cliff);
    
    for column in new_field.population {
        for cell in column {
            assert_eq!(false, cell);
        }
    }
}
