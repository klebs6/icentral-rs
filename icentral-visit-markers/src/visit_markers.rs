crate::ix!();

pub const UNVISITED: bool = false;

error_tree!{

    pub enum MarkVisitedError {
        Default,
    }

    pub enum MarkUnvisitedError {
        Default,
    }
}

#[derive(Clone,Debug)]
pub struct VisitMarkers {
    name: String,
    data: MaybeIndexedMap<bool>,
}

impl VisitMarkers {
    delegate!{
        to self.data {
            pub fn len(&self) -> usize;

            pub fn clear(&mut self);
        }
    }
}

impl VisitMarkers {

    pub fn new_from_nodes(nodes: Vec<NodeId>, name: &str) -> Self {

        debug!("creating new VisitMarkers named {} from {} nodes", name, nodes.len());

        let mut x = Self::empty_mapped(name);

        for node in nodes {
            x.data[node] = false;
        }

        x
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty indexed VisitMarkers named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty mapped VisitMarkers named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn iter_unvisited(&self) -> Vec<NodeId> {

        self.data
            .iter()
            .filter(|(k,v)| !**v)
            .map(|p| p.0)
            .collect()
    }

    pub fn new(len: usize, name: &str) -> Self {

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,UNVISITED),
        }
    }

    pub fn new_with_single_node_visited(len: usize, n: NodeId, name: &str) -> Self {

        debug!("creating new VisitMarkers named {} with node {} visited", name, n);

        let mut x = Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,UNVISITED),
        };

        x.visit(n);

        x
    }

    pub fn fill(&mut self, val: bool) {

        debug!("filling visit_markers with val: {}", val);

        self.data.fill(val);
    }

    pub fn fill_to_len(&mut self, len: usize, val: bool) {

        debug!("filling visit_markers to len: {}, val: {}", len, val);

        self.data.refill(len,val);
    }

    pub fn set_visited(&mut self, 
        id:  NodeId, 
        val: bool) 
    {
        match val {
            true  => debug!("marking node {} visited",   id),
            false => debug!("marking node {} unvisited", id),
        }

        self.data.set(id, val);
    }

    pub fn visited(&self, id: NodeId)  -> bool {
        self.data[id]
    }

    pub fn unvisited(&self, id: NodeId)  -> bool {
        !self.data[id]
    }

    pub fn visit(&mut self, id: NodeId) 
    {
        self.set_visited(id,true);
    }

    pub fn unvisit(&mut self, id: NodeId) 
    {
        self.set_visited(id,false);
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("reinitializing visit_markers to len: {}", len);

        self.data.refill(len,UNVISITED);
    }
}
