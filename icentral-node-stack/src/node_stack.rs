crate::ix!();

#[derive(Clone,Debug)]
pub struct NodeIdStack {
    name: String,
    data: Vec<NodeId>,
}

impl NodeIdStack {

    pub fn new(first: NodeId, name: &str) -> Self {

        debug!("creating new NodeIdStack named {} from initial node {}", name, first);

        let mut stack = NodeIdStack::empty(name);

        stack.push(first);

        stack
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl CreateNamedEmpty for NodeIdStack {

    fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            data: vec![],
        }
    }
}

impl Clear for NodeIdStack {

    fn clear(&mut self) {

        debug!("clearing NodeIdStack {} of len: {}", self.name, self.len());

        self.data.clear();
    }
}

impl NodeAtIndex for NodeIdStack {

    fn node_at_index(&self, idx: usize) -> NodeId {
        self.data[idx]
    }
}

impl SetNodeAtIndex for NodeIdStack {

    fn set_node_at_index(&mut self, idx: usize, n: NodeId) {

        debug!("setting node {} at idx {} of NodeIdStack {}", n, idx, self.name);

        self.data[idx] = n;
    }
}

impl Push for NodeIdStack {

    type Item = NodeId;

    fn push(&mut self, n: Self::Item) {

        debug!("pushing node {} into NodeIdStack {}", n, self.name);

        self.data.push(n);
    }
}

impl Pop for NodeIdStack {

    type Item = NodeId;

    fn pop(&mut self) -> Option<Self::Item> {

        let n = self.data.pop();

        debug!("popped node {:?} from NodeIdStack, {}", n, self.name);

        n
    }
}
