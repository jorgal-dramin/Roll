use super::dice::Dice;
use super::rollable::{Rollable, NumericRollable};
use std::convert::TryFrom;

pub struct CompoundDice {
    rollables: Vec<Box<Rollable<u16>>>,
}

impl <'v> CompoundDice {
    pub fn new(rollables: Vec<Box<Rollable<u16>>>) -> Result<CompoundDice, String> {
        if rollables.is_empty() {
            Err("Not able to create compound dice that does not have rollables".to_string())
        } else {
            Ok(CompoundDice {
                rollables
            })
        }
    }
}

impl <'v> Rollable<u16> for CompoundDice {
    fn roll(&mut self) -> &u16 {
        unimplemented!();
    }
}

impl NumericRollable for CompoundDice {

    fn max(&self) -> u16 {
        unimplemented!();
    }

    fn min(&self) -> u16 {
        unimplemented!();
    }
}

impl <'v> TryFrom<&str> for CompoundDice {
    type Error = String;

    fn try_from(formula: &str) -> Result<CompoundDice, Self::Error> {
        unimplemented!();
    }
}

//========== Unit Tests ==========//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_no_rollable_err() {
        let dices: Vec<Box<Rollable<u16>>> = vec![];
        assert!(CompoundDice::new(dices).is_err())
    }

    #[test]
    fn new_with_one_rollable_ok() {
        let dices: Vec<Box<Rollable<u16>>> = vec![Box::new(Dice::new(&(1..6).collect()).unwrap())];
        assert!(CompoundDice::new(dices).is_ok())
    }

    #[test]
    fn new_with_multiple_rollables_ok() {
        let dices: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(7..20).collect()).unwrap()),
        ];
        assert!(CompoundDice::new(dices).is_ok())
    }

    #[test]
    fn new_with_nested_compound_dice_ok() {
        let dices1: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(7..20).collect()).unwrap()),
        ];
        let compound_dice_1_result = CompoundDice::new(dices1);
        assert!(compound_dice_1_result.is_ok());

        let dices2: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(3..7).collect()).unwrap()),
            Box::new(Dice::new(&(1..100).collect()).unwrap()),
            Box::new(Dice::new(&(0..29).collect()).unwrap()),
        ];
        let compound_dice_2_result = CompoundDice::new(dices2);
        assert!(compound_dice_2_result.is_ok());

        let compound_dices: Vec<Box<Rollable<u16>>> = vec![
            Box::new(compound_dice_1_result.unwrap()),
            Box::new(compound_dice_2_result.unwrap()),
        ];
        assert!(CompoundDice::new(compound_dices).is_ok());
    }

    #[test]
    fn roll_with_one_dice() {
        let sub_dices: Vec<Box<Rollable<u16>>> = vec![Box::new(Dice::new(&(1..6).collect()).unwrap())];
        let mut dice = CompoundDice::new(sub_dices).unwrap(); // unwrap is safe here; method is untder test a few lines above
        for _ in (1 .. 200) {
            let result = dice.roll();
            assert!(*result >= 1 && *result <= 6)
        }
    }

    #[test]
    fn roll_with_three_dices() {
        let sub_dices: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(7..20).collect()).unwrap()),
        ];
        let mut dice = CompoundDice::new(sub_dices).unwrap(); // unwrap is safe here; method is untder test a few lines above
        for _ in (1 .. 200) {
            let result = dice.roll();
            assert!(*result >= 9 && *result <= 32)
        }
    }

    #[test]
    fn roll_with_nested_compound_dice() {
        let dices1: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(7..20).collect()).unwrap()),
        ];
        let compound_dice_1_result = CompoundDice::new(dices1);
        assert!(compound_dice_1_result.is_ok());

        let dices2: Vec<Box<Rollable<u16>>> = vec![
            Box::new(Dice::new(&(3..7).collect()).unwrap()),
            Box::new(Dice::new(&(1..100).collect()).unwrap()),
            Box::new(Dice::new(&(0..29).collect()).unwrap()),
        ];
        let compound_dice_2_result = CompoundDice::new(dices2);
        assert!(compound_dice_2_result.is_ok());

        let compound_dices: Vec<Box<Rollable<u16>>> = vec![
            Box::new(compound_dice_1_result.unwrap()),
            Box::new(compound_dice_2_result.unwrap()),
        ];
        let compound_dice_result = CompoundDice::new(compound_dices);
        assert!(compound_dice_result.is_ok());
        let mut outer_compound_dice = compound_dice_result.unwrap();

        for _ in (1 .. 1000) {
            let result = outer_compound_dice.roll();
            assert!(*result >= 13 && *result <= 168)
        }
    }

    #[test]
    fn try_from_string_invalid_formulas_err() {
        assert!(CompoundDice::try_from("formula: &str").is_err());
        assert!(CompoundDice::try_from("W12").is_err());
        assert!(CompoundDice::try_from("d2D3").is_err());
        assert!(CompoundDice::try_from("0d3").is_err());
        assert!(CompoundDice::try_from("10 d3").is_err());
        assert!(CompoundDice::try_from("10d 3").is_err());
        assert!(CompoundDice::try_from("").is_err());
        assert!(CompoundDice::try_from("   ").is_err());
        assert!(CompoundDice::try_from("d4:D3").is_err());
    }

    #[test]
    fn try_from_string_valid_formulas_ok() {
        assert!(CompoundDice::try_from("d3").is_ok());
        assert!(CompoundDice::try_from("7d8").is_ok());
        assert!(CompoundDice::try_from("4D3+3D4").is_ok());
        assert!(CompoundDice::try_from(" 4D3 + 3D4 ").is_ok());
        assert!(CompoundDice::try_from("3d12 + 3").is_ok());
        assert!(CompoundDice::try_from("3d12 - 3").is_ok());
        assert!(CompoundDice::try_from("3d12-3").is_ok());
        assert!(CompoundDice::try_from("d3+2D23-D6").is_ok());
        assert!(CompoundDice::try_from("d3 + 2D23 - D6+2").is_ok());
    }

    #[test]
    fn try_all_from_string_ok() {

        let compound_dice_result = CompoundDice::try_from("d3 + 2D23 - D6 + 12");
        assert!(compound_dice_result.is_ok());
        let mut compound_dice = compound_dice_result.unwrap();

        assert_eq!(8, compound_dice.min());
        assert_eq!(59, compound_dice.max());

        for _ in (1 .. 1000) {
            let result = compound_dice.roll();
            assert!(*result >= 8 && *result <= 59)
        }
    }
}