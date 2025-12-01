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

    /// Returns counts for the number of times the rotation results in a final position of 0 (at
    /// most once) and the number of times the rotation passed through zero (including ending
    /// there).
    fn handle_rotation(&mut self, rot: &Rotation) -> (usize, usize) {
        let raw_sum = self.current + rot.0;
        let mut passthroughs: usize = (raw_sum / self.total_positions).abs().try_into().unwrap();
        if self.current > 0 && raw_sum <= 0 {
            passthroughs += 1;
        }
        self.current = raw_sum.rem_euclid(self.total_positions);
        let exact = if self.current == 0 { 1 } else { 0 };
        (exact, passthroughs)
    }

    /// Return the number of times the position lands on zero from the given input.
    fn handle_input(&mut self, r: impl std::io::BufRead) -> (usize, usize) {
        r.lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| Rotation::from_str(&line))
            .filter_map(Result::ok)
            .fold((0, 0), |acc, rot| {
                let (exact, passthrough) = self.handle_rotation(&rot);
                (acc.0 + exact, acc.1 + passthrough)
            })
    }
}

fn main() {
    let (exact, passthrough) = Position::new(50, 100).handle_input(std::io::stdin().lock());
    println!("old password: {}", exact);
    println!("new password: {}", passthrough);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_rotation() {
        let start_num = 75;
        for case in [
            (10, (0, 0)),
            (24, (0, 0)),
            (25, (1, 1)),
            (26, (0, 1)),
            (224, (0, 2)),
            (225, (1, 3)),
            (226, (0, 3)),
            (-10, (0, 0)),
            (-74, (0, 0)),
            (-75, (1, 1)),
            (-76, (0, 1)),
            (-574, (0, 5)),
            (-575, (1, 6)),
            (-576, (0, 6)),
        ] {
            let rot = super::Rotation(case.0);
            let result = super::Position::new(start_num, 100).handle_rotation(&rot);
            assert_eq!(result, case.1);
        }
    }

    #[test]
    fn test_handle_rotation_starting_at_zero() {
        let start_num = 0;
        for case in [
            (10, (0, 0)),
            (99, (0, 0)),
            (100, (1, 1)),
            (101, (0, 1)),
            (999, (0, 9)),
            (1000, (1, 10)),
            (1001, (0, 10)),
            (-10, (0, 0)),
            (-99, (0, 0)),
            (-100, (1, 1)),
            (-101, (0, 1)),
            (-399, (0, 3)),
            (-400, (1, 4)),
            (-401, (0, 4)),
        ] {
            let rot = super::Rotation(case.0);
            let result = super::Position::new(start_num, 100).handle_rotation(&rot);
            assert_eq!(result, case.1);
        }
    }

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
        let (exact, passthroughs) = super::Position::new(50, 100).handle_input(test_input);
        assert_eq!(exact, 3);
        assert_eq!(passthroughs, 6);
    }
}
