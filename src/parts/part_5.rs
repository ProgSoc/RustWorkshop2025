use input_macro::input;

// Import traits from the standard library (std),
// which we can implement for types that we define in our program.
// Is there a way to make `Direction` easier to work with?

// TODO: Use a macro here so that `Direction`
//       no longer gets moved out of variables all the time.
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_string(direction: &str) -> Option<Self> {
        match direction {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

// Let's implement the `fmt::Display` trait for `Direction`
// so that we can put these objects directly into a `println!` macro.

// Is there a way to make `Action` easier to work with?

// TODO: Do to the `Action` enum what you did to the `Direction` enum.
enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

impl Action {
    fn from_string(action: &str) -> Result<Self, String> {
        match action {
            "splash" => Ok(Self::Splash),
            "left" | "right" | "up" | "down" => {
                let direction = Direction::from_string(action).unwrap();
                Ok(Self::Movement(direction))
            }
            _ if action.starts_with("deposit") => {
                let sequence: Vec<&str> = action.split_whitespace().collect();
                if sequence.len() == 2 {
                    if let Ok(int_value) = sequence[1].parse::<u64>() {
                        Ok(Self::Deposit(int_value))
                    } else {
                        Err("Invalid integer.".to_string())
                    }
                } else {
                    Err("'deposit' must be followed by a single integer.".to_string())
                }
            }
            _ => Err("Invalid action.".to_string()),
        }
    }
}

// Let's also implement `fmt::Display` for `Action`,
// and we can rely on the definition for `fmt::Display` for `Direction` as well.

// TODO: Implement `fmt::Display` for `Action`.

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new_position() -> Self {
        Self { x: 0, y: 0 }
    }
}

// TODO: Implement the `Default` trait for `Position`.


// When we just print a `Position` out via the `Display` trait,
// we won't need to implement getters for `Position` in this program.
// We just remove them here to make the compiler a little happier.

// TODO: Implement the `fmt::Display` trait for `Position`.


// Let's also make it possible to do something like
// `Position` + `Direction`. What trait would we implement
// to enable this functionality?

// TODO: Implement a trait that allows adding a `Direction` to a `Position`.

pub fn main() {

    let mut money = 0;

    // TODO: Uncomment the below once you have implemented `Default` for `Position`.
    // let mut pos: Position = Default::default();

    // TODO: Uncomment the below once you have
    //       implemented `fmt::Display` for `Position`.
    // println!("Start position: {}", pos);

    for _ in 0..6 {
        let direction_string = input!("Input action: ").to_lowercase();

        match Action::from_string(&direction_string) {
            Ok(action) => {
                // We can put the printing functionality just in one line
                // outside the `match` statement.
                // Though of course, this relies on the `fmt::Display` trait
                // being implemented for `Action`.

                // TODO: Uncomment the below once you have
                //       implemented `fmt::Display` for `Action`.
                // println!("You {action}.");

                match action {
                    Action::Splash => {}
                    Action::Movement(d) => {
                        // This will need some trait to overload the operator.

                        // TODO: Uncomment the below when you have implemented
                        //       the appropriate trait (hint: it's impl'd for `Position`).
                        // pos = pos + d;
                    }
                    Action::Deposit(value) => {
                        money += value;
                    }
                }
            }
            Err(error_msg) => {
                println!("{error_msg}");
            }
        }
    }

    // Implementing the `Display` trait also gives you
    // access to the `to_string` method.

    // TODO: Uncomment the below once you have
    //       implemented `fmt::Display` for `Position`.
    // println!("End position: {}", pos.to_string());

    println!("Final money: ${money}");
}
