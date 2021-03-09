use super::rollable::Rollable;
use rand::seq::SliceRandom;
use std::convert::TryFrom;

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

impl <'v> TryFrom<&'v str> for Dice<'v, u16> {
    type Error = String;

    fn try_from(formula: &'v str) -> Result<Dice<'v, u16>, Self::Error> {
        let min_value: u16 = match formula.chars().next() {
            Some('d') => Ok(0),
            Some('D') => Ok(1),
            _ => Err(format!("'{}' is not a valid dice formula: Does not start with 'd' or 'D'", formula.to_string()))
        }?;

        let sides: u16  = formula[1..].parse::<u16>().or(Err(format!("'{}' is not a valid dice formula: Number not parsable", formula)))?;

        let sides_vec: &'v Vec<u16> = (min_value..(sides - 1 + min_value)).collect();
        Dice::new(&sides_vec)
    }
}