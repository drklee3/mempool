use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fee(pub f64);

impl Ord for Fee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl Eq for Fee {}
