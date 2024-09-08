crate::ix!();

#[derive(Debug)]
pub struct AllPairsDistances {
    name: String,
    data: MaybeIndexedMap<DistanceMap>,
}

impl Default for AllPairsDistances {

    fn default() -> Self {
        Self::empty_indexed("default_all_pairs_distances")
    }
}

impl AllPairsDistances {

    /// this will be (len x len)
    ///
    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new AllPairsdistances named {} of len: {}", name, len);

        let mut data = MaybeIndexedMap::empty_indexed();

        data.reserve(len);

        for idx in 0..len {

            let my_distances_name = name![name, format!("distances_at_idx_{}",idx)];

            let distances = DistanceMap::new(len, my_distances_name);

            data.set(nodeid![idx],distances);
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

impl CreateEmptyIndexed for AllPairsDistances {

    fn empty_indexed(name: &str) -> Self {

        debug!("creating new indexed AllPairsDistances named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }
}

impl CreateEmptyMapped for AllPairsDistances {

    fn empty_mapped(name: &str) -> Self {

        debug!("creating new mapped AllPairsDistances named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}

impl Index<(NodeId,NodeId)> for AllPairsDistances {

    type Output = f64;

    fn index(&self, idx: (NodeId,NodeId)) -> &Self::Output {
        self.data[idx.0].distance_ref(idx.1)
    }
}

impl IndexMut<(NodeId,NodeId)> for AllPairsDistances {

    fn index_mut(&mut self, idx: (NodeId,NodeId)) -> &mut Self::Output {
        self.data[idx.0].distance_mut(idx.1)
    }
}
