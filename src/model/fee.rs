use std::cmp::Ord;
use std::cmp::Ordering;

/// A wrapper type for a float in order to manually implement Ord traits to be
/// able to use it as a key in the BTreeMap. Due to float inconsistencies, this
/// may not be an ideal long term strategy and could be split off into something
/// more accurate and lossless such as a sign-exponent-mantissa triplet.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fee(pub f64);

impl Ord for Fee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl Eq for Fee {}
