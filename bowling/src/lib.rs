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

    fn is_openframe(&self) -> bool {
        self.score() != PINS_AND_FRAMES_LIMIT
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

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > PINS_AND_FRAMES_LIMIT {
            return Err(Error::NotEnoughPinsLeft);
        }
        let len = self.frames.len() as u16;
        if len >= PINS_AND_FRAMES_LIMIT {
            if len == PINS_AND_FRAMES_LIMIT {
                let last = self.frames.last_mut().unwrap();
                return match last.second {
                    Some(_) if last.is_openframe() => Err(Error::GameComplete),
                    Some(_) => Ok(self.frames.push(Frame{first: Some(pins), second: None})),
                    None if last.is_strike() => Ok(self.frames.push(Frame{first: Some(pins), second: None})),
                    None if last.first.unwrap() + pins > PINS_AND_FRAMES_LIMIT => Err(Error::NotEnoughPinsLeft),
                    None => Ok(last.second = Some(pins))
                };
            } else {
                // Bonus (11 Frames)
                let bonuses = match self.frames.get((PINS_AND_FRAMES_LIMIT-1) as usize).unwrap() {
                    f if f.is_strike() => 2,
                    f if f.is_spare() => 1,
                    _ => 0
                };
                let last = self.frames.last_mut().unwrap();
                return match (bonuses, last) {
                    (0, _) => Err(Error::GameComplete),
                    (1, l) if l.first.is_some() => Err(Error::GameComplete),
                    (1, l) => Ok(l.first = Some(pins)),
                    (2, l) if l.second.is_some() => Err(Error::GameComplete),
                    (2, l) if !l.is_strike() && l.first.unwrap() + pins > PINS_AND_FRAMES_LIMIT => Err(Error::NotEnoughPinsLeft),
                    (2, l) => Ok(l.second = Some(pins)),
                    (_, _) => Err(Error::GameComplete)
                };
            }
        } else {
            // Frame
            let frame = self.frames.last_mut();
            return match frame {
                Some(f) if !f.is_strike() && f.second.is_none() => {
                    if f.first.unwrap_or_default() + pins > PINS_AND_FRAMES_LIMIT {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        Ok(f.second = Some(pins))
                    }
                },
                _ => Ok(self.frames.push(Frame{first: Some(pins), second: None}))
            };
        }
    }

    pub fn score(&self) -> Option<u16> {
        let len = self.frames.len() as u16;
        if len < PINS_AND_FRAMES_LIMIT {
            return None;
        }
        let tens = self.frames.get((PINS_AND_FRAMES_LIMIT-1) as usize).unwrap();
        let bonuses = match tens {
            f if f.is_strike() => 2,
            f if f.is_spare() => 1,
            _ => 0
        };
        if bonuses > 0 {
            if len == PINS_AND_FRAMES_LIMIT {
                return None;
            }
            let last = self.frames.last().unwrap(); // Bonus Frame
            let throws = match (last.first.is_some(), last.second.is_some()) {
                (true, true) => 2,
                (false, false) => 0,
                (_, _) => 1
            };
            if bonuses != throws {
                return None;
            }
        }
        Some(self.frames.iter()
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
