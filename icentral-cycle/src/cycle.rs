crate::ix!();

/*
 | But in the last days it shall come to pass, that
 | the mountain of the house of the LORD shall be
 | established in the top of the mountains, and it
 | shall be exalted above the hills; and people shall
 | flow unto it.
 |
 | And many nations shall come, and say, Come, and
 | let us go up to the mountain of the LORD, and to
 | the house of the God of Jacob; and he will teach
 | us of his ways, and we will walk in his paths: for
 | the law shall go forth of Zion, and the word of
 | the LORD from Jerusalem.
 |
 | And he shall judge among many people, and rebuke
 | strong nations afar off; and they shall beat their
 | swords into plowshares, and their spears into
 | pruninghooks: nation shall not lift up a sword
 | against nation, neither shall they learn war any
 | more.
 |
 | But they shall sit every man under his vine and
 | under his fig tree; and none shall make them
 | afraid: for the mouth of the LORD of hosts hath
 | spoken it.
 |
 | For all people will walk every one in the name of
 | his god, and we will walk in the name of the LORD
 | our God for ever and ever.
 |
 | -Micah, Ch. 4
 */
#[derive(Clone,Debug)]
pub struct Cycle {
    edges: Vec<Edge>,
}

impl NumEdges for Cycle {

    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl std::ops::Index<usize> for Cycle {

    type Output = Edge;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.edges[idx]
    }
}

/**
  | minimum cycle basis for a graph is a set
  | of cycles the representation is a vector
  | of edge lists
  |
  */
#[derive(Debug)]
pub struct MinimumCycleBasis {
    cycle_vec: Vec<Cycle>,
}

impl CreateEmpty for MinimumCycleBasis {

    fn empty() -> Self {
        Self {
            cycle_vec: vec![]
        }
    }
}

impl MinimumCycleBasis {

    pub fn cycles(&self) -> &Vec<Cycle> {
        &self.cycle_vec
    }

    pub fn num_cycles(&self) -> usize {
        self.cycle_vec.len()
    }

    pub fn cycle(&self, idx: NodeId) -> &Cycle {
        &self.cycle_vec[idx.val()]
    }
}
