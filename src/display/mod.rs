use crate::field::Cell;

pub enum Id {
    CONSOLE
}

struct Resolution {
    x : u32,
    y : u32
}

pub struct Display {
    id : Id,

    resolution : Resolution    
}

impl Display {
    pub fn create(id : Id, y_resolution : u32, x_resolution : u32) -> Self {
        Self {
            id : id,
            resolution : Resolution {
                x : x_resolution,
                y : y_resolution
            }
        }
    }

    pub fn draw(&self, field : &Vec<Vec<Cell>>) {
        if field.len() > self.resolution.y as usize {
            panic!("Field's Y dimension violates display size");
        } 

        if field[0].len() != self.resolution.x as usize {
            panic!("Field's X dimension violates display size");
        }

        println!("");
        for _top_border in 0..field[0].len() + 2 {
          print!("-");
        }
        println!("");

        for row in field {
            print!("|");
            for cell in row {
                if cell.alive == true {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }

        for _botton_border in 0..field[0].len() + 2 {
          print!("-");
        }
        println!("");
    }
}
