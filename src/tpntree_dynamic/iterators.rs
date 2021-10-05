use crate::{impl_breadth_first_iterator, impl_depth_first_iterator};

impl_breadth_first_iterator!();
impl_depth_first_iterator!();

#[cfg(test)]
mod tests {
    use crate::tpntree_dynamic::TpnTree;

    #[test]
    fn iterate_depth_first() {
        let mut tree = TpnTree::<f64>::root(1.0, 2);

        tree.data = Some(1.0);
        tree.divide();

        tree.get_child_mut(3).and_then::<(), _>(|child| {
            child.data = Some(2.0);
            child.divide();
            child.get_child_mut(3).and_then::<(), _>(|childchild| {
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
        let mut tree = TpnTree::<f64>::root(1.0, 2);

        tree.data = Some(1.0);
        tree.divide();

        tree.get_child_mut(0).and_then::<(), _>(|child| {
            *child.data_mut() = Some(2.0);
            child.divide();
            None
        });
        tree.get_child_mut(1).and_then::<(), _>(|child| {
            *child.data_mut() = Some(3.0);
            child.divide();
            None
        });

        let mut iter = tree.iter_breadth_first();

        assert_eq!(iter.next().and_then(|t| t.data().as_ref()), Some(&1.0));
        assert_eq!(iter.next().and_then(|t| t.data().as_ref()), Some(&2.0));
        assert_eq!(iter.next().and_then(|t| t.data().as_ref()), Some(&3.0));
    }
}
