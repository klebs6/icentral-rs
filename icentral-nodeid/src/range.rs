crate::ix!();

pub struct NodeIdRange {
    start: usize,
    len:   usize,
}

impl NodeIdRange {

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn range(&self) -> std::ops::Range<NodeId> {
        std::ops::Range {
            start: nodeid![self.start],
            end:   nodeid![self.start + self.len]
        }
    }
}

impl IntoIterator for NodeIdRange {

    type Item     = NodeId;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

        let start = self.start;
        let end   = start + self.len;

        let vec: Vec<NodeId> = (start..end).map(|x| NodeId::from(x)).collect();

        vec.into_iter()
    }
}

impl NodeIdRange {

    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
        }
    }
}
