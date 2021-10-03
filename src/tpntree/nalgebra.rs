use nalgebra::DVector;

use super::TpnTree;

#[cfg(feature = "nalgebra")]
impl<const N: usize> TpnTree<f64, N> {
    /// Calculate the variance of TpnTrees with f64 data.
    pub fn variance(&self) -> f64 {
        DVector::from_iterator(
            self.children.len(),
            self.children.iter().map(|c| c.data.unwrap_or(0.0)),
        )
        .variance()
    }
}

#[cfg(test)]
mod tests {
    use crate::tpntree::TpnTree;

    #[test]
    fn calculate_variance_alone() {
        let tree = TpnTree::<f64, 2>::root(1.0);

        assert!(tree.variance() < f64::EPSILON);
    }

    #[test]
    fn calculate_variance_with_children() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        assert!(tree.divide().is_ok());

        for (i, c) in tree.iter_children_mut().enumerate() {
            c.data = Some(i as f64)
        }

        // population variance of 0,1,2,3
        assert!((tree.variance() - 1.25).abs() < f64::EPSILON);
    }
}
