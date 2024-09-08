crate::ix!();
   
//-------------------------------------------[icentral/src/unit_tests.h]

#[derive(Debug)]
pub enum GraphMock {
    PaperZero,
    Paper,
    Basic,
}

pub fn paper_edges_for_dachshund() -> Vec<(usize, usize)> {
    vec![
        (0,1),
        (0,3),
        (1,2),
        (2,3),
        (2,4),
        (2,7),
        (3,5),
        (5,6),
    ]
}

impl Named for GraphMock {

    fn name(&self) -> Cow<'_,str> {
        match self {
            GraphMock::PaperZero => "paper-zero",
            GraphMock::Paper     => "paper",
            GraphMock::Basic     => "basic",
        }.into()
    }
}

impl GraphMock {

    pub fn fill<G: ConnectNodeIds + InitWithSize>(&self, graph: &mut G)  {
        match self {
            GraphMock::PaperZero => {
                debug!("filling graph from GraphMock::PaperZero");
                graph.init_size(8);
                graph.connect_nodeids(NodeId::from(0),  NodeId::from(1));
                graph.connect_nodeids(NodeId::from(0),  NodeId::from(3));
                graph.connect_nodeids(NodeId::from(1),  NodeId::from(2));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(3));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(4));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(7));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(5));
                graph.connect_nodeids(NodeId::from(5),  NodeId::from(6));
            }
            GraphMock::Paper => {
                debug!("filling graph from GraphMock::Paper");
                graph.init_size(9);
                graph.connect_nodeids(NodeId::from(0),  NodeId::from(0));
                graph.connect_nodeids(NodeId::from(1),  NodeId::from(2));
                graph.connect_nodeids(NodeId::from(1),  NodeId::from(4));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(3));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(4));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(5));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(8));
                graph.connect_nodeids(NodeId::from(4),  NodeId::from(6));
                graph.connect_nodeids(NodeId::from(6),  NodeId::from(7));
            }
            GraphMock::Basic => {
                debug!("filling graph from GraphMock::Basic");
                graph.init_size(20);
                graph.connect_nodeids(NodeId::from(0),  NodeId::from(1));
                graph.connect_nodeids(NodeId::from(1),  NodeId::from(2));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(3));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(0));
                graph.connect_nodeids(NodeId::from(0),  NodeId::from(4));
                graph.connect_nodeids(NodeId::from(4),  NodeId::from(5));
                graph.connect_nodeids(NodeId::from(5),  NodeId::from(6));
                graph.connect_nodeids(NodeId::from(6),  NodeId::from(7));
                graph.connect_nodeids(NodeId::from(7),  NodeId::from(8));
                graph.connect_nodeids(NodeId::from(8),  NodeId::from(4));
                graph.connect_nodeids(NodeId::from(5),  NodeId::from(9));
                graph.connect_nodeids(NodeId::from(9),  NodeId::from(8));
                graph.connect_nodeids(NodeId::from(3),  NodeId::from(10));
                graph.connect_nodeids(NodeId::from(10), NodeId::from(11));
                graph.connect_nodeids(NodeId::from(11), NodeId::from(12));
                graph.connect_nodeids(NodeId::from(12), NodeId::from(13));
                graph.connect_nodeids(NodeId::from(13), NodeId::from(14));
                graph.connect_nodeids(NodeId::from(14), NodeId::from(15));
                graph.connect_nodeids(NodeId::from(15), NodeId::from(10));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(16));
                graph.connect_nodeids(NodeId::from(16), NodeId::from(17));
                graph.connect_nodeids(NodeId::from(17), NodeId::from(18));
                graph.connect_nodeids(NodeId::from(2),  NodeId::from(19));
            }
        }
    }
}
