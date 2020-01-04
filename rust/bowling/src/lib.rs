#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    first_throw: u16,
    second_throw: Option<u16>
}

pub struct BowlingGame {
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
	    frames: Vec::new(),
	}
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
	if self.frames.last().second_throw.is_some() || self.frames.last(). {
	}
    }

    pub fn score(&self) -> Option<u16> {
	if self.frames.len() == 10 {
	    unimplemented!()
	} else {
	    None
	}
    }
}
