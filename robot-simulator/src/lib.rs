// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<u8> for Direction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Direction::North as u8 => Ok(Direction::North),
            v if v == Direction::East as u8 => Ok(Direction::East),
            v if v == Direction::South as u8 => Ok(Direction::South),
            v if v == Direction::West as u8 => Ok(Direction::West),
            v => Err(format!("Unknown value: {}", v)),
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    fn turn(self, dir: u8) -> Self {
        let new_dir = (self.direction as u8).wrapping_add(dir) % 4;
        Self {
            x: self.x,
            y: self.y,
            direction: Direction::try_from(new_dir).unwrap(),
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        self.turn(1)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        self.turn(u8::MAX)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self {
                x: self.x,
                y: self.y + 1,
                direction: self.direction,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
                direction: self.direction,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - 1,
                direction: self.direction,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
                direction: self.direction,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut result = self;
        for c in instructions.chars() {
            result = match c {
                'R' => result.turn_right(),
                'L' => result.turn_left(),
                'A' => result.advance(),
                _ => panic!("Unexpected movement: {}", c),
            }
        }
        result
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
