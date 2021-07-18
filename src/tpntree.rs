mod iterators;
mod nalgebra;

use bitvec::bitvec;

#[derive(Debug)]
pub struct TpnTree<D, const N: usize> {
    /// Coordinates of the N-dim cube center.
    coordinates: [f64; N],
    /// Length from center of N-dim cube to its edges.
    span: f64,
    /// Height in tree.
    level: usize,
    /// There are 2^N children, one times two per axis.
    children: Option<Vec<Self>>,
    /// Any potential data the tree might hold.
    data: Option<D>,
}

impl<D, const N: usize> TpnTree<D, N> {
    /// Creates a new TpnTree.
    pub fn new(coordinates: [f64; N], span: f64, level: usize) -> Self {
        Self {
            coordinates,
            span,
            level,
            children: None,
            data: None,
        }
    }

    /// Creates a new TpnTree at the center at level zero.
    pub fn root(span: f64) -> Self {
        Self::new([0.0; N], span, 0)
    }

    /// Divides self along each axis and fills self.children.
    /// Returns true if division happened, returns false when already divided.
    pub fn divide(&mut self) -> bool {
        if self.children.is_none() {
            let mut children = Vec::<Self>::new();
            let mut pattern = bitvec![0; self.coordinates.len()];
            let mut do_break;

            loop {
                do_break = pattern.all();
                let mut coordinates = self.coordinates;
                // generate sign pattern from bits
                for i in 0..self.coordinates.len() {
                    coordinates[i] += self.span / 2.0 - self.span * pattern[i] as usize as f64;
                }
                children.push(Self::new(coordinates, self.span / 2.0, self.level + 1));

                let mut carry = pattern.to_bitvec();
                carry.set_all(false);
                let mut one = carry.to_bitvec();
                one.set(0, true);

                // "add one" to the pattern
                loop {
                    carry = pattern.clone() & one.clone();
                    pattern ^= one;
                    one = carry.clone();
                    one.push(false);
                    one.shift_right(1);
                    one.pop();
                    if one.not_any() {
                        break;
                    }
                }

                if do_break {
                    break;
                }
            }
            self.children = Some(children);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::TpnTree;

    #[test]
    pub fn divide_into_subregions_dim_1() {
        let mut tree = TpnTree::<(), 1>::root(2.0);

        assert!(tree.divide());
        assert_eq!(tree.children.as_ref().map(|c| c.len()).unwrap(), 2);

        assert_eq!(
            tree.children.as_ref().map(|x| x[0].coordinates),
            Some([1.0])
        );
        assert_eq!(
            tree.children.as_ref().map(|x| x[1].coordinates),
            Some([-1.0])
        );

        assert!(!tree.divide());
    }

    #[test]
    pub fn divide_into_subregions_dim_2() {
        let mut tree = TpnTree::<(), 2>::root(1.0);

        assert!(tree.divide());
        assert_eq!(tree.children.as_ref().map(|c| c.len()).unwrap(), 4);

        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, -0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, -0.5]))
            .unwrap());

        assert!(!tree.divide());
    }

    #[test]
    pub fn divide_into_subregions_dim_3() {
        let mut tree = TpnTree::<(), 3>::root(1.0);

        assert!(tree.divide());
        assert_eq!(tree.children.as_ref().map(|c| c.len()).unwrap(), 8);

        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, 0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, 0.5, -0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, -0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [0.5, -0.5, -0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, 0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, 0.5, -0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, -0.5, 0.5]))
            .unwrap());
        assert!(tree
            .children
            .as_ref()
            .map(|c| c.iter().any(|x| x.coordinates == [-0.5, -0.5, -0.5]))
            .unwrap());

        assert!(!tree.divide());
    }
}
