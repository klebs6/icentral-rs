crate::ix!();

#[derive(Debug)]
pub struct AllPairsShortestPathCounts {
    name: String,
    data: MaybeIndexedMap<PathCounts>,
}

impl Default for AllPairsShortestPathCounts {

    fn default() -> Self {
        Self::empty_indexed("default_all_pairs_shortest_path_counts")
    }
}

impl CreateEmptyIndexed for AllPairsShortestPathCounts {

    fn empty_indexed(name: &str) -> Self {

        debug!("creating new indexed AllPairsShortestPathCounts named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }
}

impl CreateEmptyMapped for AllPairsShortestPathCounts {

    fn empty_mapped(name: &str) -> Self {

        debug!("creating new mapped AllPairsShortestPathCounts named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}

impl AllPairsShortestPathCounts {

    /// this will be (len x len)
    ///
    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new AllPairsShortestPathCounts named {} of len: {}", name, len);

        let mut data = MaybeIndexedMap::empty_indexed();

        data.reserve(len);

        for idx in 0..len {

            let my_path_counts_name = name![name, format!("path_counts_at_idx{}", idx)];
            let my_path_counts      = PathCounts::new(len,my_path_counts_name);

            let id = nodeid![idx];

            data.set(id, my_path_counts);
        }

        Self {
            name: name.to_owned(),
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Index<(NodeId,NodeId)> for AllPairsShortestPathCounts {

    type Output = usize;

    fn index(&self, idx: (NodeId,NodeId)) -> &Self::Output {
        self.data[idx.0].path_count_for_node_ref(idx.1)
    }
}

impl IndexMut<(NodeId,NodeId)> for AllPairsShortestPathCounts {

    fn index_mut(&mut self, idx: (NodeId,NodeId)) -> &mut Self::Output {
        self.data[idx.0].path_count_for_node_mut(idx.1)
    }
}
