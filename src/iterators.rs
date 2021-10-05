#[macro_export]
macro_rules! get_tree_type {
    ( $n:ident ) => {
        crate::tpntree::TpnTree<T, $n >
    };
    ( ) => {
        crate::tpntree_dynamic::TpnTree<T>
    };
}

#[macro_export]
macro_rules! impl_depth_first_iterator {
    ( $( $n:ident )? ) => {
            impl<T $(,
                const $n: usize
            )?> $crate::get_tree_type!( $( $n )?) {
                /// Iterate the tree depth first, starting with the root.
                pub fn iter_depth_first(&self) -> DepthFirstIterator<T $(,
                $n
            )?> {
                    DepthFirstIterator::new(self)
                }
            }

            pub struct DepthFirstIterator<'a, T $(,
                const $n: usize
            )?> {
                stack: Vec<&'a $crate::get_tree_type!( $( $n )?)>,
            }

            impl<'a, T $(,
                const $n: usize
            )?> DepthFirstIterator<'a, T $(,
                $n
            )?> {
                fn new(root: &'a $crate::get_tree_type!( $( $n )?)) -> Self {
                    Self { stack: vec![root] }
                }
            }

            impl<'a, T $(,
                const $n: usize
            )?> Iterator for DepthFirstIterator<'a, T $(,
                $n
            )?> {
                type Item = &'a $crate::get_tree_type!( $( $n )?);

                fn next(&mut self) -> Option<Self::Item> {
                    self.stack.pop().map(|tree| {
                        for child in tree.iter_children() {
                            self.stack.push(child);
                        }
                        tree
                    })
                }
        }
    };
}

#[macro_export]
macro_rules! impl_breadth_first_iterator {
    ( $( $n:ident )? ) => {
            impl<T $(,
                const $n: usize
            )?> $crate::get_tree_type!( $( $n )?) {
                /// Iterate the tree breadth first, starting with the root.
                pub fn iter_breadth_first(&self) -> BreadthFirstIterator<T $(,
                $n
            )?> {
                    BreadthFirstIterator::new(self)
                }
            }

            pub struct BreadthFirstIterator<'a, T $(,
                const $n: usize
            )?> {
                queue: std::collections::VecDeque<&'a $crate::get_tree_type!( $( $n )?)>,
            }

            impl<'a, T $(,
                const $n: usize
            )?> BreadthFirstIterator<'a, T $(,
                $n
            )?> {
                fn new(root: &'a $crate::get_tree_type!( $( $n )?)) -> Self {
                    Self {
                        queue: vec![root].into_iter().collect(),
                    }
                }
            }

            impl<'a, T $(,
                const $n: usize
            )?> Iterator for BreadthFirstIterator<'a, T $(,
                $n
            )?> {
                type Item = &'a $crate::get_tree_type!( $( $n )?);

                fn next(&mut self) -> Option<Self::Item> {
                    self.queue.pop_front().map(|tree| {
                        for child in tree.iter_children() {
                            self.queue.push_back(child);
                        }
                        tree
                    })
                }
            }
    };
}
