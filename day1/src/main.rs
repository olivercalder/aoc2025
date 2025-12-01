use std::num::ParseIntError;
use std::str::FromStr;

struct Rotation(i32);

enum ParseRotationError {
    ParsePrefix,
    ParseInt(ParseIntError),
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, num) = s.split_at(1);
        let count: i32 = num.parse().map_err(|e| ParseRotationError::ParseInt(e))?;
        let rot = match prefix {
            "L" => -count,
            "R" => count,
            _ => return Err(ParseRotationError::ParsePrefix),
        };
        Ok(Rotation(rot))
    }
}

struct Position {
    current: i32,
    total_positions: i32,
}

impl Position {
    fn new(start: i32, total_positions: i32) -> Self {
        Position {
            current: start,
            total_positions,
        }
    }

    /// Returns true if the rotation results in a final position of 0.
    fn handle_rotation(&mut self, rot: &Rotation) -> bool {
        self.current = (self.current + rot.0).rem_euclid(self.total_positions);
        self.current == 0
    }

    /// Return the number of times the position lands on zero from the given input.
    fn handle_input(&mut self, r: impl std::io::BufRead) -> usize {
        r.lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| Rotation::from_str(&line))
            .filter_map(Result::ok)
            .filter(|rot| self.handle_rotation(rot))
            .count()
    }
}

fn main() {
    let count = Position::new(50, 100).handle_input(std::io::stdin().lock());
    println!("password: {}", count)
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_example() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let count = super::Position::new(50, 100).handle_input(test_input);
        assert_eq!(count, 3);
    }
}
