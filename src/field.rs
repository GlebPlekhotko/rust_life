pub enum FenceType {
    Cliff,
    FadeAway,
    Warp
}

enum FieldSide {
    Inside,
    Outside,
    Top, 
    Bottom, 
    Left,
    Right,
}

struct Cell {
    kill : bool,
    hatch : bool
}

struct Fence {

    top_population : Vec<Vec<bool>>,
    top_cell : Vec<Vec<Cell>>,
    
    bottom_population : Vec<Vec<bool>>,
    bottom_cell : Vec<Vec<Cell>>,
    
    left_population : Vec<Vec<bool>>,
    left_cell : Vec<Vec<Cell>>,
    
    right_population : Vec<Vec<bool>>,
    right_cell : Vec<Vec<Cell>>
}

pub struct Field {
    pub population : Vec<Vec<bool>>,
    
    fence : Option<Fence>,
    fence_type : FenceType,

    cell : Vec<Vec<Cell>>,
    
    pub cells_total : u32,
    pub alive : u32,
}

pub fn equal(first : &Field, second : &Field) -> bool {
    let mut are_equal = true;
    
    if first.population.len() != second.population.len() {
        are_equal = false;
    }
    
    if first.population[0].len() != second.population[0].len() {
        are_equal = false;
    }
    
    if are_equal == true {
        let mut first_column_iter = first.population.iter();
        let mut second_column_iter = second.population.iter();
        
        are_equal = 'columns_loop : loop {
            let first_column = first_column_iter.next();
            let second_column = second_column_iter.next();
            
            match first_column {
                None => break 'columns_loop true,
                _ => ()
            }
        
            let mut first_cell_iter = first_column.iter();
            let mut second_cell_iter = second_column.iter(); 
            
            'cells_loop : loop {
                let first_cell = first_cell_iter.next();
                let second_cell = second_cell_iter.next();
                
                match first_cell {
                    None => break 'cells_loop,
                    _ => ()
                }
                
                if first_cell != second_cell {
                    break 'columns_loop false;
                }
            }
        };   
    }
    
    are_equal
}

impl Field {
    pub fn create(x_size : u32, y_size : u32, fence : FenceType) -> Self {
        if x_size == 0 || y_size == 0 {
            panic!("Field cannot be zero in size!");
        }
        
        let mut new_field = Self {
            population : Vec::new(),
            
            fence : None,
            fence_type : fence,
            
            cell : Vec::new(),
            
            cells_total : x_size * y_size,
            alive : 0
        };
        
        for x in 0..x_size {
            new_field.population.push(Vec::new());
            new_field.cell.push(Vec::new());
            
            for _y in 0..y_size {
                new_field.population[x as usize].push(false);
                new_field.cell[x as usize].push(Cell{kill : false, hatch : false});
            }
        }
        
        /* If instance is set to have the "Fade Away" approach to the items going out of the population area, it needs
           some hidden cells. "Hidden" because we don't want to show them to the entity using the instance and so 
           cannot include them plainly into the population area. Rather we define them as distinct arrays. Assumed 
           layout is as following:
              _______________________
             |          Top          |
             |_______________________|
             |   |               |   |
             | L |               | R |
             | e |  Population   | i |
             | f |     Area      | g |
             | t |               | h |
             |   |               | t | 
             |___|_______________|___| 
             |        Bottom         |
             |_______________________|
            
           Here we have "Population Area" of "X" by "Y" cells size. "Hidden" areas must be by "Z" items larger in each 
           direction, where "Z" is the size of the largest pattern expected to leave the "Population Area". For the 
           "Glider" pattern "Z = 3":
              O  --
               O  3
             OOO --
             | |
             |3| */
        
        if let FenceType::FadeAway = new_field.fence_type {
            let largest_pattern = 3;
            
            new_field.fence = Some( 
                Fence { 
                    top_population : Vec::new(),
                    top_cell : Vec::new(),
                    
                    bottom_population : Vec::new(),
                    bottom_cell : Vec::new(),
                    
                    left_population : Vec::new(),
                    left_cell : Vec::new(),
                    
                    right_population : Vec::new(),
                    right_cell : Vec::new(),
                } 
            );
            
            let mut fence = new_field.fence.as_mut().unwrap();
            
            // Here top and bottom fences are created. Their width must be "multiply by 2" of the largest pattern
            for x in 0..(x_size + largest_pattern * 2) {
                fence.top_population.push(Vec::new());
                fence.top_cell.push(Vec::new());
                
                fence.bottom_population.push(Vec::new());
                fence.bottom_cell.push(Vec::new());
                
                for _y in 0..largest_pattern {
                    fence.top_population[x as usize].push(false);
                    fence.top_cell[x as usize].push(Cell{kill : false, hatch : false});
                    
                    fence.bottom_population[x as usize].push(false);
                    fence.bottom_cell[x as usize].push(Cell{kill : false, hatch : false});
                }
            }
            
            for x in 0..largest_pattern {
                fence.left_population.push(Vec::new());
                fence.left_cell.push(Vec::new());
                
                fence.right_population.push(Vec::new());
                fence.right_cell.push(Vec::new());
                
                for _y in 0..y_size {
                    fence.left_population[x as usize].push(false);
                    fence.left_cell[x as usize].push(Cell{kill : false, hatch : false});
                    
                    fence.right_population[x as usize].push(false);
                    fence.right_cell[x as usize].push(Cell{kill : false, hatch : false});
                }
            }
        }
        
        new_field
    }
    
