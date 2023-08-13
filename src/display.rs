pub enum Id {
    CONSOLE
}

struct Resolution {
    x : usize,
    y : usize
}

pub struct Display {
    id : Id,

    resolution : Resolution    
}

impl Display {
    pub fn create(id : Id, x_resolution : usize, y_resolution : usize) -> Self {
        Self {
            id : id,
            resolution : Resolution {
                x : x_resolution,
                y : y_resolution
            }
        }
    }

    pub fn draw(&self, field : &Vec<Vec<bool>>) {
        if field.len() > self.resolution.x {
            panic!("Field's X dimension violates display size");
        } 

        if field[0].len() != self.resolution.y {
            panic!("Field's Y dimension violates display size");
        }

        println!("");
        for _top_border in 0..field.len() + 2 {
          print!("-");
        }
        println!("");

        for row in 0..field[0].len() {
            print!("|");
            for column in field {
                if (*column)[row] == true {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }

        for _botton_border in 0..field.len() + 2 {
          print!("-");
        }
        println!("");
    }
}
