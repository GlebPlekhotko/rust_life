use life::field::*;
use life::field::FenceType::*;

#[test]
fn block() {
    let mut block = Field::create(4, 4, Cliff);
    let mut expected = Field::create(4, 4, Cliff);

    block.population[2][2] = true;
    block.population[2][3] = true;
    block.population[3][2] = true;
    block.population[3][3] = true;

    expected.population[2][2] = true;
    expected.population[2][3] = true;
    expected.population[3][2] = true;
    expected.population[3][3] = true;

    block.update(1);

    assert_eq!(true, equal(&expected, &block));
}

#[test]
fn hive() {
    let mut hive = Field::create(6, 5, Cliff);
    let mut expected = Field::create(6, 5, Cliff);

    hive.population[2][3] = true;
    hive.population[3][2] = true;
    hive.population[3][4] = true;
    hive.population[4][2] = true;
    hive.population[4][4] = true;
    hive.population[5][3] = true;

    expected.population[2][3] = true;
    expected.population[3][2] = true;
    expected.population[3][4] = true;
    expected.population[4][2] = true;
    expected.population[4][4] = true;
    expected.population[5][3] = true;

    hive.update(1);

    assert_eq!(true, equal(&expected, &hive));
}

#[test]
fn blinker_first_generation() {
    let mut blinker = Field::create(5, 5, Cliff);
    let mut expected = Field::create(5, 5, Cliff);

    blinker.population[2][4] = true;
    blinker.population[3][4] = true;
    blinker.population[4][4] = true;

    expected.population[3][3] = true;
    expected.population[3][4] = true;

    blinker.update(1);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_second_generation() {
    let mut blinker = Field::create(5, 5, Cliff);
    let expected = Field::create(5, 5, Cliff);

    blinker.population[2][4] = true;
    blinker.population[3][4] = true;
    blinker.population[4][4] = true;

    blinker.update(2);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn toad_first_generation() {
    let mut toad = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    toad.population[4][3] = true;
    toad.population[4][4] = true;
    toad.population[4][5] = true;
    toad.population[5][2] = true;
    toad.population[5][3] = true;
    toad.population[5][4] = true;    

    expected.population[3][4] = true;
    expected.population[4][2] = true;
    expected.population[4][5] = true;
    expected.population[5][2] = true;
    expected.population[5][5] = true;

    toad.update(1);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn toad_second_generation() {
    let mut toad = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    toad.population[4][3] = true;
    toad.population[4][4] = true;
    toad.population[4][5] = true;
    toad.population[5][2] = true;
    toad.population[5][3] = true;
    toad.population[5][4] = true;    

    expected.population[4][3] = true;
    expected.population[4][4] = true;
    expected.population[4][5] = true;

    toad.update(2);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn glider_first_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][3] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(1);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_second_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(2);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_third_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][4] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(3);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fourth_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][4] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fifth_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][4] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_sixth_generation() {
    let mut glider = Field::create(6, 6, Cliff);
    let mut expected = Field::create(6, 6, Cliff);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][4] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}