    fn cell(&mut self, x : i32, y : i32) -> &mut Cell {
        let (side, x, y) = self.translate_coordinates(x, y);
        
        match side {
            FieldSide::Inside => &mut self.cell[x][y],
            FieldSide::Top    => &mut self.fence.as_mut().unwrap().top_cell[x][y],
            FieldSide::Bottom => &mut self.fence.as_mut().unwrap().bottom_cell[x][y],
            FieldSide::Left   => &mut self.fence.as_mut().unwrap().left_cell[x][y],
            FieldSide::Right  => &mut self.fence.as_mut().unwrap().right_cell[x][y],
            FieldSide::Outside => panic!("Non-existing cell referenced")
        }
    }
    
    fn inhabitant(&mut self, x : i32, y : i32) -> &mut bool {
        let (side, x, y) = self.translate_coordinates(x, y);
        
        match side {
            FieldSide::Inside => &mut self.population[x][y],
            FieldSide::Top    => &mut self.fence.as_mut().unwrap().top_population[x][y],
            FieldSide::Bottom => &mut self.fence.as_mut().unwrap().bottom_population[x][y],
            FieldSide::Left   => &mut self.fence.as_mut().unwrap().left_population[x][y],
            FieldSide::Right  => &mut self.fence.as_mut().unwrap().right_population[x][y],
            FieldSide::Outside => panic!("Non-existing inhabitant referenced")
        }
    }
    
    pub fn populate_randomly(&mut self, density : f32) {
        if (density == 0.0) || (density >= 1.0) {
            panic!("Population density must be above 0 and below 1.0");
        }

        self.alive = 0;

        for colum in &mut self.population {
            for cell in colum {
                if rand::random::<f32>() < density {
                    *cell = true;
                    self.alive += 1;
                } else {
                    *cell = false;
                }
            }
        }
    }
    
