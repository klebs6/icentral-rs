crate::ix!();

impl CreateNamedEmpty for SubGraph {

    fn empty(name: &str) -> Self {

        let nodes_map_name             = name![name, "nodes_map"];
        let edges_name                 = name![name, "edges"];
        let label_map_name             = name![name, "label_map"];
        let parents_name               = name![name, "parents"];
        let path_counts_name           = name![name, "path_counts"];
        let new_path_counts_name       = name![name, "new_path_counts"];
        let inc_path_counts_name       = name![name, "inc_path_counts"];
        let distances_name             = name![name, "distances"];
        let pair_dependencies_name     = name![name, "pair_dependencies"];
        let new_pair_dependencies_name = name![name, "new_pair_dependencies"];
        let sigmas_name                = name![name, "sigmas"];
        let new_sigmas_name            = name![name, "new_sigmas"];
        let visit_markers_name         = name![name, "visit_markers"];
        let stack_name                 = name![name, "stack"];
        let queue_name                 = name![name, "queue"];

        Self {
            name:                  name.to_owned(),
            nodes_map:             NeighborsMap::empty_indexed(nodes_map_name),
            edges:                 Edges::empty(edges_name),
            label_map:             LabelMap::empty(label_map_name),
            parents:               ParentsMap::empty_indexed(parents_name),
            path_counts:           PathCounts::empty_indexed(path_counts_name),
            new_path_counts:       PathCounts::empty_indexed(new_path_counts_name),
            inc_path_counts:       PathCounts::empty_indexed(inc_path_counts_name),
            distances:             DistanceMap::empty_indexed(distances_name),
            pair_dependencies:     PairDependencies::empty_indexed(pair_dependencies_name),
            new_pair_dependencies: PairDependencies::empty_indexed(new_pair_dependencies_name),
            sigmas:                SigmaMap::empty_indexed(sigmas_name),
            new_sigmas:            SigmaMap::empty_indexed(new_sigmas_name),
            visit_markers:         VisitMarkers::empty_indexed(visit_markers_name),
            stack:                 NodeIdStack::empty(stack_name),
            queue:                 NodeIdQueue::empty(queue_name),
        }
    }
}

impl SubGraph {

    /// used in the `From` implementations
    ///
    fn new_from_nodes_edges_and_label_map(
        nodes_map: NeighborsMap, 
        edges:     Edges, 
        label_map: LabelMap, 
        name:      &str) -> Self 
    {
        let len = nodes_map.len();

        assert!(nodes_map.len() == label_map.len());

        let parents_name               = name![name, "parents"];
        let path_counts_name           = name![name, "path_counts"];
        let new_path_counts_name       = name![name, "new_path_counts"];
        let inc_path_counts_name       = name![name, "inc_path_counts"];
        let distances_name             = name![name, "distances"];
        let pair_dependencies_name     = name![name, "pair_dependencies"];
        let new_pair_dependencies_name = name![name, "new_pair_dependencies"];
        let sigmas_name                = name![name, "sigmas"];
        let new_sigmas_name            = name![name, "new_sigmas"];
        let visit_markers_name         = name![name, "visit_markers"];
        let stack_name                 = name![name, "stack"];
        let queue_name                 = name![name, "queue"];

        Self {
            name:                   name.to_owned(),
            nodes_map,
            edges,
            label_map,
            parents:                ParentsMap::new(len, parents_name),
            path_counts:            PathCounts::new(len, path_counts_name),
            new_path_counts:        PathCounts::new(len, new_path_counts_name),
            inc_path_counts:        PathCounts::new(len, inc_path_counts_name),
            distances:              DistanceMap::new(len, distances_name),
            pair_dependencies:      PairDependencies::new(len, pair_dependencies_name),
            new_pair_dependencies:  PairDependencies::new(len, new_pair_dependencies_name),
            sigmas:                 SigmaMap::new(len, sigmas_name),
            new_sigmas:             SigmaMap::new(len, new_sigmas_name),
            visit_markers:          VisitMarkers::new(len, visit_markers_name),
            stack:                  NodeIdStack::empty(stack_name),
            queue:                  NodeIdQueue::empty(queue_name),
        }
    }

    /// QUESTION: why is this impl different from
    /// `&Graph`
    ///
    pub fn new_from_graph_hash_ref<GH>(gh: &GH, name: &str) -> Self 
        where GH: NumNodes + MappedNodes + GetEdges + GetNodeIdRange + GetNeighborsForNode
    {

        debug!("creating new SubGraph named {} from GraphHash of len {}", name, gh.num_nodes());

        let len = gh.num_nodes();

        let nodes_map_name = name![name, "nodes_map"];
        let edges_name     = name![name, "edges"];
        let label_map_name = name![name, "label_map"];

        let label_map = LabelMap::new_from_graph_ref(gh, label_map_name);
        let nodes_map = NeighborsMap::new(len, nodes_map_name);
        let edges     = Edges::new_remapped(&label_map,gh, edges_name);

        SubGraph::new_from_nodes_edges_and_label_map(
            nodes_map,
            edges,
            label_map,
            name
        )
    }

    /// QUESTION: why is this impl different from
    /// `&GraphHash`
    ///
    pub fn new_from_graph_ref<G>(g: &G, name: &str) -> Self 
        where G: NumNodes + MappedNodes + GetNodeIdRange + GetNeighborsForNode + GetEdges
    {

        let num_nodes = g.num_nodes();

        let nodes_map_name = name![name, "nodes_map"];
        let edges_name     = name![name, "edges"];
        let label_map_name = name![name, "label_map"];

        let nodes_map = NeighborsMap::new_from_graph_ref(g, nodes_map_name);
        let edges     = Edges::new_from_graph_ref(g, edges_name);
        let label_map = LabelMap::new_from_graph_ref(g, label_map_name);

        SubGraph::new_from_nodes_edges_and_label_map(
            nodes_map,
            edges,
            label_map,
            name
        )
    }
}
