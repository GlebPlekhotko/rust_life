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
