use super::rollable::Rollable;
use rand::Rng;

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
}