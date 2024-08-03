use std::fmt::Display;

#[derive(Debug)]
/// `daff-rs`'s `Diff` struct.
///
/// The `Diff` struct stores reconciliation data.
pub struct Diff {
    curr_source: String,
    next_soruce: String,
    pub data: String,
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
    source: String,
}

impl Csv {
    pub fn new(buffer: String) -> Self {
        Self { source: buffer }
    }

    /// Compute the diff with another `Csv`.
    pub fn compare(&self, other: &Self) -> Diff {
        compare_source(&self.source, &other.source)
    }
}

fn compare_source(curr: &str, next: &str) -> Diff {
    Diff {
        curr_source: curr.to_string(),
        next_soruce: next.to_string(),
        data: String::from("TODO"),
    }
}
