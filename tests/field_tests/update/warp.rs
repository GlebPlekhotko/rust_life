use life::field::*;
use life::field::FenceType::*;

#[test]
fn block_horizontal() {
    let mut block = Field::create(4, 4, Warp);
    let mut expected = Field::create(4, 4, Warp);

    block.population[0][1] = true;
    block.population[0][2] = true;
    block.population[3][1] = true;
    block.population[3][2] = true;

    expected.population[0][1] = true;
    expected.population[0][2] = true;
    expected.population[3][1] = true;
    expected.population[3][2] = true;

    block.update(1);

    assert_eq!(true, equal(&expected, &block));
}

#[test]
fn block_vertical() {
    let mut block = Field::create(4, 4, Warp);
    let mut expected = Field::create(4, 4, Warp);

    block.population[1][0] = true;
    block.population[2][0] = true;
    block.population[1][3] = true;
    block.population[2][3] = true;

    expected.population[1][0] = true;
    expected.population[2][0] = true;
    expected.population[1][3] = true;
    expected.population[2][3] = true;

    block.update(1);

    assert_eq!(true, equal(&expected, &block));
}

#[test]
fn hive_horizontal() {
    let mut hive = Field::create(6, 5, Warp);
    let mut expected = Field::create(6, 5, Warp);

    hive.population[0][1] = true;
    hive.population[0][3] = true;
    hive.population[1][2] = true;
    hive.population[4][2] = true;
    hive.population[5][1] = true;
    hive.population[5][3] = true;

    expected.population[0][1] = true;
    expected.population[0][3] = true;
    expected.population[1][2] = true;
    expected.population[4][2] = true;
    expected.population[5][1] = true;
    expected.population[5][3] = true;

    hive.update(1);

    assert_eq!(true, equal(&expected, &hive));
}

#[test]
fn hive_vertical() {
    let mut hive = Field::create(6, 5, Warp);
    let mut expected = Field::create(6, 5, Warp);

    hive.population[1][0] = true;
    hive.population[3][0] = true;
    hive.population[2][1] = true;
    hive.population[2][3] = true;
    hive.population[1][4] = true;
    hive.population[3][4] = true;

    expected.population[1][0] = true;
    expected.population[3][0] = true;
    expected.population[2][1] = true;
    expected.population[2][3] = true;
    expected.population[1][4] = true;
    expected.population[3][4] = true;

    hive.update(1);

    assert_eq!(true, equal(&expected, &hive));
}

#[test]
fn blinker_horizontal_first_generation() {
    let mut blinker = Field::create(5, 5, Warp);
    let mut expected = Field::create(5, 5, Warp);

    blinker.population[0][1] = true;
    blinker.population[0][2] = true;
    blinker.population[0][3] = true;

    expected.population[0][2] = true;
    expected.population[1][2] = true;
    expected.population[4][2] = true;

    blinker.update(1);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_horizontal_second_generation() {
    let mut blinker = Field::create(5, 5, Warp);
    let mut expected = Field::create(5, 5, Warp);

    blinker.population[0][1] = true;
    blinker.population[0][2] = true;
    blinker.population[0][3] = true;

    expected.population[0][1] = true;
    expected.population[0][2] = true;
    expected.population[0][3] = true;

    blinker.update(2);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_vertical_first_generation() {
    let mut blinker = Field::create(5, 5, Warp);
    let mut expected = Field::create(5, 5, Warp);

    blinker.population[2][0] = true;
    blinker.population[2][1] = true;
    blinker.population[2][4] = true;

    expected.population[1][0] = true;
    expected.population[2][0] = true;
    expected.population[3][0] = true;

    blinker.update(1);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_vertical_second_generation() {
    let mut blinker = Field::create(5, 5, Warp);
    let mut expected = Field::create(5, 5, Warp);

    blinker.population[2][0] = true;
    blinker.population[2][1] = true;
    blinker.population[2][4] = true;

    expected.population[2][0] = true;
    expected.population[2][1] = true;
    expected.population[2][4] = true;

    blinker.update(2);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn toad_horizontal_first_generation() {
    let mut toad = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    toad.population[0][1] = true;
    toad.population[0][2] = true;
    toad.population[0][3] = true;
    toad.population[5][2] = true;
    toad.population[5][3] = true;
    toad.population[5][4] = true;

    expected.population[0][1] = true;
    expected.population[0][4] = true;
    expected.population[1][2] = true;
    expected.population[4][3] = true;
    expected.population[5][1] = true;
    expected.population[5][4] = true; 

    toad.update(1);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn toad_horizontal_second_generation() {
    let mut toad = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    toad.population[0][1] = true;
    toad.population[0][2] = true;
    toad.population[0][3] = true;
    toad.population[5][2] = true;
    toad.population[5][3] = true;
    toad.population[5][4] = true;

    expected.population[0][1] = true;
    expected.population[0][2] = true;
    expected.population[0][3] = true;
    expected.population[5][2] = true;
    expected.population[5][3] = true;
    expected.population[5][4] = true; 

    toad.update(2);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn toad_vertical_first_generation() {
    let mut toad = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    toad.population[2][0] = true;
    toad.population[2][1] = true;
    toad.population[2][5] = true;
    toad.population[3][0] = true;
    toad.population[3][4] = true;
    toad.population[3][5] = true;

    expected.population[1][0] = true;
    expected.population[2][1] = true;
    expected.population[2][4] = true;
    expected.population[3][1] = true;
    expected.population[3][4] = true;
    expected.population[4][5] = true; 

    toad.update(1);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn toad_vertical_second_generation() {
    let mut toad = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    toad.population[2][0] = true;
    toad.population[2][1] = true;
    toad.population[2][5] = true;
    toad.population[3][0] = true;
    toad.population[3][4] = true;
    toad.population[3][5] = true;

    expected.population[2][0] = true;
    expected.population[2][1] = true;
    expected.population[2][5] = true;
    expected.population[3][0] = true;
    expected.population[3][4] = true;
    expected.population[3][5] = true; 

    toad.update(2);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn glider_first_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][4] = true;
    expected.population[4][3] = true;
    expected.population[4][5] = true;
    expected.population[5][4] = true;
    expected.population[5][5] = true;     

    glider.update(1);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_second_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][4] = true;
    expected.population[0][5] = true;
    expected.population[4][5] = true;
    expected.population[5][3] = true;
    expected.population[5][5] = true;     

    glider.update(2);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_third_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][4] = true;
    expected.population[0][5] = true;
    expected.population[4][4] = true;
    expected.population[5][0] = true;
    expected.population[5][5] = true;     

    glider.update(3);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fourth_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][0] = true;
    expected.population[0][4] = true;
    expected.population[0][5] = true;
    expected.population[4][5] = true;
    expected.population[5][0] = true;     

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fifth_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[0][0] = true;
    expected.population[0][5] = true;
    expected.population[1][5] = true;
    expected.population[5][0] = true;
    expected.population[5][4] = true;     

    glider.update(5);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_twelveth_generation() {
    let mut glider = Field::create(6, 6, Warp);
    let mut expected = Field::create(6, 6, Warp);

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

    glider.update(12);

    assert_eq!(true, equal(&expected, &glider));
}
