crate::ix!();

pub trait GetNodesToMucs {

    fn get_nodes_to_mucs<'a>(&'a self) -> &'a MucIdMap;
}

/// Maps NodeId to MucId
#[derive(Debug,Clone)]
pub struct MucIdMap {
    name: String,
    data: MaybeIndexedMap<MinimumUnionCycleId>,
}

impl MucIdMap {

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> MaybeIndexedMapIterator<'_,MinimumUnionCycleId> {
        self.data.iter()
    }
}

impl CreateEmptyIndexed for MucIdMap {

    fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed MucIdMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }
}

impl CreateEmptyMapped for MucIdMap {

    fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped MucIdMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}

impl MucIdForNode for MucIdMap {

    fn mucid_for_node(&self, node: NodeId) -> MinimumUnionCycleId {

        let mucid = self.data[node];

        debug!("in {}, computed mucid {} for node: {}", self.name, mucid, node);

        mucid
    }
}

impl ReinitWithLen for MucIdMap {

    fn reinit(&mut self, len: usize) {

        debug!("reinitializing MucIdMap {} to len: {}", self.name, len);

        self.data.refill(len,MinimumUnionCycleId::inf());
    }
}

impl FillWith for MucIdMap {

    type Item = MinimumUnionCycleId;

    fn fill(&mut self, val: Self::Item) {

        debug!("filling MucIdMap {} with value: {}", self.name, val);

        self.data.fill(val);
    }
}

impl SetMucIdForNode for MucIdMap {

    fn set_mucid_for_node(
        &mut self, 
        node:  NodeId, 
        mucid: MinimumUnionCycleId) 
    {
        debug!("in {}, setting mucid to {} for node: {}", self.name, mucid, node);

        self.data.set(node, mucid);
    }
}
