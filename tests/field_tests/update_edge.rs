use life::field::*;
use life::field::FenceType::*;

#[test]
fn block_cliff() {
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
fn block_fade_away() {
    let mut block = Field::create(4, 4, FadeAway);
    let mut expected = Field::create(4, 4, FadeAway);

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
fn hive_cliff() {
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
fn hive_fade_away() {
    let mut hive = Field::create(6, 5, FadeAway);
    let mut expected = Field::create(6, 5, FadeAway);

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
fn blinker_cliff_first_generation() {
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
fn blinker_cliff_second_generation() {
    let mut blinker = Field::create(5, 5, Cliff);
    let mut expected = Field::create(5, 5, Cliff);

    blinker.population[2][4] = true;
    blinker.population[3][4] = true;
    blinker.population[4][4] = true;

    blinker.update(2);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_fade_away_first_generation() {
    let mut blinker = Field::create(5, 5, FadeAway);
    let mut expected = Field::create(5, 5, FadeAway);

    blinker.population[2][4] = true;
    blinker.population[3][4] = true;
    blinker.population[4][4] = true;

    expected.population[3][3] = true;
    expected.population[3][4] = true;

    blinker.update(1);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn blinker_fade_away_second_generation() {
    let mut blinker = Field::create(5, 5, FadeAway);
    let mut expected = Field::create(5, 5, FadeAway);

    blinker.population[2][4] = true;
    blinker.population[3][4] = true;
    blinker.population[4][4] = true;

    expected.population[2][4] = true;
    expected.population[3][4] = true;
    expected.population[4][4] = true;

    blinker.update(2);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn toad_cliff_first_generation() {
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
fn toad_cliff_second_generation() {
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
fn toad_fade_away_first_generation() {
    let mut toad = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

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
fn toad_fade_away_second_generation() {
    let mut toad = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    toad.population[4][3] = true;
    toad.population[4][4] = true;
    toad.population[4][5] = true;
    toad.population[5][2] = true;
    toad.population[5][3] = true;
    toad.population[5][4] = true;    

    expected.population[4][3] = true;
    expected.population[4][4] = true;
    expected.population[4][5] = true;
    expected.population[5][2] = true;
    expected.population[5][3] = true;
    expected.population[5][4] = true;    

    toad.update(2);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn glider_cliff_first_generation() {
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
fn glider_cliff_second_generation() {
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
fn glider_cliff_third_generation() {
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
fn glider_cliff_fourth_generation() {
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
fn glider_cliff_fifth_generation() {
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
fn glider_cliff_sixth_generation() {
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
fn glider_fade_away_first_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

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
fn glider_fade_away_second_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][5] = true;
    expected.population[5][3] = true;
    expected.population[5][5] = true;

    glider.update(2);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fade_away_third_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][4] = true;
    expected.population[5][5] = true;

    glider.update(3);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fade_away_fourth_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[4][5] = true;

    glider.update(4);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fade_away_fifth_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    expected.population[5][4] = true;

    glider.update(5);

    assert_eq!(true, equal(&expected, &glider));
}

#[test]
fn glider_fade_away_sixth_generation() {
    let mut glider = Field::create(6, 6, FadeAway);
    let mut expected = Field::create(6, 6, FadeAway);

    glider.population[3][4] = true;
    glider.population[4][5] = true;
    glider.population[5][3] = true;
    glider.population[5][4] = true;
    glider.population[5][5] = true;

    glider.update(6);

    assert_eq!(true, equal(&expected, &glider));
}
