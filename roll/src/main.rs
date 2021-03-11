mod dice;

use dice::Dice;
use dice::Rollable;
use std::env;
use std::convert::TryFrom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dice_str = &args[args.len() - 1];

    

    match Dice::<u16>::try_from(dice_str.as_str()) {
        Ok(mut dice) => println!("{}", dice.roll()),
        Err(msg) => println!("{}", msg)
    }
/*
    match Dice::new(&vec![1, 2, 3, 4, 5, 6]) {
        Ok(mut dice) => println!("{}", dice.roll()),
        Err(msg) => println!("{}", msg)
    }
    */
}
