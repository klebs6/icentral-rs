crate::ix!();

#[derive(Debug)]
pub struct LabelMap {

    name:   String,

    /**
      | maps original node id to id in the the
      | subgraph maps internal labels to original
      |
      */
    in_out: Vec<NodeId>,

    /**
      | original labels to internal
      |
      */
    out_in: HashMap<NodeId,NodeId>,
}

impl Clear for LabelMap {

    fn clear(&mut self) {

        debug!("clearing LabelMap of len: {}", self.len());

        self.in_out.clear();
        self.out_in.clear();
    }
}

impl CreateNamedEmpty for LabelMap {

    fn empty(name: &str) -> Self {

        debug!("creating new empty LabelMap named {}", name);

        Self {
            name:   name.to_owned(),
            in_out: vec![],
            out_in: HashMap::new(),
        }
    }
}

impl LabelMap {

    pub fn new_with_len(len: usize, name: &str) -> Self {

        debug!("creating new LabelMap named {} from graph of len: {}", name, len);

        let in_out = vec![NodeId::bad(); len];
        let out_in = HashMap::new();

        let mut label_map = Self {
            name: name.to_owned(),
            in_out,
            out_in
        };

        // subgraph.nodeid_range()
        for node in NodeIdRange::new(0,len) {
            label_map.insert_outin(node,node);
        }

        label_map
    }

    pub fn len(&self) -> usize {

        assert!(self.in_out.len() == self.out_in.len());

        self.in_out.len()
    }

    pub fn inout(&self, node: NodeId) -> NodeId {
        self.in_out[node.val()]
    }

    pub fn outin(&self, node: NodeId) -> NodeId {
        self.out_in[&node]
    }

    pub fn mapped_edge(&self, theirs: &Edge) -> Edge {

        let mut src: NodeId = self.out_in[&theirs.src];
        let mut dst: NodeId = self.out_in[&theirs.dst];

        let mine = Edge::new(src, dst);

        debug!("remapping edge {} into map: {}", theirs, mine);

        mine
    }

    pub fn projected_edge(&self, mine: &Edge) -> Edge {

        let their_src = self.inout(mine.src);
        let their_dst = self.inout(mine.dst);

        let theirs = Edge::new(their_src, their_dst);

        debug!("projecting edge {} into their space: {}", mine, theirs);

        theirs
    }

    pub fn insert_outin(&mut self, src: NodeId, dst: NodeId) {
        self.out_in.insert(src,dst);
    }

    pub fn resize_inout(&mut self, len: usize, default: NodeId) {

        debug!("resizing LabelMap {} to len {}", self.name, len);

        self.in_out.resize(len,default);
    }

    pub fn new_from_graph_ref<GH: NumNodes + MappedNodes>(
        gh:   &GH, 
        name: &str

    ) -> Self {

        debug!("creating new LabelMap named {} from GraphHash of len: {}", name, gh.num_nodes());

        let mut label_map = LabelMap {
            name: name.to_owned(),
            in_out: vec![],
            out_in: HashMap::new(),
        };

        let graph_len = gh.num_nodes();

        label_map.in_out.resize(graph_len,NodeId::zero());

        for (idx,id) in gh.mapped_nodes().iter().enumerate() {

            label_map.in_out[idx] = *id;

            label_map.out_in.insert(
                *id,
                nodeid![idx]
            );
        }

        info!("LabelMap created\n{:#?}", label_map);

        label_map 
    }
}
