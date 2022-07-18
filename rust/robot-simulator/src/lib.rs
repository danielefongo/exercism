// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot {
            position: (x, y),
            direction,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot {
            direction,
            position: self.position,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot {
            direction,
            position: self.position,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let position = match self.direction {
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::West => (self.position.0 - 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0 + 1, self.position.1),
        };
        Robot {
            position,
            direction: self.direction,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
