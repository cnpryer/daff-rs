use std::fmt::Display;

/// `daff-rs`'s `Diff` struct.
///
/// The `Diff` struct stores reconciliation data.
pub struct Diff {}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
    data: Data,
}

impl Csv {
    pub fn new(buffer: String) -> Self {
        Self {
            data: Data::from_csv_buffer(buffer),
        }
    }

    /// Compute the diff from another Csv.
    pub fn compare(&self, other: &Self) -> Diff {
        compare_data(&self.data, &other.data)
    }
}

fn compare_data(a: &Data, b: &Data) -> Diff {
    Diff {}
}

/// The `Data` data structure.
///
/// `Data` stores a standard column-based layout for data at the moment.
struct Data {
    inner: Vec<Vec<String>>,
}

impl Data {
    // helper
    fn from_csv_buffer(buffer: String) -> Self {
        Data { inner: Vec::new() }
    }
}
