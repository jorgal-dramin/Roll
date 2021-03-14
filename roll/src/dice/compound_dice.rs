use super::dice::Dice;
use super::rollable::Rollable;
use std::convert::TryFrom;

pub struct CompoundDice<'v> {
    rollables: Vec<Box<Rollable<'v, u16>>>,
}

impl <'v> CompoundDice<'v> {
    pub fn new(rollables: Vec<Box<Rollable<'v, u16>>>) -> Result<CompoundDice, String> {
        if rollables.is_empty() {
            Err("Not able to create compound dice that does not have rollables".to_string())
        } else {
            Ok(CompoundDice {
                rollables
            })
        }
    }
}

impl <'v> Rollable<'v, u16> for CompoundDice<'v> {
    fn roll(&mut self) -> &u16 {
        unimplemented!();
    }
}

impl <'v> TryFrom<&str> for CompoundDice<'v> {
    type Error = String;

    fn try_from(formula: &str) -> Result<CompoundDice<'v>, Self::Error> {
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
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(1..6).collect()).unwrap()),
            Box::new(Dice::new(&(7..20).collect()).unwrap()),
        ];
        let compound_dice_2_result = CompoundDice::new(dices2);
        assert!(compound_dice_2_result.is_ok());

        let compound_dices: Vec<Box<Rollable<u16>>> = vec![
            Box::new(compound_dice_1_result.unwrap()),
            Box::new(compound_dice_2_result.unwrap()),
        ];
        assert!(CompoundDice::new(compound_dices).is_ok());
    }
}