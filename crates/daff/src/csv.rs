use std::{collections::HashSet, fmt::Display};

/// A `Csv` data structure.
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

    /// Compute the `Changes` from another `Csv`.
    pub fn compare(&self, other: &Self) -> Changes {
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

fn compare_data(curr: &Vec<Vec<String>>, next: &Vec<Vec<String>>) -> Changes {
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

    changes
}

#[derive(Debug)]
pub struct Changes {
    pub positions: HashSet<[usize; 2]>,
}

impl Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
