use std::fmt::Display;

use similar::TextDiff;

pub struct Text(String);

impl Text {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn compare(&self, other: &Self) -> Changes {
        compare_text(&self, &other)
    }
}

fn compare_text(curr: &Text, next: &Text) -> Changes {
    Changes {
        diff: TextDiff::from_lines(&curr.0, &next.0)
            .unified_diff()
            .to_string(),
    }
}

#[derive(Debug)]
pub struct Changes {
    pub diff: String,
}

impl Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
