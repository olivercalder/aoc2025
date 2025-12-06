use std::num::ParseIntError;
use std::str::FromStr;

enum ParseNumsOrOpsError {
    ParseNum(ParseIntError),
    ParseOp,
    ParseNeither,
    ParseEmpty,
}

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

fn main() {
    let result: usize = vertical_math(std::io::stdin().lock()).sum();
    println!("Sum of vertical computations: {result}");
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
}
