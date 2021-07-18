use std::collections::VecDeque;

use super::TpnTree;

impl<D, const N: usize> TpnTree<D, N> {
    /// Iterate the tree depth first, starting with the root.
    fn iter_depth_first(&self) -> TpnTreeDepthFirstIterator<D, N> {
        TpnTreeDepthFirstIterator::new(self)
    }

    /// Iterate the tree breadth first, starting with the root.
    fn iter_breadth_first(&self) -> TpnTreeBreadthFirstIterator<D, N> {
        TpnTreeBreadthFirstIterator::new(self)
    }
}

pub struct TpnTreeDepthFirstIterator<'a, D, const N: usize> {
    stack: Vec<&'a TpnTree<D, N>>,
}

impl<'a, D, const N: usize> TpnTreeDepthFirstIterator<'a, D, N> {
    pub fn new(root: &'a TpnTree<D, N>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, D, const N: usize> Iterator for TpnTreeDepthFirstIterator<'a, D, N> {
    type Item = &'a TpnTree<D, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().and_then(|tree| {
            if let Some(children) = &tree.children {
                for child in children {
                    self.stack.push(child);
                }
            }
            Some(tree)
        })
    }
}

pub struct TpnTreeBreadthFirstIterator<'a, D, const N: usize> {
    queue: VecDeque<&'a TpnTree<D, N>>,
}

impl<'a, D, const N: usize> TpnTreeBreadthFirstIterator<'a, D, N> {
    pub fn new(root: &'a TpnTree<D, N>) -> Self {
        Self {
            queue: vec![root].into_iter().collect(),
        }
    }
}

impl<'a, D, const N: usize> Iterator for TpnTreeBreadthFirstIterator<'a, D, N> {
    type Item = &'a TpnTree<D, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().and_then(|tree| {
            if let Some(children) = &tree.children {
                for child in children {
                    self.queue.push_back(child);
                }
            }
            Some(tree)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tpntree::TpnTree;

    #[test]
    fn iterate_depth_first() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        tree.data = Some(1.0);
        tree.divide();

        tree.children
            .as_mut()
            .unwrap()
            .last_mut()
            .and_then::<(), _>(|child| {
                child.data = Some(2.0);
                child.divide();
                child
                    .children
                    .as_mut()
                    .unwrap()
                    .last_mut()
                    .and_then::<(), _>(|childchild| {
                        childchild.data = Some(3.0);
                        None
                    });
                None
            });

        let mut iter = tree.iter_depth_first();

        assert_eq!(iter.next().and_then(|t| t.data), Some(1.0));
        assert_eq!(iter.next().and_then(|t| t.data), Some(2.0));
        assert_eq!(iter.next().and_then(|t| t.data), Some(3.0));
    }

    #[test]
    fn iterate_breadth_first() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        tree.data = Some(1.0);
        tree.divide();

        tree.children
            .as_mut()
            .unwrap()
            .get_mut(0)
            .and_then::<(), _>(|child| {
                child.data = Some(2.0);
                child.divide();
                None
            });
        tree.children
            .as_mut()
            .unwrap()
            .get_mut(1)
            .and_then::<(), _>(|child| {
                child.data = Some(3.0);
                child.divide();
                None
            });

        let mut iter = tree.iter_breadth_first();

        assert_eq!(iter.next().and_then(|t| t.data), Some(1.0));
        assert_eq!(iter.next().and_then(|t| t.data), Some(2.0));
        assert_eq!(iter.next().and_then(|t| t.data), Some(3.0));
    }
}
