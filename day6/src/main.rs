use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

enum ParseNumsOrOpsError {
    ParseNum(ParseIntError),
    ParseOp,
    ParseNeither,
    ParseEmpty,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = ParseNumsOrOpsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(ParseNumsOrOpsError::ParseOp),
        }
    }
}

enum NumsOrOps {
    Nums(Vec<usize>),
    Ops(Vec<Op>),
}

impl FromStr for NumsOrOps {
    type Err = ParseNumsOrOpsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split_whitespace();
        let first = vals.next().ok_or(ParseNumsOrOpsError::ParseEmpty)?;
        let mut new = NumsOrOps::new(first)?;
        for val in vals {
            new.add(val)?;
        }
        Ok(new)
    }
}

impl NumsOrOps {
    fn new(first_val: &str) -> Result<Self, ParseNumsOrOpsError> {
        if let Ok(num) = first_val.parse::<usize>() {
            Ok(NumsOrOps::Nums(vec![num]))
        } else if let Ok(op) = first_val.parse::<Op>() {
            Ok(NumsOrOps::Ops(vec![op]))
        } else {
            Err(ParseNumsOrOpsError::ParseNeither)
        }
    }

    fn add(&mut self, val: &str) -> Result<(), ParseNumsOrOpsError> {
        match self {
            NumsOrOps::Nums(nums) => nums.push(
                val.parse::<usize>()
                    .map_err(ParseNumsOrOpsError::ParseNum)?,
            ),
            NumsOrOps::Ops(ops) => ops.push(val.parse::<Op>()?),
        }
        Ok(())
    }

    fn len(&self) -> usize {
        match self {
            NumsOrOps::Nums(nums) => nums.len(),
            NumsOrOps::Ops(ops) => ops.len(),
        }
    }
}

fn vertical_math(r: impl std::io::BufRead) -> impl Iterator<Item = usize> {
    let mut cols: Vec<Vec<usize>> = Vec::new();
    r.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| NumsOrOps::from_str(&line))
        .filter_map(Result::ok)
        .find_map(|row| {
            while cols.len() < row.len() {
                // should only occur on the first row
                cols.push(Vec::new());
            }
            match row {
                NumsOrOps::Nums(nums) => {
                    for (i, num) in nums.into_iter().enumerate() {
                        cols[i].push(num);
                    }
                    None
                }
                NumsOrOps::Ops(ops) => Some(ops),
            }
        })
        .unwrap()
        .into_iter()
        .zip(cols)
        .map(|(op, col)| match op {
            Op::Add => col.into_iter().sum(),
            Op::Mul => col.into_iter().product(),
        })
}

struct RawColumn {
    num: usize,
    op: Option<Op>,
}

#[derive(Debug)]
struct SemanticColumn {
    nums: Vec<usize>,
    op: Op,
}

impl SemanticColumn {
    fn compute(&self) -> usize {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mul => self.nums.iter().product(),
        }
    }
}

/// [GridReader] is an iterator over the [SemanticColumn]s in a grid.
struct GridReader {
    width: usize,
    curr_col: usize,
    grid: Vec<String>, // for simplicity, split and own
}

impl GridReader {
    fn new(r: impl std::io::BufRead) -> Self {
        let rows: Vec<String> = r
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .collect();
        GridReader {
            width: rows.iter().map(|r| r.len()).max().unwrap(),
            curr_col: 0,
            grid: rows,
        }
    }

    fn next_raw_column(&mut self) -> Option<RawColumn> {
        if self.curr_col >= self.width {
            return None;
        }
        let mut pos = self.curr_col;
        self.curr_col += 1;
        let mut digits = String::new();
        let mut op: Option<Op> = None;
        for row in &self.grid {
            let Some(c) = row.as_bytes().get(pos) else {
                continue;
            };
            match c {
                b'0'..=b'9' => digits.push((*c).into()),
                b'+' => op = Some(Op::Add),
                b'*' => op = Some(Op::Mul),
                _ => {} // ignore it
            }
        }
        if digits.is_empty() {
            return None;
        }
        let num: usize = digits.parse().unwrap();
        Some(RawColumn { num, op })
    }
}

impl Iterator for GridReader {
    type Item = SemanticColumn;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col >= self.width {
            return None;
        }
        let mut nums: Vec<usize> = Vec::new();
        let mut op: Option<Op> = None;
        while let Some(raw_col) = self.next_raw_column() {
            nums.push(raw_col.num);
            op = op.or(raw_col.op);
        }
        op.map(|o| SemanticColumn { nums, op: o })
    }
}

fn columnar_math(r: impl std::io::BufRead) -> impl Iterator<Item = usize> {
    let reader = GridReader::new(r);
    reader.map(|sem_col| sem_col.compute())
}

fn main() {
    let mut input_buf = Vec::new();
    std::io::stdin().lock().read_to_end(&mut input_buf).unwrap();
    let complete_input = String::from_utf8(input_buf).unwrap();
    let standard: usize = vertical_math(std::io::BufReader::new(complete_input.as_bytes())).sum();
    println!("Sum of standard computations: {standard}");
    let columnar: usize = columnar_math(std::io::BufReader::new(complete_input.as_bytes())).sum();
    println!("Sum of columnar computations: {columnar}");
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_vertical_math() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result: Vec<usize> = super::vertical_math(test_input).collect();
        assert_eq!(result, vec![33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_columnar_math() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result: Vec<usize> = super::columnar_math(test_input).collect();
        assert_eq!(result, vec![8544, 625, 3253600, 1058]);
    }
}
