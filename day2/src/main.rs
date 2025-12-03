use std::num::ParseIntError;
use std::str::FromStr;

/// Returns true if the number is two repeating sequences of digits. For example, 99, or 1212, or
/// 98769876.
fn is_invalid(num: usize) -> bool {
    let log10 = num.ilog10();
    if log10 % 2 == 0 {
        return false;
    }
    let base10mask = 10_usize.pow(log10.div_ceil(2));
    num / base10mask == num % base10mask
}

enum ParseRangeError {
    ParseNums,
    ParseInt(ParseIntError),
}

fn parse_range(s: &str) -> Result<(usize, usize), ParseRangeError> {
    let nums: Vec<&str> = s.split('-').collect();
    if nums.len() != 2 {
        return Err(ParseRangeError::ParseNums);
    }
    let start: usize = nums[0].parse().map_err(ParseRangeError::ParseInt)?;
    let end: usize = nums[1].parse().map_err(ParseRangeError::ParseInt)?;
    Ok((start, end))
}

fn find_all_ids(r: impl std::io::BufRead) -> impl Iterator<Item = usize> {
    r.lines()
        .map_while(Result::ok)
        .flat_map(|line| {
            line.split(',')
                .filter(|entry| !entry.is_empty())
                .map(parse_range)
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        })
        .flat_map(|(start, end)| start..=end)
}

fn filter_invalid_ids(ids: impl Iterator<Item = usize>) -> impl Iterator<Item = usize> {
    ids.filter(|id| is_invalid(*id))
}

fn main() {
    let all = find_all_ids(std::io::stdin().lock());
    let invalid = filter_invalid_ids(all);
    let sum: usize = invalid.sum();
    println!("sum of invalid IDs: {sum}")
}

#[cfg(test)]
mod tests {
    use crate::{filter_invalid_ids, find_all_ids, is_invalid};

    const SIMPLE_INPUT: &str = "2-5,9-11";
    const EXAMPLE_ONELINE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const EXAMPLE_MULTILINE: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_invalid() {
        let expected: Vec<(usize, bool)> = vec![
            (5, false),
            (55, true),
            (56, false),
            (111, false),
            (121, false),
            (1212, true),
            (1221, false),
        ];
        let result = expected
            .iter()
            .map(|(n, _)| (*n, is_invalid(*n)))
            .collect::<Vec<_>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_all_ids() {
        let input = std::io::BufReader::new(SIMPLE_INPUT.as_bytes());
        let result: Vec<usize> = find_all_ids(input).collect();
        assert_eq!(result, vec![2, 3, 4, 5, 9, 10, 11]);
    }

    #[test]
    fn test_filter_invalid_ids() {
        let input = std::io::BufReader::new(SIMPLE_INPUT.as_bytes());
        let result: Vec<usize> = filter_invalid_ids(find_all_ids(input)).collect();
        assert_eq!(result, vec![11]);
    }

    #[test]
    fn test_filter_invalid_ids_oneline() {
        let input = std::io::BufReader::new(EXAMPLE_ONELINE.as_bytes());
        let result: Vec<usize> = filter_invalid_ids(find_all_ids(input)).collect();
        assert_eq!(
            result,
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859]
        )
    }

    #[test]
    fn test_filter_invalid_ids_multiline() {
        let input = std::io::BufReader::new(EXAMPLE_MULTILINE.as_bytes());
        let result: Vec<usize> = filter_invalid_ids(find_all_ids(input)).collect();
        assert_eq!(
            result,
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859]
        )
    }
}
