//! This crate contains an N-dimensional generalization of a region quadtree called **T**wo-**p**ower-__n__-tree or tpntree,
//! as there exist 2^N children per node, where N is the number of dimensions.
//! A quadtree is the two-dimensional case, an octtree is the three-dimensional case of the tpntree.

pub mod tpntree;
pub mod tpntree_dynamic;
