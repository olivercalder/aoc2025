use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ParseBatteryError {
    TooShort,
    ParseBattery,
    ParseInt(ParseIntError),
}

// Naive, simple approach which is O(N*M) for len N and line with length M. But it doesn't matter,
// Rust is fast.
fn max_battery_of_length(len: usize, line: &str) -> Result<usize, ParseBatteryError> {
    if line.len() < len {
        return Err(ParseBatteryError::TooShort);
    }
    let mut digits = String::new();
    let mut prev_index: isize = -1; // a hack so we start looking at 0
    for i in 0..len {
        let start_index = (prev_index + 1) as usize;
        let (greatest, neg_ind) = line
            .get(start_index..(line.len() - len + 1 + i))
            .ok_or(ParseBatteryError::ParseBattery)?
            .chars()
            .enumerate()
            .map(|(ind, byt)| (byt, -(ind as isize)))
            .max()
            .ok_or(ParseBatteryError::ParseBattery)?;
        digits.push(greatest);
        prev_index = start_index as isize - neg_ind;
    }
    digits.parse().map_err(ParseBatteryError::ParseInt)
}

fn extract_batteries(r: impl std::io::BufRead) -> impl Iterator<Item = (usize, usize)> {
    r.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                max_battery_of_length(2, &line).unwrap(),
                max_battery_of_length(12, &line).unwrap(),
            )
        })
}

fn main() {
    let (orig, static_friction): (usize, usize) = extract_batteries(std::io::stdin().lock())
        .fold((0, 0), |acc, joltages| {
            (acc.0 + joltages.0, acc.1 + joltages.1)
        });
    println!("Sum of batteries: {orig}");
    println!("Sum of batteries with static friction: {static_friction}");
}

#[cfg(test)]
mod tests {
    use crate::{extract_batteries, max_battery_of_length};
    use std::io::BufRead;

    const EXAMPLE_INPUT: &str = "
987654321111111
811111111111119
234234234234278
818181911112111";

    const LONGER_INPUT: &str = "
3133322312313332336153233333232281412234221222433272332313372222212233114622232233232321251122522243
3122243233322223222333513239233621333333523352333332361333233332142327423622333222313333242321112633
4453322423234323362634238645943333332463321659433346534324232461344544333233244323632243313334262243
6448895538826857235274976247543575444367645757464434697575874665478695238342662743886877975373645693
1134322241232322332224331133221412522322512233243421322616252222333223234632221323236212122235222232";

    #[test]
    fn test_max_battery_of_length_2() {
        let expected = vec![98, 89, 78, 92];
        for (line, exp) in std::io::BufReader::new(EXAMPLE_INPUT.as_bytes())
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .zip(expected)
        {
            assert_eq!(max_battery_of_length(2, &dbg!(line)), Ok(exp));
        }
    }

    #[test]
    fn test_max_battery_of_length_12() {
        let expected = vec![987654321111, 811111111119, 434234234278, 888911112111];
        for (line, exp) in std::io::BufReader::new(EXAMPLE_INPUT.as_bytes())
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .zip(expected)
        {
            assert_eq!(max_battery_of_length(12, &dbg!(line)), Ok(exp));
        }
    }

    #[test]
    fn test_extract_batteries() {
        let input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result: Vec<(usize, usize)> = extract_batteries(input).collect();
        assert_eq!(
            result,
            vec![
                (98, 987654321111),
                (89, 811111111119),
                (78, 434234234278),
                (92, 888911112111)
            ]
        );
    }

    #[test]
    fn test_extract_batteries_longer_input() {
        let input = std::io::BufReader::new(LONGER_INPUT.as_bytes());
        let result: Vec<usize> = extract_batteries(input).map(|(x, _)| x).collect();
        assert_eq!(result, vec![87, 97, 99, 99, 66]);
    }
}
