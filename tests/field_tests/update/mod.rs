mod cliff;
mod fade_away;
mod warp;

use life::field::*;
use life::field::FenceType::*;

#[test]
fn block() {
    let mut block = Field::create(4, 4, Cliff);
    let mut expected = Field::create(4, 4, Cliff);

    block.population[1][1] = true;
    block.population[1][2] = true;
    block.population[2][1] = true;
    block.population[2][2] = true;

    expected.population[1][1] = true;
    expected.population[1][2] = true;
    expected.population[2][1] = true;
    expected.population[2][2] = true;

    block.update(1);

    assert_eq!(true, equal(&expected, &block));
}

#[test]
fn hive() {
    let mut hive = Field::create(6, 5, Cliff);
    let mut expected = Field::create(6, 5, Cliff);

    hive.population[1][2] = true;
    hive.population[1][3] = true;
    hive.population[2][1] = true;
    hive.population[2][4] = true;
    hive.population[3][2] = true;
    hive.population[3][3] = true;

    expected.population[1][2] = true;
    expected.population[1][3] = true;
    expected.population[2][1] = true;
    expected.population[2][4] = true;
    expected.population[3][2] = true;
    expected.population[3][3] = true;

    hive.update(1);

    assert_eq!(true, equal(&expected, &hive));
}

#[test]
fn blinker() {
    let mut blinker = Field::create(5, 5, Cliff);
    let mut expected = Field::create(5, 5, Cliff);

    blinker.population[2][1] = true;
    blinker.population[2][2] = true;
    blinker.population[2][3] = true;

    expected.population[1][2] = true;
    expected.population[2][2] = true;
    expected.population[3][2] = true;

    blinker.update(1);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn toad() {
    let mut toad = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    toad.population[2][2] = true;
    toad.population[2][3] = true;
    toad.population[2][4] = true;
    toad.population[3][1] = true;
    toad.population[3][2] = true;
    toad.population[3][3] = true;    

    expected.population[1][3] = true;
    expected.population[2][1] = true;
    expected.population[2][4] = true;
    expected.population[3][1] = true;
    expected.population[3][4] = true;
    expected.population[4][2] = true; 

    toad.update(1);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn glider() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[1][2] = true;
    glider.population[2][3] = true;
    glider.population[3][1] = true;
    glider.population[3][2] = true;
    glider.population[3][3] = true;

    expected.population[2][3] = true;
    expected.population[3][4] = true;
    expected.population[4][2] = true;
    expected.population[4][3] = true;
    expected.population[4][4] = true;     

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}
