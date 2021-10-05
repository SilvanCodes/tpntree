use crate::{impl_breadth_first_iterator, impl_depth_first_iterator};

impl_breadth_first_iterator!(N);
impl_depth_first_iterator!(N);

#[cfg(test)]
mod tests {
    use crate::tpntree::TpnTree;

    #[test]
    fn iterate_depth_first() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        tree.data = Some(1.0);
        assert!(tree.divide().is_ok());

        tree.get_child_mut(3).and_then::<(), _>(|child| {
            child.data = Some(2.0);
            assert!(child.divide().is_ok());
            child.get_child_mut(3).and_then::<(), _>(|childchild| {
                childchild.data = Some(3.0);
                None
            });
            None
        });

        let mut iter = tree.iter_depth_first();

        assert_eq!(iter.next().and_then(|t| t.data()), Some(&1.0));
        assert_eq!(iter.next().and_then(|t| t.data()), Some(&2.0));
        assert_eq!(iter.next().and_then(|t| t.data()), Some(&3.0));
    }

    #[test]
    fn iterate_breadth_first() {
        let mut tree = TpnTree::<f64, 2>::root(1.0);

        tree.data = Some(1.0);
        assert!(tree.divide().is_ok());

        tree.get_child_mut(0).and_then::<(), _>(|child| {
            child.data_mut().insert(2.0);
            assert!(child.divide().is_ok());
            None
        });
        tree.get_child_mut(1).and_then::<(), _>(|child| {
            child.data_mut().insert(3.0);
            assert!(child.divide().is_ok());
            None
        });

        let mut iter = tree.iter_breadth_first();

        assert_eq!(iter.next().and_then(|t| t.data()), Some(&1.0));
        assert_eq!(iter.next().and_then(|t| t.data()), Some(&2.0));
        assert_eq!(iter.next().and_then(|t| t.data()), Some(&3.0));
    }
}