    fn translate_coordinates(&self, mut x : i32, mut y : i32) -> (FieldSide, usize, usize) {
        let x_max = self.population.len() as i32;
        let y_max = self.population[0].len() as i32;
        let mut side : FieldSide = FieldSide::Inside;
        
        loop {
            if let FenceType::Cliff = self.fence_type {
                if x < 0 {
                    x = 0;
                    side = FieldSide::Outside;
                }
                
                if x >= x_max {
                    x = x_max - 1;
                    side = FieldSide::Outside;
                }
                 
                if y < 0 {
                    y = 0;
                    side = FieldSide::Outside;
                }
                 
                if y >= y_max {
                    y = y_max - 1;
                    side = FieldSide::Outside;
                }
                 
                break;
            }

            if let FenceType::Warp = self.fence_type {
                if x < 0 {
                    x = x_max - 1;
                }
                
                if x >= x_max {
                    x = 0;
                }
                 
                if y < 0 {
                    y = y_max - 1;
                }
                 
                if y >= y_max {
                    y = 0;
                }

                side = FieldSide::Inside;

                break;
            }
            
            /* If "y" coordinate is negative, then we address the cell in the "top" fence area which possesses the
               following layout:
               
               O-----------------------------------------------> x'
               |                 Top Off Fence
               | <- (x'-x)/2 -> .             .  <- (x'-x)/2 -> 
               |                .             .
               V ---------------O-------------> x
               y'               |
                                | Population
                                | 
                                V
                                y
               
               So to translate the negative "y" coordinate, we must add the "height" off the "off-fence" area. The "x"
               coordinate (which might be both negative and positive) we must add half of the "off-fence" width minus
               the width of the "population" area. */
            if y < 0 {
                y = y + self.fence.as_ref().unwrap().top_population[0].len() as i32;
                x = x + self.fence.as_ref().unwrap().left_population.len() as i32;
                side = FieldSide::Top;
                break;
            }
            
            /* If "y" coordinate is greater, than "height" of the population area, then we should address the "bottom"
               off fence area:
               
                                O-------------> x
                                |
                                | Population
                                | 
               O----------------V------------------------------> x'
               |                .             .
               | <- (x'-x)/2 -> .             .  <- (x'-x)/2 -> 
               |                .             .
               V               Bottom Off Fence
               y'
               
               The "y" coordinate is obtained by subtracting the "height" of the population area and adding height of the
               off-fence area. Translation of the "x" coordinate is the same as for the previous case. */
            if y >= y_max {
                y = y - y_max;
                x = x + self.fence.as_ref().unwrap().left_population.len() as i32;
                side = FieldSide::Bottom;
                break;
            }
            
            /* Negative "x" coordinate addresses the left "off-fence" area:
            
                               x'
               O--------------->O-------------> x
               |                |
               | Left Off Fence | Population
               |                | 
               V                V
               y'               y
               
               The "y" coordinate does not need translation in this case, while for the "x" it's enough to add the width of
               the "off-fence" area. */
            if x < 0 {
                x = x + self.fence.as_ref().unwrap().left_population.len() as i32;
                side = FieldSide::Left;
                break;
            }
            
            /* The "x" greater than width of the "population" area addresses the right "off-fence" area:
            
                               x'
               O------------->O---------------> x
               |              |                
               | Population   | Right Off Fence 
               |              |                
               V              V
               y'             y
               
               The "y" coordinate remains the same. The "x" coordinate must be subtracted width of the "population" 
               area. */
            if x >= x_max {
                x = x - x_max;
                side = FieldSide::Right;
                break;
            }
            break;
        }
        
        if x < 0 || y < 0 {
            side = FieldSide::Outside;
        }
        
        (side, x as usize, y as usize)
    }
    
    fn alive(&mut self, x : i32, y : i32) -> bool {
        return *self.inhabitant(x, y);
    }
    
    fn neighbours(&mut self, x : i32, y : i32) -> u32 {
        let x_width = self.population.len() as i32;
        let y_width = self.population[0].len() as i32;
        let mut neighbours = 0;
        
        for x_neighbour in x - 1 ..= x + 1 {
            for y_neighbour in y - 1 ..= y + 1 {
                let (neighbour_side, _, _) = self.translate_coordinates(x_neighbour, y_neighbour);
                
                if let FieldSide::Outside = neighbour_side {
                    continue;
                }
                
                if x_neighbour == x && y_neighbour == y {
                    continue;
                }
                
                if self.alive(x_neighbour, y_neighbour) == true {
                    neighbours += 1;
                }
            }
        }
        
        neighbours
    }
    
