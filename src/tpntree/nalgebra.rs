use nalgebra::DVector;

use super::TpnTree;

#[cfg(feature = "nalg")]
impl<const N: usize> TpnTree<f64, N> {
    /// Calculate the variance of TpnTrees with f64 data.
    pub fn variance(&self) -> f64 {
        if let Some(children) = &self.children {
            DVector::from_iterator(
                children.len(),
                children.iter().map(|c| c.data.unwrap_or(0.0)),
            )
            .variance()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tpntree::TpnTree;

    #[test]
    fn calculate_variance_alone() {
        let tree = TpnTree::<f64, 2>::root(1.0);

        assert_eq!(tree.variance(), 0.0);
    }

    #[test]
    fn calculate_variance_with_children() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        tree.divide();

        if let Some(children) = tree.children.as_deref_mut() {
            for (i, c) in children.iter_mut().enumerate() {
                c.data = Some(i as f64)
            }
        }

        // population variance of 0,1,2,3
        assert_eq!(tree.variance(), 1.25);
    }
}
