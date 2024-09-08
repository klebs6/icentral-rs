crate::ix!();


//---------------------------------------
pub struct NodeIdEnumerate<I: Iterator>(pub std::iter::Enumerate<I>);

impl<I: Iterator> Iterator for NodeIdEnumerate<I> {

    type Item = (NodeId, <I as Iterator>::Item);

    fn next(&mut self) -> Option<(NodeId, <I as Iterator>::Item)> {

        let (i,a) = self.0.next()?;

        Some((nodeid!(i), a))
    }
}

//---------------------------------------
pub struct IndexedIterator<'m, T>(NodeIdEnumerate<std::slice::Iter<'m,T>>);

impl<'m,T> IndexedIterator<'m,T> {

    pub fn new(x: std::slice::Iter<'m,T>) -> Self {
        IndexedIterator(NodeIdEnumerate(x.enumerate()))
    }
}

impl<'m,T> Iterator for IndexedIterator<'m,T> {

    type Item = (NodeId, &'m T);

    fn next(&mut self) -> Option<(NodeId, &'m T)> {
        self.0.next()
    }
}

//---------------------------------------
pub struct MappedIterator<'m, T> {
    inner: std::collections::hash_map::Iter<'m, NodeId, T>,
}

impl<'m,T> MappedIterator<'m,T> {

    pub fn new(x: std::collections::hash_map::Iter<'m,NodeId,T>) -> Self {
        MappedIterator {
            inner: x,
        }
    }
}

impl<'m,T> Iterator for MappedIterator<'m,T> {

    type Item = (NodeId, &'m T);

    fn next(&mut self) -> Option<(NodeId, &'m T)> {

        let (i,a) = self.inner.next()?;

        Some((i.clone(),a))
    }
}

//---------------------------------------
pub enum MaybeIndexedMapIterator<'m, T> {
    Indexed(IndexedIterator<'m,T>),
    Mapped(MappedIterator<'m,T>),
}

impl<'m, T: Clone> MaybeIndexedMapIterator<'m, T> {

    pub fn new_indexed(x: std::slice::Iter<'m,T>) -> Self {
        let it = IndexedIterator::new(x);
        MaybeIndexedMapIterator::Indexed(it)
    }

    pub fn new_mapped(x: std::collections::hash_map::Iter<'m,NodeId,T>) -> Self {
        let it = MappedIterator::new(x);
        MaybeIndexedMapIterator::Mapped(it)
    }
}

impl<'m, T: Clone> Iterator for MaybeIndexedMapIterator<'m, T> {

    type Item = (NodeId, &'m T);

    fn next(&mut self) -> Option<(NodeId, &'m T)> {
        match self {
            MaybeIndexedMapIterator::Indexed(it) => {
                it.next()
            }
            MaybeIndexedMapIterator::Mapped(it) => {
                it.next()
            }
        }
    }
}
