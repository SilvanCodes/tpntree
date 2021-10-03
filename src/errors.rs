use std::{error::Error, fmt::Display};

#[derive(Debug, Eq, PartialEq)]
pub enum TpnTreeError {
    DoesNotSpan,
    CanNotDivide,
    DimensionMismatch,
}

impl Display for TpnTreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TpnTreeError::DoesNotSpan => write!(
                f,
                "The tree does not span over the provided data coordinates."
            ),
            TpnTreeError::CanNotDivide => write!(f, "The tree has been divided before."),
            TpnTreeError::DimensionMismatch => write!(
                f,
                "The dimension of the data did not match the dimension of the tree."
            ),
        }
    }
}

impl Error for TpnTreeError {}
