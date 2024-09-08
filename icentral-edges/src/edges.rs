crate::ix!();

//-------------------------------------------------
#[derive(Clone)]
pub struct Edges {
    name:  String,
    inner: HashSet<Edge>,
}

impl fmt::Debug for Edges {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut entries: Vec<&Edge> = self.inner.iter().collect();

        entries.sort();

        let binding = f.debug_list();

        let mut builder = binding;

        builder.entries(entries.iter());

        builder.finish()
    }
}

impl HasEdge for Edges {

    delegate!{
        to self.inner {
            #[call(contains)]
            fn has_edge(&self, edge: &Edge) -> bool;
        }
    }
}

impl SetName for Edges {

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

impl CreateNamedEmpty for Edges {

    fn empty(name: &str) -> Self {
        Self {
            inner: HashSet::new(),
            name:  name.to_owned(),
        }
    }
}

impl Edges {

    pub fn new_from_graph_ref<G: GetEdges>(g: &G, name: &str) -> Self {

        debug!("creating new Edges named {} from graph with {} edges", name, g.edges().len());

        let mut edges = g.edges().clone();

        edges.set_name(name);

        edges
    }

    delegate!{
        to self.inner {
            pub fn len(&self) -> usize;

            pub fn clear(&mut self);

            #[call(insert)]
            pub fn insert_edge(&mut self, edge: Edge);

            #[call(remove)]
            pub fn remove_edge(&mut self, edge: &Edge);
        }
    }

    pub fn new_remapped<GH: GetEdges + NumNodes>(
        label_map: &LabelMap, 
        gh:        &GH, 
        name:      &str

    ) -> Self {

        debug!(
            "creating new Edges named {} from LabelMap of len {} and GraphHash of len {}", 
            name,
            label_map.len(), 
            gh.num_nodes()
        );

        let mut edges = Self::empty(name);

        for their_edge in gh.edges().iter() {

            let my_edge = label_map.mapped_edge(&their_edge);

            edges.insert_edge(my_edge);
        }

        edges
    }

    pub fn iter(&self) 
    -> std::collections::hash_set::Iter<'_, Edge> 
    {
        self.inner.iter()
    }

    pub fn extend(&mut self, other: &Edges) {
        self.inner.extend(other.inner.clone());
    }

    pub fn connects(&self, src: NodeId, dst: NodeId) -> bool {

        debug!("from Edges, check whether we have an edge connection between {} and {}", src, dst);

        let mut e1: Edge = Edge::new(src,dst);
        let mut e2: Edge = Edge::new(dst,src);

        self.has_edge(&e1) || self.has_edge(&e2)
    }

    pub fn unlink_all(&mut self, src: NodeId, dst: NodeId) {

        debug!("from Edges, unlink_all edges between src {} and dst {}", src, dst);

        let mut e0 = Edge::new(src,dst);
        let mut e1 = Edge::new(dst,src);

        self.remove_edge(&e0);
        self.remove_edge(&e1);
    }

    pub fn edges_to_node(&self, src: NodeId) -> Vec<Edge> {

        let edges_to = self.inner.iter().cloned().filter(|e| e.dst == src).collect();

        debug!("computed edges_to_node {:?} for node: {}", edges_to, src);

        edges_to
    }

    pub fn edges_from_node(&self, src: NodeId) -> Vec<Edge> {

        let edges_from = self.inner.iter().cloned().filter(|e| e.src == src).collect();

        debug!("computed edges_from_node {:?} for node: {}", edges_from, src);

        edges_from
    }

    pub fn new_from_vec(x: Vec<Edge>, name: &str) -> Edges {

        debug!("creating new Edges named {} by collecting Vec<Edge> with {} Edges", name, x.len());

        let set: HashSet<Edge> 
        = x.into_iter().collect();

        Edges {
            inner: set,
            name:  name.to_owned(),
        }
    }

    pub fn new_from_set_ref(x: &HashSet<Edge>, name: &str) -> Self {

        debug!("creating new Edges named {} by cloning HashSet<Edge> with {} Edges", name, x.len());

        Edges {
            inner: x.clone(),
            name:  name.to_owned(),
        }
    }

    pub fn new_from_set(x: HashSet<Edge>, name: &str) -> Self {

        debug!("creating new Edges named {} by moving HashSet<Edge> with {} Edges", name, x.len());

        Edges {
            inner: x,
            name:  name.to_owned(),
        }
    }

    pub fn adjacency_list_for_dachshund(&self) -> Vec<(usize,usize)> {
        self.inner.iter().map(|e| (e.src.val(), e.dst.val())).collect()
    }
}