    pub fn update(&mut self, cycles : u32) {
        let mut x_start = 0 as i32;
        let mut x_end = self.population.len() as i32;
        let mut y_start = 0 as i32;
        let mut y_end = self.population[0].len() as i32;
        
        if let FenceType::FadeAway = self.fence_type {
            x_start = x_start - self.fence.as_ref().unwrap().left_population.len() as i32;
            x_end = x_end + self.fence.as_ref().unwrap().right_population.len() as i32 - 1;
            
            y_start = y_start - self.fence.as_ref().unwrap().top_population[0].len() as i32;
            y_end = y_end + self.fence.as_ref().unwrap().bottom_population[0].len() as i32 - 1;
        }

        for _cycle in 0..cycles {
            for x in x_start..x_end {
                for y in y_start..y_end {
                    let cell_neighbours = self.neighbours(x, y);
                    
                    if self.alive(x, y) == true {
                        if cell_neighbours < 2 || cell_neighbours > 3 {
                            self.cell(x, y).kill = true;
                            self.cell(x, y).hatch = false;
                        }
                    } else {
                        if cell_neighbours == 3 {
                            self.cell(x, y).kill = true;
                            self.cell(x, y).hatch = true;
                        }
                    }
                }
            }
            
            for x in x_start..x_end {
                for y in y_start..y_end {
                    if self.alive(x, y) == true {
                        if self.cell(x, y).kill == true {
                            *(self.inhabitant(x, y)) = false;
                        }
                    } else {
                        if self.cell(x, y).hatch == true {
                            *(self.inhabitant(x, y)) = true;
                        }
                    }                   
                    
                    self.cell(x, y).kill = false;
                    self.cell(x, y).hatch = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn translate_coordinates_inside_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(0, 0);
        
        assert!(if let FieldSide::Inside = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }

    #[test]
    fn translate_coordinates_inside_max_max() {

        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(9, 19);
        
        assert!(if let FieldSide::Inside = side { true } else { false });
        assert_eq!(9, x);
        assert_eq!(19, y);
    }
    
    #[test]
    fn translate_coordinates_top_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(-3, -3);
        
        assert!(if let FieldSide::Top = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_top_max_max() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(12, -1);
        
        assert!(if let FieldSide::Top = side { true } else { false });
        assert_eq!(15, x);
        assert_eq!(2, y);
    }
    
    #[test]
    fn translate_coordinates_bottom_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(-3, 20);
        
        assert!(if let FieldSide::Bottom = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_bottom_max_max() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(12, 22);
        
        assert!(if let FieldSide::Bottom = side { true } else { false });
        assert_eq!(15, x);
        assert_eq!(2, y);
    }
    
    #[test]
    fn translate_coordinates_left_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(-3, 0);
        
        assert!(if let FieldSide::Left = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_left_max_max() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(-1, 19);
        
        assert!(if let FieldSide::Left = side { true } else { false });
        assert_eq!(2, x);
        assert_eq!(19, y);
    }
    
    #[test]
    fn translate_coordinates_right_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(10, 0);
        
        assert!(if let FieldSide::Right = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_right_max_max() {
        let test_field = Field::create(10, 20, FenceType::FadeAway);
        
        let (side, x, y) = test_field.translate_coordinates(12, 19);
        
        assert!(if let FieldSide::Right = side { true } else { false });
        assert_eq!(2, x);
        assert_eq!(19, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_negative_x() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(-1, 0);
        
        assert!(if let FieldSide::Outside = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_over_max_x() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(10, 0);
        
        assert!(if let FieldSide::Outside = side { true } else { false });
        assert_eq!(9, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_negative_y() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(0, -1);
        
        assert!(if let FieldSide::Outside = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_over_max_y() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(0, 20);
        
        assert!(if let FieldSide::Outside = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(19, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_zero_zero() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(0, 0);
        
        assert!(if let FieldSide::Inside = side { true } else { false });
        assert_eq!(0, x);
        assert_eq!(0, y);
    }
    
    #[test]
    fn translate_coordinates_cliff_max_max() {
        let test_field = Field::create(10, 20, FenceType::Cliff);
        
        let (side, x, y) = test_field.translate_coordinates(9, 19);
        
        assert!(if let FieldSide::Inside = side { true } else { false });
        assert_eq!(9, x);
        assert_eq!(19, y);
    }
}
