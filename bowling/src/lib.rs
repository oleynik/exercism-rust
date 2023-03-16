#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const PINS_AND_FRAMES_LIMIT: u16 = 10;
struct Frame {
    first: Option<u16>,
    second: Option<u16>
}

impl Frame {
    fn score(&self) -> u16 {
        self.first.unwrap_or_default() + self.second.unwrap_or_default()
    }

    fn is_strike(&self) -> bool {
        self.first.unwrap_or_default() == PINS_AND_FRAMES_LIMIT
    }

    fn is_spare(&self) -> bool {
        !self.is_strike() && self.score() == PINS_AND_FRAMES_LIMIT
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![]
        }
    }

    fn number_of_bonuses(tens_frame: Option<&Frame>) -> u16 {
        match tens_frame {
            Some(f) if f.is_strike() => 2,
            Some(f) if f.is_spare() => 1,
            _ => 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > PINS_AND_FRAMES_LIMIT {
            return Err(Error::NotEnoughPinsLeft);
        }
        let len = self.frames.len() as u16;
        let tens = self.frames.get((PINS_AND_FRAMES_LIMIT-1) as usize);
        let bonus_numbers = BowlingGame::number_of_bonuses(tens);
        match (len, bonus_numbers, &mut self.frames) {
            (11, 2, vec) if vec.last().unwrap().second.is_some() => Err(Error::GameComplete),
            (11, 2, vec) if !vec.last().unwrap().is_strike() && vec.last().unwrap().first.unwrap_or_default() + pins > PINS_AND_FRAMES_LIMIT => Err(Error::NotEnoughPinsLeft),
            (11, 1, _) => Err(Error::GameComplete),
            (10, 0, vec) if vec.last().unwrap().second.is_some() => Err(Error::GameComplete),
            (0, _, _) => Ok(self.frames.push(Frame{first: Some(pins), second: None})),
            (_, _, vec) if vec.last().unwrap().is_strike() => Ok(self.frames.push(Frame{first: Some(pins), second: None})),
            (_, _, vec) if vec.last().unwrap().second.is_some() => Ok(self.frames.push(Frame{first: Some(pins), second: None})),
            (_, _, vec) if vec.last().unwrap().first.unwrap_or_default() + pins > PINS_AND_FRAMES_LIMIT => Err(Error::NotEnoughPinsLeft),
            (_, _, vec) => Ok(vec.last_mut().unwrap().second = Some(pins))
        }
    }

    pub fn score(&self) -> Option<u16> {
        let len = self.frames.len() as u16;
        let tens = self.frames.get((PINS_AND_FRAMES_LIMIT-1) as usize);
        let bonus_numbers = BowlingGame::number_of_bonuses(tens);
        match (len, bonus_numbers) {
            (11, 2) if self.frames.last().unwrap().second.is_none() => None,
            (10, 1 | 2) => None,
            (10, _) if self.frames.last().unwrap().second.is_none() => None,
            (0..=9, _) => None,
            (_, _) => Some(self.frames.iter()
                .chain((0..2).map(|_|&Frame {first: Some(0), second: Some(0)}).into_iter())
                .collect::<Vec<&Frame>>()
                .windows(3)
                .take(PINS_AND_FRAMES_LIMIT as usize)
                .map(|w| {
                    match w {
                        w if w[0].is_strike() && w[1].is_strike() => w[0].score() + w[1].score() + w[2].first.unwrap(),
                        w if w[0].is_strike() => w[0].score() + w[1].score(),
                        w if w[0].is_spare() => w[0].score() + w[1].first.unwrap(),
                        w => w[0].score()
                    }
                })
                .sum())
        }
    }
}
