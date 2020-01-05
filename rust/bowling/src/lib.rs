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
    fn is_strike(&self) -> bool {
	self.first_throw == 10
    }

    fn is_spare(&self) -> bool {
	self.second_throw.is_some() && self.first_throw + self.second_throw.unwrap() == 10
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
		score: None,
            });
        }
        Ok(())
    }

    fn compute_frame_scores(&mut self) {
	// reverse computation
	for i in (0..self.frames.len()).rev() {
	    let mut score = self.frames[i].first_throw + self.frames[i].second_throw.unwrap_or(0);
	    if self.frames[i].is_spare() {
		score += self.frames.get(i + 1).and_then(|f| f.score).unwrap_or(0);
	    }
	    if self.frames[i].is_strike() {
		score += self.frames.get(i + 1).and_then(|f| f.score).unwrap_or(0);
		score += self.frames.get(i + 2).and_then(|f| f.score).unwrap_or(0);
	    }
	    self.frames[i].score = Some(score);
	}
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() >= 10 {
	    Some(self.frames.iter().map(|f| f.score.unwrap()).sum())
        } else {
            None
	}
	
    }
}
