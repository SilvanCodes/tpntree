//! This crate conatins an N-dimensional generalization of a quad-tree called **T**wo-**p**ower-__n__-tree or tpn-tree,
//! as there exist 2^N children per node, where N is the number of dimensions.

mod tpntree;

pub use tpntree::TpnTree;
