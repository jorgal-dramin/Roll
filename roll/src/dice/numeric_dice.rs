use super::rollable::{Rollable, NumericRollable};
use rand::Rng;
use std::convert::TryFrom;


pub struct NumericDice {
    sides: Vec<u16>,
    upside_idx: usize,
}

impl <'v> NumericDice {
    pub fn new(sides: &Vec<u16>) -> Result<NumericDice, String> {
        if sides.is_empty() {
            Err("New numeric dice has no sides".to_string())
        } else {
            let sides_copy = sides.clone();
            Ok(NumericDice {
                upside_idx : 0,
                sides: sides_copy,
            })
        }
    }

    pub fn upside(&'v self) -> &'v u16 {
        &self.sides[self.upside_idx]
    }
}

impl <'v> Rollable<u16> for NumericDice {
    fn roll(&mut self) -> &u16 {
        self.upside_idx = rand::thread_rng().gen_range(0..self.sides.len());
        &self.sides[self.upside_idx]
    }
}

impl NumericRollable for NumericDice {

    fn max(&self) -> u16 {
        unimplemented!();
    }

    fn min(&self) -> u16 {
        unimplemented!();
    }
}

impl TryFrom<&str> for NumericDice {
    type Error = String;

    fn try_from(formula: &str) -> Result<NumericDice, Self::Error> {
        let min_value: u16 = match formula.chars().next() {
            Some('d') => Ok(0),
            Some('D') => Ok(1),
            _ => Err(format!("'{}' is not a valid dice formula: Does not start with 'd' or 'D'", formula.to_string()))
        }?;

        let sides: u16  = formula[1..].parse::<u16>().or(Err(format!("'{}' is not a valid dice formula: Number not parsable", formula)))?;
        if sides == 0 {
            Err(format!("'{}' is not a valid dice formula: Number must be greater than 0", formula))
        } else {
            let sides_vec = (min_value..(sides + min_value)).collect(); // upper bound is exclusive
            NumericDice::new(&sides_vec)
        }
    }
}

//========== Unit Tests ==========//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_string_dice_ok() {
        let dice_result = NumericDice::new(&vec![1, 1, 2, 3, 5, 8, 13]);
        assert!(dice_result.is_ok());
        let dice = dice_result.unwrap();
        assert_eq!(&1, dice.upside());
    }

    #[test]
    fn new_dice_err() {
        let dice_result = NumericDice::new(&Vec::<u16>::new());
        assert!(dice_result.is_err());
    }

    #[test]
    fn roll_dice_result_ok() {
        let sides = vec![1, 1, 2, 3, 5, 8, 13];
        let dice_result = NumericDice::new(&sides);
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in (0..200) {
            let result = dice.roll();
            sides.contains(&result);
        }
    }

    #[test]
    fn min_max_1_ok() {
        let sides = vec![30, 45, 12, 4, 56, 67];
        let dice_result = NumericDice::new(&sides);
        assert!(dice_result.is_ok());
        let dice = dice_result.unwrap();
        assert_eq!(4, dice.min());
        assert_eq!(67, dice.max());
    }

    #[test]
    fn min_max_2_ok() {
        let sides = vec![8, 6, 2, 4, 0];
        let dice_result = NumericDice::new(&sides);
        assert!(dice_result.is_ok());
        let dice = dice_result.unwrap();
        assert_eq!(0, dice.min());
        assert_eq!(8, dice.max());
    }

    #[test]
    fn roll_dice_from_str_err_invalid_formulas() {
        let dice_result = NumericDice::try_from("invalid");
        assert!(dice_result.is_err());
        let dice_result = NumericDice::try_from(" d2");
        assert!(dice_result.is_err());
        let dice_result = NumericDice::try_from("d-32");
        assert!(dice_result.is_err());
        let dice_result = NumericDice::try_from("W32");
        assert!(dice_result.is_err());
        let dice_result = NumericDice::try_from("D0");
        assert!(dice_result.is_err());
    }

    #[test]
    fn roll_dice_from_str_lower_case_d_ok() {
        let dice_result = NumericDice::try_from("d8");
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in 0..250 {
            let result = dice.roll();
            assert!(*result < 8); // check for >= 0 is unneccessary; a u16 is al... ah, you got it :)
        }
    }

    #[test]
    fn roll_dice_from_str_upper_case_d_ok() {
        let dice_result = NumericDice::try_from("D10");
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in 0..250 {
            let result = dice.roll();
            assert!(*result <= 10 && *result > 0);
        }
    }
}