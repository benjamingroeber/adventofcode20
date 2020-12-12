use std::error::Error;
use std::num::ParseIntError;
use std::ops::Neg;
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions: Vec<Instruction> = helpers::parse_lines_file("./assets/days/day12.txt")?;

    // Part 1
    // Figure out where the navigation instructions lead. What is the Manhattan distance between
    // that location and the ship's starting position?
    let mut ferry = Ferry::new();
    for i in &instructions {
        ferry.act(*i)
    }
    let (x, y) = ferry.relative_position();
    println!(
        "Part1: Relative position {}, {}, Manhattan Distance: {}",
        x,
        y,
        x.abs() + y.abs()
    );

    // Part 2
    // Figure out where the navigation instructions actually lead. What is the Manhattan distance
    // between that location and the ship's starting position?
    let mut ferry = Ferry::new();
    for i in &instructions {
        ferry.act_with_waypoint(*i)
    }
    let (x, y) = ferry.relative_position();
    println!(
        "Part2: Relative position {}, {}, Manhattan Distance: {}",
        x,
        y,
        x.abs() + y.abs()
    );
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Ferry {
    x: Unit,
    y: Unit,
    direction: Direction,
    waypoint: Waypoint,
}

impl Ferry {
    pub fn new() -> Self {
        Ferry {
            x: 0,
            y: 0,
            // The ship starts by facing east.
            direction: Direction::East,
            waypoint: Waypoint::default(),
        }
    }

    pub fn relative_position(&self) -> (Unit, Unit) {
        (self.x, self.y)
    }

    // Part 1
    pub fn act(&mut self, instruction: Instruction) {
        match instruction.action {
            // Action N means to move north by the given value.
            // Action S means to move south by the given value.
            // Action E means to move east by the given value.
            // Action W means to move west by the given value.
            Action::Direction(d) => self.travel(d, instruction.value),
            // Action L means to turn left the given number of degrees.
            // Action R means to turn right the given number of degrees.
            Action::Turn(t) => self.turn(t, instruction.value),
            // Action F means to move forward by the given value in the direction the ship is currently facing.
            Action::Forward => self.travel(self.direction, instruction.value),
        }
    }

    // Only multiples of 90 degrees are allowed
    fn turn(&mut self, turn_to: Turn, degrees: Unit) {
        assert_eq!(degrees % 90, 0);
        let mut times = degrees / 90;
        while times > 0 {
            self.direction = turn(self.direction, turn_to);
            times -= 1
        }
    }

    // Move `distance` units in the given `direction`
    fn travel(&mut self, direction: Direction, distance: Unit) {
        match direction {
            Direction::North => self.y += distance,
            Direction::East => self.x += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x -= distance,
        }
    }

    // Part 2
    pub fn act_with_waypoint(&mut self, instruction: Instruction) {
        match instruction.action {
            Action::Direction(d) => self.move_waypoint(d, instruction.value),
            Action::Turn(t) => self.rotate_waypoint(t, instruction.value),
            Action::Forward => self.travel_to_waypoint(instruction.value),
        }
    }

    // Action N means to move the waypoint north by the given value.
    // Action S means to move the waypoint south by the given value.
    // Action E means to move the waypoint east by the given value.
    // Action W means to move the waypoint west by the given value
    fn move_waypoint(&mut self, direction: Direction, distance: Unit) {
        match direction {
            Direction::North => self.waypoint.y += distance,
            Direction::East => self.waypoint.x += distance,
            Direction::South => self.waypoint.y -= distance,
            Direction::West => self.waypoint.x -= distance,
        }
    }

    // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    // Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    fn rotate_waypoint(&mut self, turn_to: Turn, degrees: Unit) {
        assert_eq!(degrees % 90, 0);
        let waypoint = self.waypoint;
        // normalize degrees to right turn and one circle
        let degrees = match turn_to {
            Turn::Left => 360 - degrees % 360,
            Turn::Right => degrees % 360,
        };

        let (x, y) = match degrees {
            0 => (waypoint.x, waypoint.y),
            // clockwise vector rotation
            90 => (waypoint.y, waypoint.x.neg()),
            // inversion
            180 => (waypoint.x.neg(), waypoint.y.neg()),
            // counterclockwise vector rotation
            270 => (waypoint.y.neg(), waypoint.x),
            _ => unreachable!(),
        };
        self.waypoint.x = x;
        self.waypoint.y = y;
    }

    // Action F means to move forward to the waypoint a number of times equal to the given value.
    fn travel_to_waypoint(&mut self, times: Unit) {
        self.y += self.waypoint.y * times;
        self.x += self.waypoint.x * times;
    }
}

fn turn(origin: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::Left => match origin {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        },
        Turn::Right => match origin {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
    }
}

#[derive(Copy, Clone, Debug)]
struct Waypoint {
    x: Unit,
    y: Unit,
}

impl Default for Waypoint {
    fn default() -> Self {
        // The waypoint starts 10 units east and 1 unit north relative to the ship.
        Waypoint { x: 10, y: 1 }
    }
}

type Unit = i128;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    action: Action,
    value: Unit,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
    Direction(Direction),
    Turn(Turn),
    Forward,
}

impl FromStr for Instruction {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(action) = s.chars().next() {
            let value = s[action.len_utf8()..].parse()?;
            let action = match action {
                'N' => Ok(Action::Direction(Direction::North)),
                'E' => Ok(Action::Direction(Direction::East)),
                'S' => Ok(Action::Direction(Direction::South)),
                'W' => Ok(Action::Direction(Direction::West)),
                'L' => Ok(Action::Turn(Turn::Left)),
                'R' => Ok(Action::Turn(Turn::Right)),
                'F' => Ok(Action::Forward),
                _ => Err(InstructionError::Unknown(action)),
            }?;
            Ok(Instruction { action, value })
        } else {
            Err(InstructionError::Empty)
        }
    }
}

#[derive(Debug, Clone, Error)]
enum InstructionError {
    #[error("could not parse instruction")]
    ParseValue(#[from] ParseIntError),
    #[error("unrecognized instruction")]
    Unknown(char),
    #[error("empty instruction")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART1_EXAMPLE: &str = "F10
N3
F7
R90
F11";
    #[test]
    fn test_example_part1() {
        let instructions: Result<Vec<Instruction>, _> =
            PART1_EXAMPLE.lines().map(|l| l.parse()).collect();
        let mut ship = Ferry::new();

        for i in &instructions.unwrap() {
            ship.act(*i)
        }

        // At the end of these instructions, the ship's Manhattan distance from its starting
        // position is 17 + 8 = 25.
        assert_eq!(ship.x.abs() + ship.y.abs(), 25)
    }

    #[test]
    fn test_exmaple_part2() {
        let instructions: Result<Vec<Instruction>, _> =
            PART1_EXAMPLE.lines().map(|l| l.parse()).collect();
        let mut ship = Ferry::new();

        for i in &instructions.unwrap() {
            ship.act_with_waypoint(*i)
        }

        //After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
        // position is 17 + 8 = 25.
        assert_eq!(ship.x.abs() + ship.y.abs(), 286)
    }
}
