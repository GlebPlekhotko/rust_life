use rand;

pub struct Cell {
    pub alive : bool,
    
    kill : bool,
    hatch : bool
}

pub fn create(y_width : u32, x_width : u32) -> Vec<Vec<Cell>> {
    let mut field : Vec<Vec<Cell>> = Vec::new();

    if x_width == 0 {
        panic!("Field's X dimension cannot be zero");    
    }

    if y_width == 0 {
        panic!("Field's Y dimension cannot be zero");    
    }

    for y in 0..y_width {
        field.push(Vec::new());

        for _x in 0..x_width {
            field[y as usize].push(Cell{alive: false, kill: false, hatch: false});
        }
    }

    field
}

fn equal(first_field : &Vec<Vec<Cell>>, second_field : &Vec<Vec<Cell>>) -> bool {
    if first_field.len() != second_field.len() || first_field[0].len() != second_field[0].len() {
        panic!("Fields' size not matched");
    }

    let mut fields_equal = true;

    'compare_loop: 
    for y in 0..first_field.len() {
        for x in 0..first_field[0].len() {
            if first_field[y][x].alive != second_field[y][x].alive {
                fields_equal = false;
                break 'compare_loop;
            }
        }     
    }

    fields_equal
}

pub fn random_populate(field : &mut Vec<Vec<Cell>>, density : f32) {
    if density >= 1.0 {
        panic!("Random population density must be below 1.0");
    }

    for row in field {
        for cell in row { 
            if rand::random::<f32>() < density {           
                cell.alive = true;
            } else {
                cell.alive = false;
            }
        }
    }
}

pub fn update(field : &mut Vec<Vec<Cell>>) {
    let x_width = field[0].len() as i32;
    let y_width = field.len() as i32;

    for x in 0..x_width {
        for y in 0..y_width {
            let mut cell_neighbours : u32 = 0;
            
            for x_neighbour in x - 1 ..= x + 1 {
                for y_neighbour in y - 1 ..= y + 1 {

                    if x_neighbour == -1 || x_neighbour == x_width  {
                        continue;
                    }

                    if y_neighbour == -1 || y_neighbour == y_width  {
                        continue;
                    }
                    
                    if x_neighbour == x && y_neighbour == y {
                        continue
                    }

                    if field[y_neighbour as usize][x_neighbour as usize].alive == true {
                        cell_neighbours += 1;
                    }
                }
            }
            
            let cell : &mut Cell = &mut field[y as usize][x as usize];
            if cell.alive == true {
                if cell_neighbours < 2 || cell_neighbours > 3 {
                    cell.kill = true;
                }
            } else {
                if cell_neighbours == 3 {
                    cell.hatch = true;
                }
            }
        }
    }

    for row in field {
        for cell in row {
            if cell.alive == true {
                if cell.kill == true {
                    cell.alive = false;
                }
            } else {
                if cell.hatch == true {
                    cell.alive = true;                
                }
            }

            cell.kill = false;
            cell.hatch = false;
        }
    }
}

//==================== Tests ====================

#[test]
fn empty_upon_creation() {
    let field = create(3,3);

    for column in field {
        for cell in column {
            assert_eq!(false, cell.alive);
        }        
    }    
}

#[test]
fn equal_fields() {
    let first_field = create(3,3);
    let second_field = create(3,3);

    assert_eq!(true, equal(&first_field, &second_field));    
}

#[test]
fn not_equal_fields() {
    let mut first_field = create(3,3);
    let mut second_field = create(3,3);

    first_field[1][1].alive = true;

    assert_eq!(false, equal(&first_field, &second_field));    
}

#[test]
fn block() {
    let mut block = create(4,4);
    let mut expected = create(4,4);

    block[1][1].alive = true;
    block[1][2].alive = true;
    block[2][1].alive = true;
    block[2][2].alive = true;

    expected[1][1].alive = true;
    expected[1][2].alive = true;
    expected[2][1].alive = true;
    expected[2][2].alive = true;

    update(&mut block);

    assert_eq!(true, equal(&expected, &block));    
}

#[test]
fn hive() {
    let mut hive = create(5,6);
    let mut expected = create(5,6);

    hive[1][2].alive = true;
    hive[1][3].alive = true;
    hive[2][1].alive = true;
    hive[2][4].alive = true;
    hive[3][2].alive = true;
    hive[3][3].alive = true;

    expected[1][2].alive = true;
    expected[1][3].alive = true;
    expected[2][1].alive = true;
    expected[2][4].alive = true;
    expected[3][2].alive = true;
    expected[3][3].alive = true;

    update(&mut hive);

    assert_eq!(true, equal(&expected, &hive));    
}

#[test]
fn blinker() {
    let mut blinker = create(5,5);
    let mut expected = create(5,5);

    blinker[2][1].alive = true;
    blinker[2][2].alive = true;
    blinker[2][3].alive = true;

    expected[1][2].alive = true;
    expected[2][2].alive = true;
    expected[3][2].alive = true;

    update(&mut blinker);

    assert_eq!(true, equal(&expected, &blinker));
}

#[test]
fn toad() {
    let mut toad = create(6,6);
    let mut expected = create(6,6);

    toad[2][2].alive = true;
    toad[2][3].alive = true;
    toad[2][4].alive = true;
    toad[3][1].alive = true;
    toad[3][2].alive = true;
    toad[3][3].alive = true;    

    expected[1][3].alive = true;
    expected[2][1].alive = true;
    expected[2][4].alive = true;
    expected[3][1].alive = true;
    expected[3][4].alive = true;
    expected[4][2].alive = true; 

    update(&mut toad);

    assert_eq!(true, equal(&expected, &toad));
}

#[test]
fn glider() {
    let mut glider = create(6,6);
    let mut expected = create(6,6);

    glider[1][2].alive = true;
    glider[2][3].alive = true;
    glider[3][1].alive = true;
    glider[3][2].alive = true;
    glider[3][3].alive = true;

    expected[2][3].alive = true;
    expected[3][4].alive = true;
    expected[4][2].alive = true;
    expected[4][3].alive = true;
    expected[4][4].alive = true;     

    for _cycle in 0..4 {
        update(&mut glider);
    }

    assert_eq!(true, equal(&expected, &glider));
}
