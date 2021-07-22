mod iterators;
mod nalgebra;

use bitvec::bitvec;

#[derive(Debug, Clone)]
pub struct TpnTree<D, const N: usize> {
    /// Coordinates of the N-dimensional hyperrectangle center.
    coordinates: [f64; N],
    /// Length of the normals from center of N-dimensional hyperrectangle to its faces.
    span: [f64; N],
    /// Height in tree.
    level: usize,
    /// There are zero or 2^N children, one times two per axis.
    children: Vec<Self>,
    /// Any potential data the tree might hold.
    data: Option<D>,
}

impl<D, const N: usize> TpnTree<D, N> {
    /// Creates a new TpnTree.
    ///
    /// Use this function if you need explicit control over the initial coordinates or a span with various length along different axis.
    /// Usually the [`TpnTree::root`] function should suffice and is simpler to use.
    ///
    /// # Examples
    ///
    /// Here we create a one dimensional tree, i.e. with one axis sitting on the center `[0.0]` with a span of 1.0 in each direction `[1.0]`.
    /// This is equal to a line segment spanning from -1.0 to 1.0 with its midpoint at 0.0.
    /// The `level` of the root is usually zero.
    /// ```
    /// # use tpntree::TpnTree;
    ///
    /// let root = TpnTree::<(), 1>::new([0.0], [1.0], 0);
    /// ```
    ///
    /// Now lets create a two dimensional tree.
    /// The tree root corresponds to a square with its center sitting at the coordinates (1.0/1.0) with edges one edge being 4.0 the other being 1.0 units long.
    /// The edges equate to twice the span as it originates from the midpoint.
    /// ```
    /// # use tpntree::TpnTree;
    /// let root = TpnTree::<(), 2>::new([1.0, 1.0], [2.0, 0.5], 0);
    /// ```
    pub fn new(coordinates: [f64; N], span: [f64; N], level: usize) -> Self {
        Self {
            coordinates,
            span,
            level,
            children: Vec::new(),
            data: None,
        }
    }

    /// Creates a new TpnTree with equal span in all dimension at the center of the space at level zero.
    ///
    /// Use this function if you want your TpnTree to represenst a centered N-dimensional hypercube.
    ///
    /// # Examples
    ///
    /// Here we create a three dimensional TpnTree with a span of 1.0 in ervery dimension.
    /// That is equal to a cube with edges of length 2.0.
    /// ```
    /// # use tpntree::TpnTree;
    /// let root = TpnTree::<(), 3>::root(1.0);
    /// ```
    pub fn root(span: f64) -> Self {
        Self::new([0.0; N], [span; N], 0)
    }

    /// Divides the TpnTree into subregions creating new TpnTrees as children.
    ///
    /// Returns true if division happened, returns false when already divided.
    ///
    /// Each created child has its center moved by half the parents span up or down along the axis.
    /// Every child is equal to one unique combination of such half span moves.
    ///
    /// # Examples
    ///
    /// Dividing in the 2D case is creating four smaller squares.
    ///
    /// +---+    +-+-+
    /// |   | => +-+-+
    /// +---+    +-+-+
    /// ```
    /// # use tpntree::TpnTree;
    /// let mut root = TpnTree::<(), 2>::root(1.0);
    ///
    /// assert!(root.divide());
    /// assert_eq!(root.child_count(), 4);
    /// ```
    pub fn divide(&mut self) -> bool {
        if self.children.is_empty() {
            let mut children = Vec::<Self>::new();
            let mut pattern = bitvec![0; self.coordinates.len()];

            // iterate for 2^N to generate all children
            for _ in 0..2usize.pow(self.coordinates.len() as u32) {
                let mut coordinates = self.coordinates;
                let mut span = self.span;
                // generate sign pattern from bits
                for i in 0..self.coordinates.len() {
                    span[i] = self.span[i] / 2.0;
                    coordinates[i] += span[i] - self.span[i] * pattern[i] as usize as f64;
                }

                children.push(Self::new(coordinates, span, self.level + 1));

                let mut carry = pattern.clone();
                carry.set_all(false);
                let mut one = carry.clone();
                one.set(0, true);

                // "add one" to the pattern, loop to accomodate for carry
                while one.any() {
                    carry = pattern.clone() & one.clone();
                    pattern ^= one;
                    one = carry.clone();
                    // push so we can shift
                    one.push(false);
                    one.shift_right(1);
                    // pop to have an overflowing shift
                    one.pop();
                }
            }
            self.children = children;
            true
        } else {
            false
        }
    }

    /// Get a reference to a child TpnTree if it exists.
    pub fn get_child(&self, index: usize) -> Option<&Self> {
        self.children.get(index)
    }
    /// Get a mutable reference to a child TpnTree if it exists.
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Self> {
        self.children.get_mut(index)
    }

    /// Returns the count of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Iterates all direct children by reference.
    pub fn iter_children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }

    /// Iterates all direct children by mutable reference.
    pub fn iter_children_mut(&mut self) -> impl Iterator<Item = &mut Self> {
        self.children.iter_mut()
    }

    /// Returns the coordinates of the center of the TpnTree.
    pub fn coordinates(&self) -> [f64; N] {
        self.coordinates
    }

    /// Returns the span of the TpnTree.
    pub fn span(&self) -> [f64; N] {
        self.span
    }

    /// Returns the data by reference of the TpnTree.
    pub fn data(&self) -> &Option<D> {
        &self.data
    }

    /// Returns the data by mutable reference of the TpnTree.
    pub fn data_mut(&mut self) -> &mut Option<D> {
        &mut self.data
    }

    /// Returns the level of the TpnTree.
    pub fn level(&self) -> usize {
        self.level
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::TpnTree;

    #[test]
    pub fn divide_into_subregions_dim_1() {
        let mut root = TpnTree::<(), 1>::root(2.0);

        assert!(root.divide());
        assert_eq!(root.child_count(), 2);

        assert_eq!(root.get_child(0).map(|c| c.coordinates()), Some([1.0]));
        assert_eq!(root.get_child(1).map(|c| c.coordinates()), Some([-1.0]));

        assert!(!root.divide());
    }

    #[test]
    pub fn divide_into_subregions_dim_2() {
        let mut root = TpnTree::<(), 2>::root(1.0);

        assert!(root.divide());
        assert_eq!(root.child_count(), 4);

        assert!(root.iter_children().any(|c| c.coordinates() == [0.5, 0.5]));
        assert!(root.iter_children().any(|c| c.coordinates() == [0.5, -0.5]));
        assert!(root.iter_children().any(|c| c.coordinates() == [-0.5, 0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [-0.5, -0.5]));

        assert!(!root.divide());
    }

    #[test]
    pub fn divide_into_subregions_dim_3() {
        let mut root = TpnTree::<(), 3>::root(1.0);

        assert!(root.divide());
        assert_eq!(root.child_count(), 8);

        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [0.5, 0.5, 0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [0.5, 0.5, -0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [0.5, -0.5, 0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [0.5, -0.5, -0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [-0.5, 0.5, 0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [-0.5, 0.5, -0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [-0.5, -0.5, 0.5]));
        assert!(root
            .iter_children()
            .any(|c| c.coordinates() == [-0.5, -0.5, -0.5]));

        assert!(!root.divide());
    }
}
