use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParseRangeError {
    ParseRange,
    ParseInt(ParseIntError),
}

#[derive(Debug, PartialEq)]
struct MyRange {
    start: usize,
    end: usize,
}

impl FromStr for MyRange {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((left, right)) = s.split_once('-') else {
            return Err(ParseRangeError::ParseRange);
        };
        let start: usize = left.parse().map_err(ParseRangeError::ParseInt)?;
        let end: usize = right.parse().map_err(ParseRangeError::ParseInt)?;
        Ok(MyRange { start, end })
    }
}

impl PartialOrd for MyRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.end < other.start {
            Some(Ordering::Less)
        } else if self.start > other.end {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None // they overlap in some way
        }
    }
}

impl MyRange {
    fn overlaps(&self, other: &MyRange) -> bool {
        !(self.end < other.start || other.end < self.start)
    }

    /// Merge existing range into the receiver. The caller must ensure that the two ranges overlap.
    fn merge(&mut self, other: &MyRange) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    fn total(&self) -> usize {
        self.end - self.start + 1
    }
}

/// A sorted vector of [MyRange]s, where no ranges may overlap. When adding a new range, if it
/// overlaps with any existing range, those ranges should be merged.
#[derive(Debug, PartialEq)]
struct Ranges(Vec<MyRange>);

impl Ranges {
    fn from(lines: impl Iterator<Item = String>) -> Self {
        lines
            .skip_while(|line| line.is_empty())
            .take_while(|line| !line.is_empty())
            .map(|line| MyRange::from_str(&line).unwrap())
            .fold(Ranges(Vec::new()), |mut acc, range| {
                acc.add_range(range);
                acc
            })
    }

    fn add_range(&mut self, mut new: MyRange) {
        if self.0.is_empty() {
            self.0.push(new);
            return;
        }

        let Some((first_matching_index, first_matching_range)) = self
            .0
            .iter()
            .enumerate()
            .find(|&(_, range)| !(*range < new))
        else {
            // new range greater than any existing, so push it to the end
            self.0.push(new);
            return;
        };
        match first_matching_range.partial_cmp(&new) {
            Some(Ordering::Equal) => return, // they're identical
            Some(Ordering::Greater) => return self.0.insert(first_matching_index, new),
            _ => new.merge(first_matching_range), // they overlap, so find the first that doesn't
        }
        let Some((first_non_matching_index, _)) = self
            .0
            .get((first_matching_index + 1)..)
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, range)| {
                if range.overlaps(&new) {
                    new.merge(range);
                    return false;
                }
                true
            })
        else {
            // all remaining ranges overlap
            let _ = self.0.drain(first_matching_index..);
            self.0.push(new);
            return;
        };
        let first_non_matching_index = first_non_matching_index + first_matching_index + 1; // adjust for skipped ranges

        // overwrite the first overlapping entry to preserve it in the vec
        self.0[first_matching_index].merge(&new);
        // remove all other overlapping entries
        let _ = self
            .0
            .drain((first_matching_index + 1)..first_non_matching_index);
    }

    fn contains(&self, number: usize) -> bool {
        match self.0.iter().find(|myrng| !(myrng.end < number)) {
            Some(matching) => matching.start <= number,
            None => false,
        }
    }

    fn total(&self) -> usize {
        self.0.iter().map(|r| r.total()).sum()
    }
}

fn count_fresh(r: impl std::io::BufRead) -> (usize, usize) {
    let mut lines = r.lines().map_while(Result::ok);
    let ranges = Ranges::from(&mut lines);
    let available = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|num| ranges.contains(*num))
        .count();
    let all = ranges.total();
    (available, all)
}

fn main() {
    let (available, all) = count_fresh(std::io::stdin().lock());
    println!("available fresh ingredients: {available}");
    println!("all fresh ingredients: {all}");
}

#[cfg(test)]
mod tests {
    use crate::{count_fresh, MyRange, Ranges};

    const EXAMPLE_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_count_fresh() {
        let input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (available, all) = count_fresh(input);
        assert_eq!((available, all), (3, 14));
    }

    const SINGLETON_INPUT: &str = "
3-5
10-10
11-11
16-20
12-18

1
5
8
10
11
12
18";

    #[test]
    fn test_count_fresh_singleton() {
        let input = std::io::BufReader::new(SINGLETON_INPUT.as_bytes());
        let (available, all) = count_fresh(input);
        assert_eq!((available, all), (5, 14));
    }

