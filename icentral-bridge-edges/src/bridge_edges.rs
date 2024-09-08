crate::ix!();

pub trait FindBridgeEdges {

    fn find_bridge_edges<GH: BccGraphHashInterface>(&mut self) -> Vec<Edge>;
}

impl<G: FindBiconnectedComponent> 
FindBridgeEdges for G {

    /// the single edge in a size 2 bcc is a bridge
    ///
    fn find_bridge_edges<GH: BccGraphHashInterface>(&mut self) -> Vec<Edge> {
        
        let mut out_vec = vec![];

        let mut bcc_vec: Vec<GH> = vec![];

        self.find_bicon_component(&mut bcc_vec);

        for i in 0..bcc_vec.len() {

            if bcc_vec[i].num_nodes() == 2 {

                let mut e: Edge = Edge::default();

                e = *bcc_vec[i].edges().iter().nth(0).unwrap();

                out_vec.push(e);
            }
        }

        out_vec
    }
}
