#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Box<dyn Frame>>,
}

trait Frame {
    fn roll(&mut self, pins: u16) -> Result<(), Error>;
    fn score(&self, next_frames: Vec<&Box<dyn Frame>>) -> u16;
    fn get_rolls(&self) -> Vec<u16>;
    fn is_complete(&self) -> bool;
}

struct SimpleFrame {
    rolls: Vec<u16>,
}

impl SimpleFrame {
    pub fn new() -> Self {
        SimpleFrame { rolls: Vec::new() }
    }
}

impl Frame for SimpleFrame {
    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.rolls.iter().sum::<u16>() + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.rolls.push(pins);
        Ok(())
    }

    fn score(&self, next_frames: Vec<&Box<dyn Frame>>) -> u16 {
        let next_rolls: Vec<u16> = next_frames
            .iter()
            .take(2)
            .flat_map(|it| it.get_rolls().iter().cloned().collect::<Vec<u16>>())
            .collect();

        match (self.rolls.len(), self.rolls.iter().sum::<u16>()) {
            (1, 10) => self.rolls.iter().sum::<u16>() + next_rolls.iter().take(2).sum::<u16>(),
            (2, 10) => self.rolls.iter().sum::<u16>() + next_rolls.first().unwrap_or(&0),
            _ => self.rolls.iter().sum::<u16>(),
        }
    }

    fn get_rolls(&self) -> Vec<u16> {
        self.rolls.clone()
    }

    fn is_complete(&self) -> bool {
        self.rolls.len() == 2 || self.rolls.iter().sum::<u16>() == 10
    }
}

struct LastFrame {
    rolls: Vec<u16>,
}

impl LastFrame {
    pub fn new() -> Self {
        LastFrame { rolls: Vec::new() }
    }
}

impl Frame for LastFrame {
    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match &self.rolls[..] {
            [10, x] if x == &10 => (),
            [10, x] if x + pins > 10 => return Err(Error::NotEnoughPinsLeft),
            _ => (),
        }
        self.rolls.push(pins);
        Ok(())
    }

    fn score(&self, _next_frames: Vec<&Box<dyn Frame>>) -> u16 {
        self.rolls.iter().sum()
    }

    fn get_rolls(&self) -> Vec<u16> {
        self.rolls.clone()
    }

    fn is_complete(&self) -> bool {
        match &self.rolls[..] {
            [x, y] if x + y < 10 => true,
            [x, y, _] if x + y == 10 => true,
            [10, _, _] => true,
            _ => false,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frames: Vec<Box<dyn Frame>> = Vec::new();
        (0..9).for_each(|_| frames.push(Box::new(SimpleFrame::new())));
        frames.push(Box::new(LastFrame::new()));

        BowlingGame { frames }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.frames
            .iter_mut()
            .find(|it| !it.is_complete())
            .map_or(Err(Error::GameComplete), |it| it.roll(pins))?;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_completed() {
            Some(
                self.frames
                    .iter()
                    .enumerate()
                    .map(|(idx, it)| it.score(self.frames.iter().skip(idx + 1).collect()))
                    .sum::<u16>(),
            )
        } else {
            None
        }
    }

    fn is_completed(&self) -> bool {
        self.frames.iter().filter(|it| it.is_complete()).count() == 10
    }
}
