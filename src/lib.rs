//! This crate contains an N-dimensional generalization of a region quadtree called **T**wo-**p**ower-__n__-tree or tpntree,
//! as there exist 2^N children per node, where N is the number of dimensions.
//! A quadtree is the two-dimensional case, an octtree is the three-dimensional case of the tpntree.

mod errors;
mod iterators;
pub mod tpntree;
pub mod tpntree_dynamic;

pub use errors::TpnTreeError;

/// [`Coordinates`] is required for a type to be used inside a [`tpntree::SpatialTree`].
///
/// Be sure to return a slice of a length equal to const generic type parameter N.
pub trait Coordinates<const N: usize> {
    fn coordinates(&self) -> &[f64];
}

impl<const N: usize> Coordinates<N> for [f64; N] {
    /// Blanket implementation for arrays of length N.
    fn coordinates(&self) -> &[f64] {
        self
    }
}

impl<const N: usize> Coordinates<N> for Vec<f64> {
    /// Blanket implementation for vectors.
    ///
    /// Panics if the length of the vec is different from N.
    fn coordinates(&self) -> &[f64] {
        assert_eq!(
            self.len(),
            N,
            "Expected vec of length {}, got vec of length {}.",
            N,
            self.len()
        );
        self
    }
}
