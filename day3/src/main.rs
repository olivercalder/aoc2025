use std::num::ParseIntError;

enum ParseBatteryError {
    ParseBattery,
    ParseInt(ParseIntError),
}

fn max_battery(line: &str) -> Result<usize, ParseBatteryError> {
    if line.len() < 2 {
        return Err(ParseBatteryError::ParseBattery);
    }
    let (greatest, neg_ind) = line
        .get(..(line.len() - 1))
        .ok_or(ParseBatteryError::ParseBattery)?
        .chars()
        .enumerate()
        .map(|(ind, byt)| (byt, -(ind as isize)))
        .max()
        .ok_or(ParseBatteryError::ParseBattery)?;
    // index was negated so we get the first occurrence of the greatest digit
    let index: usize = (-neg_ind) as usize;
    let second = line
        .get((index + 1)..)
        .ok_or(ParseBatteryError::ParseBattery)?
        .chars()
        .max()
        .ok_or(ParseBatteryError::ParseBattery)?;
    format!("{greatest}{second}")
        .parse()
        .map_err(ParseBatteryError::ParseInt)
}

fn extract_batteries(r: impl std::io::BufRead) -> impl Iterator<Item = usize> {
    r.lines()
        .map_while(Result::ok)
        .map(|line| max_battery(&line))
        .filter_map(Result::ok)
}

fn main() {
    let result: usize = extract_batteries(std::io::stdin().lock()).sum();
    println!("Sum of batteries: {result}");
}

#[cfg(test)]
mod tests {
    use crate::{extract_batteries, max_battery};

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
    fn test_extract_batteries() {
        let input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result: Vec<usize> = extract_batteries(input).collect();
        assert_eq!(result, vec![98, 89, 78, 92]);
    }

    #[test]
    fn test_extract_batteries_longer() {
        let input = std::io::BufReader::new(LONGER_INPUT.as_bytes());
        let result: Vec<usize> = extract_batteries(input).collect();
        assert_eq!(result, vec![87, 97, 99, 99, 66]);
    }
}