    const RANGE_INPUT: &str = "316912306652712-320683419496855
157110396540658-158515545043416
413380390732509-413851343783550
45534978319107-45768124861513
13873831532241-16714933495213
415961886159964-416594970472954
543818828813452-545340095506657
545666714619049-547049232876190
292208729101773-294545425285400
354113252785914-354113252785914
415961886159964-416290773279649
85848681005753-89832035631476
154864348091097-156513462758390
383854415172363-387779080829907
508100788284877-508253922520635
224767428559384-225090632954429
406367833241454-411289155251763
509481120146979-510324215823697
234467272956575-237623862906337
453363172626346-458685448350103";

    const RANGE_INPUT_SORTED: &str = "
13873831532241-16714933495213
45534978319107-45768124861513
85848681005753-89832035631476
154864348091097-156513462758390
157110396540658-158515545043416
224767428559384-225090632954429
234467272956575-237623862906337
292208729101773-294545425285400
316912306652712-320683419496855
354113252785914-354113252785914
383854415172363-387779080829907
406367833241454-411289155251763
413380390732509-413851343783550
415961886159964-416290773279649
415961886159964-416594970472954
453363172626346-458685448350103
543818828813452-545340095506657
545666714619049-547049232876190
508100788284877-508253922520635
509481120146979-510324215823697";

    #[test]
    fn test_ranges_from() {
        let ranges = Ranges::from(RANGE_INPUT.lines().map(|s| s.to_string()));
        assert_eq!(
            ranges,
            Ranges(vec![
                MyRange {
                    start: 13873831532241,
                    end: 16714933495213
                },
                MyRange {
                    start: 45534978319107,
                    end: 45768124861513
                },
                MyRange {
                    start: 85848681005753,
                    end: 89832035631476
                },
                MyRange {
                    start: 154864348091097,
                    end: 156513462758390
                },
                MyRange {
                    start: 157110396540658,
                    end: 158515545043416
                },
                MyRange {
                    start: 224767428559384,
                    end: 225090632954429
                },
                MyRange {
                    start: 234467272956575,
                    end: 237623862906337
                },
                MyRange {
                    start: 292208729101773,
                    end: 294545425285400
                },
                MyRange {
                    start: 316912306652712,
                    end: 320683419496855
                },
                MyRange {
                    start: 354113252785914,
                    end: 354113252785914
                },
                MyRange {
                    start: 383854415172363,
                    end: 387779080829907
                },
                MyRange {
                    start: 406367833241454,
                    end: 411289155251763
                },
                MyRange {
                    start: 413380390732509,
                    end: 413851343783550
                },
                MyRange {
                    start: 415961886159964,
                    end: 416594970472954
                },
                MyRange {
                    start: 453363172626346,
                    end: 458685448350103
                },
                MyRange {
                    start: 508100788284877,
                    end: 508253922520635
                },
                MyRange {
                    start: 509481120146979,
                    end: 510324215823697
                },
                MyRange {
                    start: 543818828813452,
                    end: 545340095506657
                },
                MyRange {
                    start: 545666714619049,
                    end: 547049232876190
                },
            ])
        )
    }

    #[test]
    fn test_ranges_from_presorted() {
        let ranges = Ranges::from(RANGE_INPUT_SORTED.lines().map(|s| s.to_string()));
        assert_eq!(
            ranges,
            Ranges(vec![
                MyRange {
                    start: 13873831532241,
                    end: 16714933495213
                },
                MyRange {
                    start: 45534978319107,
                    end: 45768124861513
                },
                MyRange {
                    start: 85848681005753,
                    end: 89832035631476
                },
                MyRange {
                    start: 154864348091097,
                    end: 156513462758390
                },
                MyRange {
                    start: 157110396540658,
                    end: 158515545043416
                },
                MyRange {
                    start: 224767428559384,
                    end: 225090632954429
                },
                MyRange {
                    start: 234467272956575,
                    end: 237623862906337
                },
                MyRange {
                    start: 292208729101773,
                    end: 294545425285400
                },
                MyRange {
                    start: 316912306652712,
                    end: 320683419496855
                },
                MyRange {
                    start: 354113252785914,
                    end: 354113252785914
                },
                MyRange {
                    start: 383854415172363,
                    end: 387779080829907
                },
                MyRange {
                    start: 406367833241454,
                    end: 411289155251763
                },
                MyRange {
                    start: 413380390732509,
                    end: 413851343783550
                },
                MyRange {
                    start: 415961886159964,
                    end: 416594970472954
                },
                MyRange {
                    start: 453363172626346,
                    end: 458685448350103
                },
                MyRange {
                    start: 508100788284877,
                    end: 508253922520635
                },
                MyRange {
                    start: 509481120146979,
                    end: 510324215823697
                },
                MyRange {
                    start: 543818828813452,
                    end: 545340095506657
                },
                MyRange {
                    start: 545666714619049,
                    end: 547049232876190
                },
            ])
        )
    }
}
