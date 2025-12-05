use std::mem;

// At each position with roll, look at the current count which has been placed on the position,
// then look to the right and one row down (three touching positions), and add the total number of
// rolls together. Also, add 1 to each of those positions which has a roll.

#[derive(Clone)]
struct Entry {
    is_roll: bool,
    neighbors: usize,
}

impl Entry {
    fn new() -> Entry {
        Entry {
            is_roll: false,
            neighbors: 0,
        }
    }

    fn set_roll(&mut self) {
        self.is_roll = true;
    }

    fn inc_neighbors(&mut self) {
        self.neighbors += 1;
    }

    fn is_movable(&self) -> bool {
        self.is_roll && self.neighbors < 4
    }
}

/// Remember the previous row and the current row. When a new row is processed, make the final
/// additions to the previous row, update the current row (setting is_roll correctly), and create
/// the next row. At the end, add the
struct RowRememberer {
    width: usize,
    prev_row: Vec<Entry>,
    curr_row: Vec<Entry>,
}

impl RowRememberer {
    fn new() -> Self {
        RowRememberer {
            width: 0,
            prev_row: Vec::new(),
            curr_row: Vec::new(),
        }
    }

    /// Process the given row and return the number of rolls in the previous row which could be
    /// moved.
    fn handle_row(&mut self, row: &str) -> usize {
        if row.is_empty() {
            // Should not occur, ignore this row
            return 0;
        }
        // Assume all non-empty rows have the same width
        if self.width == 0 {
            // First row, so current row full of empty entries
            self.width = row.len();
            self.prev_row = vec![Entry::new(); self.width];
            self.curr_row = vec![Entry::new(); self.width];
        }
        let mut next = vec![Entry::new(); self.width];
        for index in row
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '@')
            .map(|(i, _)| i)
        {
            if let Some(left) = index.checked_sub(1) {
                self.prev_row[left].inc_neighbors();
                self.curr_row[left].inc_neighbors();
                next[left].inc_neighbors();
            }
            self.curr_row[index].set_roll();
            self.prev_row[index].inc_neighbors();
            next[index].inc_neighbors();
            let right = index + 1;
            if right < self.width {
                self.prev_row[right].inc_neighbors();
                self.curr_row[right].inc_neighbors();
                next[right].inc_neighbors();
            }
        }
        let prev_count = self.tally_prev_row();
        // (self.prev_row, self.curr_row) = (self.curr_row, next);
        mem::swap(&mut self.prev_row, &mut self.curr_row);
        self.curr_row = next;
        prev_count
    }

    fn tally_prev_row(&self) -> usize {
        self.prev_row.iter().filter(|e| e.is_movable()).count()
    }
}

fn count_movable(r: impl std::io::BufRead) -> usize {
    let mut rememberer = RowRememberer::new();
    let all_but_last: usize = r
        .lines()
        .map_while(Result::ok)
        .map(|line| rememberer.handle_row(&line))
        .sum();
    all_but_last + rememberer.tally_prev_row()
}

fn main() {
    let result = count_movable(std::io::stdin().lock());
    println!("Movable rolls: {result}");
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_count_movable() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_movable(test_input);
        assert_eq!(result, 13);
    }
}
