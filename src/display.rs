/// The function used print out content of the field to the standard output

/**
    Nothing fancy inside. Just the two-dimensional array of "O" and " " characters depicting the alive and dead cells
    respectively. And the primitive borders around. Note that the function has no information about the console's 
    capacity, so the output may be easily distorted by the automatic line wraps.
*/
pub fn draw(field : &Vec<Vec<bool>>) {
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

    for _bottom_border in 0..field.len() + 2 {
      print!("-");
    }
    println!("");
}
