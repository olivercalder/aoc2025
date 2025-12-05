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

    fn new_with_roll(is_roll: bool) -> Entry {
        Entry {
            is_roll,
            neighbors: 0,
        }
    }

    fn set_roll(&mut self) {
        self.is_roll = true;
    }

    fn unset_roll(&mut self) {
        self.is_roll = false;
    }

    fn inc_neighbors(&mut self) {
        self.neighbors += 1;
    }

    fn dec_neighbors(&mut self) {
        self.is_roll;
        self.neighbors;
        self.neighbors -= 1;
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

struct Room {
    height: usize,
    width: usize,
    rows: Vec<Vec<Entry>>,
}

impl Room {
    fn from(r: impl std::io::BufRead) -> Room {
        let rows: Vec<Vec<Entry>> = r
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| Entry::new_with_roll(c == '@'))
                    .collect::<Vec<Entry>>()
            })
            .collect();
        let height = rows.len();
        let width = rows.last().unwrap().len();
        Room {
            height,
            width,
            rows,
        }
        .prepare()
    }

    // This should probably be optimized more...
    fn find_neighbors(&self, r: usize, c: usize, neighbors: &mut Vec<(usize, usize)>) {
        neighbors.clear();
        let n_r = r + 1;
        let n_c = c + 1;
        if r > 0 {
            let p_r = r - 1;
            if c > 0 {
                neighbors.push((p_r, c - 1));
            }
            neighbors.push((p_r, c));
            if n_c < self.width {
                neighbors.push((p_r, n_c));
            }
        }
        if c > 0 {
            neighbors.push((r, c - 1));
        }
        if n_c < self.width {
            neighbors.push((r, n_c));
        }
        if n_r < self.height {
            if c > 0 {
                neighbors.push((n_r, c - 1));
            }
            neighbors.push((n_r, c));
            if n_c < self.width {
                neighbors.push((n_r, n_c));
            }
        }
    }

    /// Assumes all neighbor counts are initially 0. Should only be called when initializing a new
    /// [Room].
    fn prepare(mut self) -> Self {
        let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(8);
        for i in 0..self.height {
            for j in 0..self.width {
                if !self.rows[i][j].is_roll {
                    continue;
                }
                self.find_neighbors(i, j, &mut neighbors);
                for (x, y) in &neighbors {
                    // Probably faster to use checked getter methods rather than pre-check coords
                    // and then do checked indexing
                    self.rows[*x][*y].inc_neighbors();
                }
            }
        }
        self
    }

    /// Removes any movable rolls, returning the total number which are movable. Rolls are greedily
    /// removed, so a roll which was not removable at the beginning of the sweep may become movable
    /// as the result of the removal of previous rolls during the sweep, and thus be itself removed
    /// during that sweep.
    fn sweep(&mut self) -> usize {
        let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(8);
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if !self.rows[i][j].is_movable() {
                    continue;
                }
                count += 1;
                self.rows[i][j].unset_roll();
                self.find_neighbors(i, j, &mut neighbors);
                for (x, y) in &neighbors {
                    self.rows[*x][*y].dec_neighbors();
                }
            }
        }
        count
    }
}

fn count_initially_movable(r: impl std::io::BufRead) -> usize {
    let mut rememberer = RowRememberer::new();
    let all_but_last: usize = r
        .lines()
        .map_while(Result::ok)
        .map(|line| rememberer.handle_row(&line))
        .sum();
    all_but_last + rememberer.tally_prev_row()
}

fn count_eventually_movable(r: impl std::io::BufRead) -> usize {
    let mut room = Room::from(r);
    let mut total_moved = 0;
    loop {
        let count = room.sweep();
        if count == 0 {
            break;
        }
        total_moved += count;
    }
    total_moved
}

fn main() {
    // Copy stdin out of laziness, we're going to make a full representation anyway...
    let input: String =
        std::io::stdin()
            .lines()
            .map_while(Result::ok)
            .fold(String::new(), |mut acc, line| {
                acc.push_str(&line);
                acc.push('\n');
                acc
            });
    let initially_movable = count_initially_movable(std::io::BufReader::new(input.as_bytes()));
    println!("Initially movable rolls: {initially_movable}");
    let eventually_movable = count_eventually_movable(std::io::BufReader::new(input.as_bytes()));
    println!("Eventually movable rolls: {eventually_movable}");
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
    fn test_count_initially_movable() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_initially_movable(test_input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_count_eventually_movable() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_eventually_movable(test_input);
        assert_eq!(result, 43);
    }

    #[test]
    fn test_find_neighbors() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let room = super::Room::from(test_input);
        for ((i, j), expected) in vec![
            ((0, 0), vec![(0, 1), (1, 0), (1, 1)]),
            ((0, 1), vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]),
            ((0, 8), vec![(0, 7), (0, 9), (1, 7), (1, 8), (1, 9)]),
            ((0, 9), vec![(0, 8), (1, 8), (1, 9)]),
            ((1, 0), vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]),
            (
                (1, 1),
                vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (1, 0),
                    (1, 2),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                ],
            ),
            (
                (1, 8),
                vec![
                    (0, 7),
                    (0, 8),
                    (0, 9),
                    (1, 7),
                    (1, 9),
                    (2, 7),
                    (2, 8),
                    (2, 9),
                ],
            ),
            ((1, 9), vec![(0, 8), (0, 9), (1, 8), (2, 8), (2, 9)]),
            (
                (5, 5),
                vec![
                    (4, 4),
                    (4, 5),
                    (4, 6),
                    (5, 4),
                    (5, 6),
                    (6, 4),
                    (6, 5),
                    (6, 6),
                ],
            ),
            ((8, 0), vec![(7, 0), (7, 1), (8, 1), (9, 0), (9, 1)]),
            (
                (8, 1),
                vec![
                    (7, 0),
                    (7, 1),
                    (7, 2),
                    (8, 0),
                    (8, 2),
                    (9, 0),
                    (9, 1),
                    (9, 2),
                ],
            ),
            (
                (8, 8),
                vec![
                    (7, 7),
                    (7, 8),
                    (7, 9),
                    (8, 7),
                    (8, 9),
                    (9, 7),
                    (9, 8),
                    (9, 9),
                ],
            ),
            ((8, 9), vec![(7, 8), (7, 9), (8, 8), (9, 8), (9, 9)]),
            ((9, 0), vec![(8, 0), (8, 1), (9, 1)]),
            ((9, 1), vec![(8, 0), (8, 1), (8, 2), (9, 0), (9, 2)]),
            ((9, 8), vec![(8, 7), (8, 8), (8, 9), (9, 7), (9, 9)]),
            ((9, 9), vec![(8, 8), (8, 9), (9, 8)]),
        ] {
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            room.find_neighbors(i, j, &mut neighbors);
            assert_eq!(neighbors, expected);
        }
    }
}
