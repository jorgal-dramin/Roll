use super::rollable::Rollable;
use rand::Rng;
use std::convert::TryFrom;

pub struct Dice <A: Clone> {
    sides: Vec<A>,
    upside_idx: usize,
}

impl <'v, A: Clone> Dice<A> {
    pub fn new(sides: &Vec<A>) -> Result<Dice<A>, String> {
        if sides.is_empty() {
            Err("New dice has no sides".to_string())
        } else {
            let sides_copy = sides.clone();
            Ok(Dice {
                upside_idx : 0,
                sides: sides_copy,
            })
        }
    }

    pub fn upside(&'v self) -> &'v A {
        &self.sides[self.upside_idx]
    }
}

impl <'v, A: Clone> Rollable<'v, A> for Dice<A> {
    fn roll(&'v mut self) -> &A {
        self.upside_idx = rand::thread_rng().gen_range(0..self.sides.len());
        &self.sides[self.upside_idx]
    }
}

impl TryFrom<&str> for Dice<u16> {
    type Error = String;

    fn try_from(formula: &str) -> Result<Dice<u16>, Self::Error> {
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
            Dice::new(&sides_vec)
        }
    }
}

//========== Unit Tests ==========//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_string_dice_ok() {
        let dice_result = Dice::new(&vec!["One", "2", "drei", "IV", "1001", "SIX"]);
        assert!(dice_result.is_ok());
        let dice = dice_result.unwrap();
        assert_eq!(&"One", dice.upside());
    }

    #[test]
    fn new_dice_err() {
        let dice_result = Dice::new(&Vec::<u8>::new());
        assert!(dice_result.is_err());
    }

    #[test]
    fn roll_dice_result_ok() {
        let sides = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        let dice_result = Dice::new(&sides);
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in (0..200) {
            let result = dice.roll();
            sides.contains(&result);
        }
    }

    #[test]
    fn roll_dice_from_str_err_invalid_formulas() {
        let dice_result = Dice::try_from("invalid");
        assert!(dice_result.is_err());
        let dice_result = Dice::try_from(" d2");
        assert!(dice_result.is_err());
        let dice_result = Dice::try_from("d-32");
        assert!(dice_result.is_err());
        let dice_result = Dice::try_from("W32");
        assert!(dice_result.is_err());
        let dice_result = Dice::try_from("D0");
        assert!(dice_result.is_err());
    }

    #[test]
    fn roll_dice_from_str_lower_case_d_ok() {
        let dice_result = Dice::try_from("d8");
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in 0..250 {
            let result = dice.roll();
            assert!(*result < 8); // check for >= 0 is unneccessary; a u16 is al... ah, you got it :)
        }
    }

    #[test]
    fn roll_dice_from_str_upper_case_d_ok() {
        let dice_result = Dice::try_from("D10");
        assert!(dice_result.is_ok());
        let mut dice = dice_result.unwrap();
        for _ in 0..250 {
            let result = dice.roll();
            assert!(*result <= 10 && *result > 0);
        }
    }
}