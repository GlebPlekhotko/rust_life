pub enum FieldFenceType {
    FieldFenceTypeCliff,
    FieldFenceTypeFadeAway
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
    right_cell : Vec<Vec<Cell>>,
}

pub struct Field {
    pub population : Vec<Vec<bool>>,
    
    fence : Option<Fence>,
    fence_type : FieldFenceType,

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
    pub fn create(x_size : u32, y_size : u32, fence : FieldFenceType) -> Self {
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
        
        if let FieldFenceType::FieldFenceTypeFadeAway = new_field.fence_type {
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
    
    // Process cells which belong to the "out of population" area.
    fn off_fence_alive(&self, mut x : i32, mut y : i32) -> bool {
        let x_max = self.population.len() as i32;
        let y_max = self.population[0].len() as i32;
        let mut alive = false;
        
        if let FieldFenceType::FieldFenceTypeCliff = self.fence_type {
            return false;
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
            x = x + (self.fence.as_ref().unwrap().left_population.len() as i32 - self.population.len() as i32 / 2);
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
        if y > y_max {
            y = y - y_max + self.fence.as_ref().unwrap().top_population[0].len() as i32;
            x = x + (self.fence.as_ref().unwrap().left_population.len() as i32 - self.population.len() as i32 / 2);
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
        if x > x_max {
            x = x - x_max - self.fence.as_ref().unwrap().right_population.len() as i32;
        }
        
        match self.fence_type {
            FieldFenceType::FieldFenceTypeCliff => 
                alive = false,
            _ => 
                alive = false
        }
        
        return alive;
    }
    
    fn count_neighbours(&self, x : i32, y : i32) -> u32 {
        let x_width = self.population.len() as i32;
        let y_width = self.population[0].len() as i32;
        let mut neighbours = 0;
        
        for x_neighbour in x - 1 ..= x + 1 {
            for y_neighbour in y - 1 ..= y + 1 {
                
                if (x_neighbour == -1) || (x_neighbour == x_width) {
                    if self.off_fence_alive(x, y) == true {
                        neighbours += 1;
                    }
                }

                if (y_neighbour == -1) || (y_neighbour == y_width) {
                    if self.off_fence_alive(x, y) == true {
                        neighbours += 1;
                    }
                }
                
                if x_neighbour == x && y_neighbour == y {
                    continue
                }

                if self.population[x_neighbour as usize][y_neighbour as usize] == true {
                    neighbours += 1;
                }
            }
        }
        
        neighbours
    }
    
    pub fn update(&mut self, cycles : u32) {
        let x_width = self.population.len() as i32;
        let y_width = self.population[0].len() as i32;

        for _cycle in 0..cycles {
            for x in 0..x_width {
                for y in 0..y_width {
                    let mut cell_neighbours : u32 = 0;
                    
                    cell_neighbours = self.count_neighbours(x, y);
                    
                    if self.population[x as usize][y as usize] == true {
                        if cell_neighbours < 2 || cell_neighbours > 3 {
                            self.cell[x as usize][y as usize].kill = true;
                            self.cell[x as usize][y as usize].hatch = false;
                        }
                    } else {
                        if cell_neighbours == 3 {
                            self.cell[x as usize][y as usize].kill = true;
                            self.cell[x as usize][y as usize].hatch = true;
                        }
                    }
                }
            }
            
            for x in 0..x_width {
                for y in 0..y_width {
                    if self.population[x as usize][y as usize] == true {
                        if self.cell[x as usize][y as usize].kill == true {
                            self.population[x as usize][y as usize] = false;
                        }
                    } else {
                        if self.cell[x as usize][y as usize].hatch == true {
                            self.population[x as usize][y as usize] = true;
                        }
                    }
                    
                    self.cell[x as usize][y as usize].kill = false;
                    self.cell[x as usize][y as usize].hatch = false;
                }
            }
        }
    }
}
