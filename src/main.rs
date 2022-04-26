
use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

// enum for Pikmin types
// Pikmin colours represented by numbers - Purple is 0 because we start at red and want to wrap around using modular arithmetic
#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator, IntEnum)]
#[repr(usize)]
pub enum Pikmin {
    Red = 1,
    Yellow = 2,
    Blue = 3,
    White = 4,
    Purple = 0,
}

impl Pikmin {
    pub fn next(num_plucked: usize) -> Self {
        Pikmin::from_int((num_plucked+1).rem_euclid(5)).unwrap() // next() - when called, unwrap the next Pikmin calculating the modulus of total Pikmin plucked 
    }
}

// Structure for the queue - this is an abstraction of Olimar's lineup in Smash
#[derive(Debug, PartialEq, Clone)]
pub struct PikminQueue {
    pluck_counter: usize, 
    lineup: Vec::<Pikmin>,
}

// Programmer-defined Error type to produce a more helpful error message (hopefully)
pub enum PluckError {
    LineupFull,
}

impl PikminQueue {
    pub fn new() -> Self {
        Self {
            pluck_counter: 0,
            lineup: Vec::<Pikmin>::new(),
        }
    }

    pub fn pluck(&mut self) -> Result<(), PluckError> {
        if self.lineup.len() == 3 {
            Err(PluckError::LineupFull)
        } else {
        self.lineup.push(Pikmin::next(self.pluck_counter));
        self.pluck_counter+=1;
        Ok(())
        }
    }
}

impl Iterator for PikminQueue {
    type Item = Pikmin;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Pikmin::Purple)
    }
}

fn main() {
    let mut olimar= PikminQueue::new();
    for i in 0..2 {
        olimar.pluck();
    }
    
}

