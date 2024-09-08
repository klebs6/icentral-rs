crate::ix!();

pub trait CreateEmptyIndexed {

    fn empty_indexed(name: &str) -> Self;
}

pub trait CreateEmptyMapped {

    fn empty_mapped(name: &str) -> Self;
}

error_tree!{

    pub enum InitMapError {
        TriedReinitMappedObject,
    }
}

#[derive(Clone)]
pub enum MaybeIndexedMap<T: Clone> {
    Indexed(Vec<T>),
    Mapped(HashMap<NodeId,T>),
}

impl<T: Clone + Debug> fmt::Debug for MaybeIndexedMap<T> {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            MaybeIndexedMap::Indexed(v) => {
                fmt.debug_map()
                    .entries(v.iter().enumerate())
                    .finish()
            }
            MaybeIndexedMap::Mapped(m) => {
                fmt.debug_map()
                    .entries(m.iter())
                    .finish()
            }
        }
    }
}

impl<T: Clone> GetNodeIdRange for MaybeIndexedMap<T> {

    fn nodeid_range(&self)
    -> Vec<NodeId>
    {
        let keys = self.keys();

        let len = keys.len();

        keys[0..len].into_iter().cloned().collect()
    }
}

impl<T: Clone> GetLimitedNodeIdRange for MaybeIndexedMap<T> {

    fn limited_nodeid_range(&self, cap: Option<usize>) 
    -> Vec<NodeId>
    {
        match cap {
            None => {
                self.nodeid_range()
            }
            Some(n) => {

                let keys = self.keys();

                let top  = min(n,keys.len());

                keys[0..top].into_iter().cloned().collect()
            }
        }
    }
}

impl<T: Clone> MaybeIndexedMap<T> {

    pub fn set(&mut self, node: NodeId, rhs: T) {
        match self {
            MaybeIndexedMap::Indexed(v) => {

                if node.val() < v.len() {
                    v[node.val()] = rhs;
                } else {
                    panic!("attempt to set OOB for Indexed Map. NodeId: {}, len: {}", node, v.len());
                }
            }
            MaybeIndexedMap::Mapped(m) => {
                m.insert(node,rhs);
            }
        }
    }

    pub fn empty_indexed() -> Self {
        Self::Indexed(vec![])
    }

    pub fn empty_mapped() -> Self {
        Self::Mapped(HashMap::new())
    }

    pub fn new(len: usize, val: T) -> Self {
        Self::Indexed(vec![val;len])
    }

    pub fn iter(&self) -> MaybeIndexedMapIterator<'_,T> {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                MaybeIndexedMapIterator::new_indexed(v.iter())
            }
            MaybeIndexedMap::Mapped(m) => {
                MaybeIndexedMapIterator::new_mapped(m.iter())
            }
        }
    }

    pub fn extend(&mut self, other: &MaybeIndexedMap<T>) {
        match (self,other) {
            (MaybeIndexedMap::Mapped(ref mut m1), MaybeIndexedMap::Mapped(ref m2)) => {
                m1.extend(m2.clone());
            }
            _ => {
                panic!("this should probably not be supported because it implies we may be mixing apples with oranges");
            }
        }
    }

    pub fn contains(&self, node: NodeId) -> bool {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.len() > node.val()
            }
            MaybeIndexedMap::Mapped(m) => {
                m.contains_key(&node)
            }
        }
    }

    pub fn remove(&mut self, node: NodeId) {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.remove(node.val());
            }
            MaybeIndexedMap::Mapped(m) => {
                m.remove(&node);
            }
        }
    }

    pub fn fill(&mut self, val: T) {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.fill(val);
            }
            MaybeIndexedMap::Mapped(m) => {
                m.values_mut().for_each(|v| *v = val.clone());
            }
        }
    }

    pub fn reserve(&mut self, len: usize) {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.reserve(len);
            }
            MaybeIndexedMap::Mapped(m) => {
                m.reserve(len);
            }
        }
    }

    pub fn refill(&mut self, len: usize, val: T) {

        self.clear();

        match self {
            MaybeIndexedMap::Indexed(v) => {

                trace!("refilling Indexed Map");
                v.reserve(1);

                v.resize(len,val);
            }
            MaybeIndexedMap::Mapped(m) => {

                let additional = len - m.len();

                trace!(
                    "refilling KV Mapped Map, len: {}. m.len(): {}, additional: {}", 
                    len, 
                    m.len(),
                    additional
                );

                m.reserve(additional);

                m.values_mut().for_each(|v| *v = val.clone());
            }
        }
    }

    pub fn clear(&mut self) {

        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.clear();
            }
            MaybeIndexedMap::Mapped(m) => {
                m.clear();
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                v.len()
            }
            MaybeIndexedMap::Mapped(m) => {
                m.len()
            }
        }
    }

    pub fn keys(&self) -> Vec<NodeId> 
    {
        match self {
            MaybeIndexedMap::Indexed(v) => (0..v.len()).map(|x| nodeid![x]).collect(),
            MaybeIndexedMap::Mapped(m)  => m.keys().cloned().collect(),
        }
    }
}

impl<T: Clone> Default for MaybeIndexedMap<T> {

    fn default() -> Self {
        Self::Mapped(HashMap::new())
    }
}

impl<T: Clone> Index<NodeId> for MaybeIndexedMap<T> {

    type Output = T;

    fn index(&self, idx: NodeId) -> &Self::Output {

        match self {
            MaybeIndexedMap::Indexed(v) => {
                &v[idx.val()]
            }
            MaybeIndexedMap::Mapped(m) => {
                &m[&idx]
            }
        }
    }
}

impl<T: Clone + Default> IndexMut<NodeId> for MaybeIndexedMap<T> {

    fn index_mut(&mut self, idx: NodeId) 
    -> &mut Self::Output 
    {
        match self {
            MaybeIndexedMap::Indexed(ref mut v) => {
                &mut v[idx.val()]
            }
            MaybeIndexedMap::Mapped(ref mut m) => {

                //autovivify
                if m.get(&idx).is_none() {
                    m.insert(idx, T::default());
                }

                m.get_mut(&idx).unwrap()
            }
        }
    }
}

impl<T: Clone> Index<Range<NodeId>> for MaybeIndexedMap<T> {

    type Output = [T];

    fn index(&self, range: Range<NodeId>) -> &Self::Output {
        match self {
            MaybeIndexedMap::Indexed(ref v) => {
                &v.as_slice()[range.start.val()..range.end.val()]
            }
            MaybeIndexedMap::Mapped(ref m) => {
                unimplemented!();
            }
        }
    }
}

impl<T: Clone> IndexMut<Range<NodeId>> for MaybeIndexedMap<T> {

    fn index_mut(&mut self, range: Range<NodeId>) -> &mut Self::Output {
        match self {
            MaybeIndexedMap::Indexed(ref mut v) => {
                &mut v.as_mut_slice()[range.start.val()..range.end.val()]
            }
            MaybeIndexedMap::Mapped(ref mut m) => {
                unimplemented!();
            }
        }
    }
}

impl MaybeIndexedMap<f64> {

    pub fn halve(&mut self) {
        match self {
            MaybeIndexedMap::Indexed(v) => {
                for item in v.iter_mut() {
                    *item /= 2.0;
                }
            }
            MaybeIndexedMap::Mapped(m) => {
                for (k,v) in m.iter_mut() {
                    *v /= 2.0;
                }
            }
        }
    }
}
