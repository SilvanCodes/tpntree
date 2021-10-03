//! This crate contains an N-dimensional generalization of a region quadtree called **T**wo-**p**ower-__n__-tree or tpntree,
//! as there exist 2^N children per node, where N is the number of dimensions.
//! A quadtree is the two-dimensional case, an octtree is the three-dimensional case of the tpntree.

mod errors;
pub mod tpntree;
pub mod tpntree_dynamic;

pub use errors::TpnTreeError;

/// [`Coordinates`] is required for a type to be used inside a [`SpatialTree`].
pub trait Coordinates {
    fn coordinates(&self) -> &[f64];
}

impl<const N: usize> Coordinates for [f64; N] {
    fn coordinates(&self) -> &[f64] {
        self
    }
}

impl Coordinates for Vec<f64> {
    fn coordinates(&self) -> &[f64] {
        self
    }
}
