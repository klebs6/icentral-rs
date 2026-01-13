// ---------------- [ File: icentral-subgraph/src/clear.rs ]
crate::ix!();

impl Clear for SubGraph {

    fn clear(&mut self) 
    {
        self.nodes_map.clear();

        self.edges.clear();

        self.label_map.clear();
    }
}
