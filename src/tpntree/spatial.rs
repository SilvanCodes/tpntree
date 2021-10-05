use std::iter::once;

use super::TpnTree;
use crate::{errors::TpnTreeError, Coordinates};

/// A helper type to work with spatial data bins.
pub type SpatialTree<T, const N: usize> = TpnTree<Vec<T>, N>;

/// A helper type to specify a tree working with 3D data.
pub type Tree3D<T = [f64; 3]> = SpatialTree<T, 3>;

impl<T: Coordinates<N>, const N: usize> SpatialTree<T, N> {
    /// Checks if the tree spans over the coordinates of the provided data.
    ///
    /// ```
    /// # use tpntree::tpntree::Tree3D;
    /// # use tpntree::TpnTreeError;
    ///  let tree = Tree3D::root(1.0);
    ///
    ///  assert_eq!(tree.spans(&[0.5,0.5,0.5]), true);
    ///  assert_eq!(tree.spans(&[1.5,0.5,0.5]), false);
    /// ```
    pub fn spans(&self, data: &T) -> bool {
        let data_coordinates = data.coordinates();

        // checks if tpn tree contains the data coordinates
        // children overlap on their edges
        self.coordinates
            .iter()
            .enumerate()
            .all(|(dimension, &coordinate)| {
                data_coordinates[dimension] <= coordinate + self.span()[dimension]
                    && data_coordinates[dimension] >= coordinate - self.span()[dimension]
            })
    }

    /// Inserts data in the tree with its center closest to the data given the constrains of the `division_condition`.
    ///
    /// The `division condition` determines when a tree divides and inserts its data into its children.
    /// Errors if the tree does not span the data.
    ///
    /// ```
    /// # use tpntree::tpntree::Tree3D;
    ///  let mut tree = Tree3D::root(1.0);
    ///  
    /// assert!(tree.insert_by_coordinates([1.0, 0.0, -1.0], &|_| false).is_ok());
    /// ```
    pub fn insert_by_coordinates(
        &mut self,
        data: T,
        division_condition: &dyn Fn(&Self) -> bool,
    ) -> Result<(), TpnTreeError> {
        // if the root tree does not span over the data, it can not be inserted
        if self.is_root() && !self.spans(&data) {
            return Err(TpnTreeError::DoesNotSpan);
        }

        if self.is_leaf() {
            if division_condition(self) {
                self.divide()?;

                for data in self
                    .data
                    .take()
                    .unwrap_or_default()
                    .into_iter()
                    .chain(once(data))
                {
                    self.insert_into_children(data, division_condition)?
                }
                Ok(())
            } else {
                self.data.get_or_insert(Vec::new()).push(data);
                Ok(())
            }
        } else {
            self.insert_into_children(data, division_condition)
        }
    }

    fn insert_into_children(
        &mut self,
        data: T,
        division_condition: &dyn Fn(&Self) -> bool,
    ) -> Result<(), TpnTreeError> {
        self.children
            .iter_mut()
            // we can savely unwrap here as dimn=ensions are checked before
            .find(|child| child.spans(&data))
            .map(|child| child.insert_by_coordinates(data, division_condition))
            .unwrap()
    }

    /// Return the tree closest to the given data coordinates.
    ///
    /// Errors if the tree does not span the data.
    ///
    /// ```
    /// # use tpntree::tpntree::Tree3D;
    ///  let mut tree = Tree3D::root(1.0);
    ///  
    /// tree.insert_by_coordinates([1.0, 0.0, -1.0], &|_| false).expect("Couldn't insert.");
    /// assert!(tree
    ///   .find_by_coordinates(&[0.0, 0.0, 0.0])
    ///   .ok()
    ///   .and_then(|tree| tree.data().map(|vec| vec.contains(&[1.0, 0.0, -1.0])))
    ///   .unwrap());
    /// ```
    pub fn find_by_coordinates(&self, data: &T) -> Result<&Self, TpnTreeError> {
        if self.is_root() && !self.spans(data) {
            return Err(TpnTreeError::DoesNotSpan);
        }

        for child in &self.children {
            if child.spans(data) {
                return child.find_by_coordinates(data);
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::tpntree::Tree3D;

    #[test]
    fn tree_contains_coordinates() {
        let tree = Tree3D::root(1.0);

        let data_inside = [1.0, 1.0, 1.0];

        assert!(tree.spans(&data_inside));
    }

    #[test]
    fn tree_does_not_contain_coordinates() {
        let tree = Tree3D::root(1.0);

        let data_outside = [1.0, 1.5, 1.0];

        assert!(!tree.spans(&data_outside));
    }

    #[test]
    fn insert_into_root() {
        let mut tree = Tree3D::root(1.0);

        let data = [1.0, 1.0, 1.0];

        assert!(tree.insert_by_coordinates(data, &|_| false).is_ok());
        assert!(tree
            .find_by_coordinates(&[0.0, 0.0, 0.0])
            .map(|tree| tree.data().as_ref().map(|vec| vec.contains(&data)))
            .unwrap()
            .unwrap());
        assert!(tree.is_leaf());
    }

    #[test]
    fn insert_and_split() {
        let mut tree = Tree3D::root(1.0);

        let data_one = [1.0, 1.0, 1.0];
        let data_two = [-1.0, -1.0, -1.0];

        let division_condition = |tree: &Tree3D| tree.data().is_some();

        assert!(tree
            .insert_by_coordinates(data_one, &division_condition)
            .is_ok());
        assert!(tree
            .insert_by_coordinates(data_two, &division_condition)
            .is_ok());
        assert!(tree
            .find_by_coordinates(&[0.5, 0.5, 0.5])
            .ok()
            .and_then(|tree| tree.data().map(|vec| vec.contains(&data_one)))
            .unwrap());
        assert!(tree
            .find_by_coordinates(&[-0.5, -0.5, -0.5])
            .ok()
            .and_then(|tree| tree.data().map(|vec| vec.contains(&data_two)))
            .unwrap());
        assert!(tree.data().is_none());
        assert!(tree.child_count() == 8);
    }
}
