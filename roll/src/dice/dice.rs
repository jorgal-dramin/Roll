use super::rollable::Rollable;
use rand::seq::SliceRandom;

pub struct Dice <'v, A> {
    sides: &'v Vec<A>,
    upside: &'v A
}

impl <'v, A> Dice<'v, A> {
    pub fn new(sides: &'v Vec<A>) -> Result<Dice<A>, String> {
        if sides.is_empty() {
            Err("New dice has no sides".to_string())
        } else {
            Ok(Dice {
                sides,
                upside : sides.get(0).unwrap(), // unwrap is safe, length has been checked
            })
        }
    }

    pub fn upside(&'v self) -> &'v A {
        self.upside
    }
}

impl <'v, A> Rollable<'v, A> for Dice<'v, A> {
    fn roll(&'v mut self) -> &A {
        self.upside = self.sides.choose(&mut rand::thread_rng()).unwrap();
        self.upside
    }
}