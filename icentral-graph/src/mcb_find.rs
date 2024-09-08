crate::ix!();

impl<GH> McbFind for Graph<GH> {

    /// will store into self.mcb
    ///
    fn mcb_find(&self) {
        todo!();
            /*
        #ifndef NO_LEDA
            leda::graph G;
            fill_leda_graph(graph, &G);

            leda::edge_array<int> len(G, 1);
            mcb::edge_num enumb(G);
            leda::array< mcb::spvecgf2 > mcb;
            int weight = mcb::UMCB_SVA(G, len, mcb, enumb);

            //cout << "Number of nodes: " << G.number_of_nodes() << endl;
            //cout << "Number of edges: " << G.number_of_edges() << endl;
            //G.print(cout);

            int i, j;
            leda::edge e;
            for (i = 0; i < enumb.dim_cycle_space(); ++i) {
                //printf("Cycle: {}", i);
                cycle_t cycle;
                forall(j, mcb[i]) { // traverse edges of i-th cycle
                    e = enumb(j);
                    // do something with edge e
                    //G.print_edge(e);
                    //cout << endl;
                    node_id_t src = G.source(e)->id();
                    node_id_t dst = G.target(e)->id();
                    edge_t edge(src, dst);
                    cycle.push_back(edge);
                }
                self.mcb->cycle_vec.push_back(cycle);
            }
        #endif
            */
    }
}
