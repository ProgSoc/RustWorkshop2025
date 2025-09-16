// This is the import of the `input_macro` crate
// that allows us to take user input conveniently.
use input_macro::input;

fn direction_index(dir: &str) -> i32 {
    match dir {
        "left" => 0,
        "right" => 1,
        "up" => 2,
        "down" => 3,
        _ => 4,
    }
}

pub fn main() {

    // Since we'll be mutating our position,
    // which is expressed in terms of `x` and `y`,
    // we need to mark these variables as mutable.
    let mut x = 0;
    let mut y = 0;

    // The `println!` macro takes a format string,
    // and automatically binds the values of variables
    // when the variable names are found inside curly brackets.
    println!("Start position: ({x}, {y})");

    for _ in 0..6 {
        let direction = input!("Input direction to move: ").to_lowercase();

        // We're hard coding integers for now,
        // since we know exactly how `direction_index` works.
        // As you'll see later in the workshop,
        // there are better ways to go about this,
        // but to demonstrate `match` statements for now,
        // this is still useful.
        match direction_index(&direction) {
            0 => {
                println!("Moved left.");
                x -= 1;
            }
            1 => {
                println!("Moved right.");
                x += 1;
            }
            2 => {
                println!("Moved up.");
                y += 1;
            }
            3 => {
                println!("Moved down.");
                y -= 1;
            }
            // Of course, we need to make sure we cover
            // every possibility in a match statement.
            _ => {
                println!("Invalid input.");
            }
        }
    }

    println!("End position: ({x}, {y})");
}
