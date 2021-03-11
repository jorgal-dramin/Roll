mod dice;

use dice::Dice;
use dice::Rollable;

fn main() {
    match Dice::new(vec![1, 2, 3, 4, 5, 6]) {
        Ok(mut dice) => println!("{}", dice.roll()),
        Err(msg) => println!("{}", msg)
    }
}
