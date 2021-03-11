use super::rollable::Rollable;
use rand::Rng;
use std::convert::TryFrom;

pub struct Dice <A: Clone> {
    sides: Vec<A>,
    upside_idx: usize,
}

impl <'v, A: Clone> Dice<A> {
    pub fn new(sides: Vec<A>) -> Result<Dice<A>, String> {
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

impl <'v> TryFrom<&'v str> for Dice<u16> {
    type Error = String;

    fn try_from(formula: &'v str) -> Result<Dice<u16>, Self::Error> {
        let min_value: u16 = match formula.chars().next() {
            Some('d') => Ok(0),
            Some('D') => Ok(1),
            _ => Err(format!("'{}' is not a valid dice formula: Does not start with 'd' or 'D'", formula.to_string()))
        }?;

        let sides: u16  = formula[1..].parse::<u16>().or(Err(format!("'{}' is not a valid dice formula: Number not parsable", formula)))?;

        let sides_vec = (min_value..(sides - 1 + min_value)).collect();
        Dice::new(sides_vec)
    }
}