use std::num::ParseIntError;

/// Returns true if the number is two repeating sequences of digits. For example, 99, or 1212, or
/// 98769876.
fn is_invalid(num: usize) -> bool {
    let length = num.ilog10() + 1;
    if length % 2 == 1 {
        return false;
    }
    let base10mask = 10_usize.pow(length / 2);
    num / base10mask == num % base10mask
}

fn is_invalid_2(num: usize) -> bool {
    let length = num.ilog10() + 1;
    if length < 2 {
        return false;
    }
    for candidate in (1..=(length / 2)).filter(|x| length.is_multiple_of(*x)) {
        // Only need to check prime factors, but easier to just check all
        let n_copies = length / candidate;
        let base10mask = 10_usize.pow(candidate);
        let target = num % base10mask;
        if (1..n_copies)
            .map(|y| (num / base10mask.pow(y)) % base10mask) // shift right by y mask-widths and mask
            .all(|z| z == target)
        {
            return true;
        }
    }
    false
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

fn filter_invalid_ids_2(ids: impl Iterator<Item = usize>) -> impl Iterator<Item = usize> {
    ids.filter(|id| is_invalid_2(*id))
}

fn main() {
    let (pt1, pt2) = find_all_ids(std::io::stdin().lock()).fold((0, 0), |acc, id| {
        match (is_invalid(id), is_invalid_2(id)) {
            (true, true) => (acc.0 + id, acc.1 + id),
            (true, false) => (acc.0 + id, acc.1),
            (false, true) => (acc.0, acc.1 + id),
            _ => acc,
        }
    });
    println!("sum of invalid IDs part 1: {pt1}");
    println!("sum of invalid IDs part 2: {pt2}");
}

#[cfg(test)]
mod tests {
    use crate::{filter_invalid_ids, filter_invalid_ids_2, find_all_ids, is_invalid, is_invalid_2};

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
            (100, false),
            (111, false),
            (121, false),
            (999, false),
            (1000, false),
            (1212, true),
            (1221, false),
            (121212, false),
            (446446, true),
            (38593859, true),
            (824824824, false),
            (2121212121, false),
        ];
        let result = expected
            .iter()
            .map(|(n, _)| (*n, is_invalid(*n)))
            .collect::<Vec<_>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_invalid_2() {
        let expected: Vec<(usize, bool)> = vec![
            (5, false),
            (55, true),
            (56, false),
            (100, false),
            (111, true),
            (121, false),
            (999, true),
            (1000, false),
            (1212, true),
            (1221, false),
            (121212, true),
            (446446, true),
            (38593859, true),
            (824824824, true),
            (2121212121, true),
        ];
        let result = expected
            .iter()
            .map(|(n, _)| (*n, is_invalid_2(*n)))
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

    #[test]
    fn test_filter_invalid_ids_2() {
        let input = std::io::BufReader::new(SIMPLE_INPUT.as_bytes());
        let result: Vec<usize> = filter_invalid_ids_2(find_all_ids(input)).collect();
        assert_eq!(result, vec![11]);
    }

    #[test]
    fn test_filter_invalid_ids_2_oneline() {
        let input = std::io::BufReader::new(EXAMPLE_ONELINE.as_bytes());
        let result: Vec<usize> = filter_invalid_ids_2(find_all_ids(input)).collect();
        assert_eq!(
            result,
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ]
        )
    }

    #[test]
    fn test_filter_invalid_ids_2_multiline() {
        let input = std::io::BufReader::new(EXAMPLE_MULTILINE.as_bytes());
        let result: Vec<usize> = filter_invalid_ids_2(find_all_ids(input)).collect();
        assert_eq!(
            result,
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ]
        )
    }
}
