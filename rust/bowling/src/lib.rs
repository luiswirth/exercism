#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    first_throw: u16,
    second_throw: Option<u16>,
    score: Option<u16> // compute in reverse order of Frames
	// then add up for score() function
}

impl Frame {
    fn is_strike() -> bool {
	first_throw == 10
    }

    fn is_spare() -> bool {
	second_throw.is_some() && first_throw + second_throw.unwrap() == 10
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }
        let second_throw = self.frames.last().is_some()
            && self.frames.last().unwrap().second_throw.is_none()
            && self.frames.last().unwrap().first_throw != 10;
        if pins > 10 || (second_throw && self.frames.last().unwrap().first_throw + pins > 10) {
            return Err(Error::NotEnoughPinsLeft);
        }

        if second_throw {
            self.frames.last_mut().unwrap().second_throw = Some(pins);
        } else {
            self.frames.push(Frame {
                first_throw: pins,
                second_throw: None,
            });
        }
        Ok(())
    }

    fn frame_score(&self, i: usize) -> u16 {
	let score = self.frames[i].first_throw + self.frames[i].second_throw.unwrap_or(0);
	if self.frames[i].is_strike() {
	    score += self.frames.get(i+1).unwrap_or(0);
	    score += self.frames.get(i+2).unwrap_or(0);
	} else if self.frames[i].is_spare() {

	}
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() != 10 {
            return None;
        }
        let mut score = 0;
	for frame in &self.frames {
	    score += frame.first_throw + frame.second_throw.unwrap_or(0);
	}

	Some(score)
    }
}
