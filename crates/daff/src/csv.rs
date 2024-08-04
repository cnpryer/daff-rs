use std::{collections::HashSet, fmt::Display};

/// A `Csv` data structure.
///
/// The `Csv` is used to compute diffs between two CSV sources.
///
/// ```rs
/// use daff::Csv;
///
/// let csv = Csv::new("a,b,c\n1,2,3");
///
/// println!(csv.compare(Csv::new("a,b,c\n1,2,4")));
/// ```
pub struct Csv {
    data: Vec<Vec<String>>,
}

impl Csv {
    pub fn new(buffer: String) -> Self {
        Self {
            data: read_csv(buffer),
        }
    }

    /// Compute the diff with another `Csv`.
    pub fn compare(&self, other: &Self) -> Diff {
        compare_data(&self.data, &other.data)
    }
}

// naive
fn read_csv(buffer: String) -> Vec<Vec<String>> {
    let mut data = vec![];

    for row in buffer.split("\n") {
        let mut data_row = vec![];

        for value in row.split(",") {
            data_row.push(value.to_string());
        }

        data.push(data_row);
    }

    data
}

fn compare_data(curr: &Vec<Vec<String>>, next: &Vec<Vec<String>>) -> Diff {
    let mut changes = Changes {
        positions: HashSet::new(),
    };

    for (i, row) in curr.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value != &next[i][j] {
                changes.positions.insert([i, j]);
            }
        }
    }

    Diff { changes }
}

#[derive(Debug)]
/// `daff-rs`'s `Diff` struct.
///
/// The `Diff` struct stores reconciliation data.
pub struct Diff {
    pub changes: Changes,
}

#[derive(Debug)]
pub struct Changes {
    pub positions: HashSet<[usize; 2]>,
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
