#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let mut temp = self.rolls.clone();
        temp.push(pins);
        if !Self::validate_rolls(&temp) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut total = 0;
        let mut idx = 0;

        for frame in 1..=10 {
            let first = self.rolls[idx];
            if first == 10 {
                // Strike
                total += 10 + self.rolls[idx + 1] + self.rolls[idx + 2];
                idx += if frame < 10 { 1 } else { 3 };
            } else {
                let second = self.rolls[idx + 1];
                let frame_pins = first + second;
                if frame_pins == 10 {
                    // Spare
                    total += 10 + self.rolls[idx + 2];
                } else {
                    // Open frame
                    total += frame_pins;
                }
                idx += if frame < 10 {
                    2
                } else if frame_pins == 10 {
                    3
                } else {
                    2
                };
            }
        }

        Some(total)
    }

    fn is_complete(&self) -> bool {
        let mut idx = 0;
        for frame in 1..=10 {
            if idx >= self.rolls.len() {
                return false;
            }
            let first = self.rolls[idx];
            if frame < 10 {
                if first == 10 {
                    idx += 1;
                } else {
                    if idx + 1 >= self.rolls.len() {
                        return false;
                    }
                    idx += 2;
                }
            } else {
                return if first == 10 {
                    self.rolls.len() >= idx + 3
                } else {
                    if idx + 1 >= self.rolls.len() {
                        return false;
                    }
                    let second = self.rolls[idx + 1];
                    if first + second == 10 {
                        // Spare: one fill
                        self.rolls.len() >= idx + 3
                    } else {
                        // Open: only two rolls
                        self.rolls.len() >= idx + 2
                    }
                }
            }
        }
        false
    }

    fn validate_rolls(rolls: &[u16]) -> bool {
        let mut idx = 0;
        for frame in 1..=10 {
            if idx >= rolls.len() {
                break;
            }
            let first = rolls[idx];
            if first > 10 {
                return false;
            }
            if frame < 10 {
                if first == 10 {
                    idx += 1;
                } else {
                    if idx + 1 >= rolls.len() {
                        break;
                    }
                    let second = rolls[idx + 1];
                    if second > 10 || first + second > 10 {
                        return false;
                    }
                    idx += 2;
                }
            } else {
                if first == 10 {
                    if idx + 1 < rolls.len() {
                        let second = rolls[idx + 1];
                        if second > 10 {
                            return false;
                        }
                    }
                    if idx + 2 < rolls.len() {
                        let second = rolls[idx + 1];
                        let third = rolls[idx + 2];
                        if second != 10 && second + third > 10 {
                            return false;
                        }
                    }
                } else {
                    if idx + 1 < rolls.len() {
                        let second = rolls[idx + 1];
                        if second > 10 || first + second > 10 {
                            return false;
                        }
                        if first + second == 10 {
                            if idx + 2 < rolls.len() && rolls[idx + 2] > 10 {
                                return false;
                            }
                        } else if rolls.len() > idx + 2 {
                            return false;
                        }
                    }
                }
                break;
            }
        }
        true
    }
}
