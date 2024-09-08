crate::ix!();

#[derive(Debug)]
pub struct SubGraphMicentraltudeDebugProduct {
    s:    NodeId,
    vg_s: usize,
    n:    NodeId,
    vg_n: usize,
    c_t:  f64,
}

pub const UNCONNECTED_ARTICULATION_POINT: Vec<usize> = vec![];

// maps articulation points to sizes of subgraphs
// connected to the bcc through them
//
#[derive(Debug)]
pub struct ArticulationPointMap {
    name: String,
    data: MaybeIndexedMap<Vec<usize>>,
}

impl Named for ArticulationPointMap {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.name)
    }
}

impl ArticulationPointMap {

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new indexed ArticulationPoint map named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new mapped ArticulationPoint map named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new ArticulationPoint map named {} with len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len, UNCONNECTED_ARTICULATION_POINT),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_,NodeId,Vec<usize>> {
        match self.data {
            MaybeIndexedMap::Indexed(ref v) => {
                unimplemented!("maybe we should implement this?")
            }
            MaybeIndexedMap::Mapped(ref m) => {
                m.iter()
            }
        }
    }

    pub fn subgraph_micentraltude_through_articulation_point(&self, source: NodeId) -> f64 {

        debug!("computing subgraph_micentraltude through articulation point: {}", source);

        let micentraltude: usize = self.data[source].iter().sum();

        micentraltude as f64
    }

    pub fn subgraphs_product_through_articulation_points(&self, 
        s: NodeId, 
        n: NodeId) -> f64 
    {
        let vg_s: usize = self.data[s].iter().sum();
        let vg_n: usize = self.data[n].iter().sum();

        let c_t: f64 = (vg_s as f64) * (vg_n as f64);

        debug!(
            "computed product of the subgraph micentraltudes through articulation points {:#?}", 
            SubGraphMicentraltudeDebugProduct {
                s,
                vg_s,
                n,
                vg_n,
                c_t
            }
        );

        c_t
    }

    pub fn has_both_articulation_points(&self, x: NodeId, y: NodeId) -> bool {

        let result = self.has_articulation_point(x) && self.has_articulation_point(y);

        match result {
            true => {
                debug!("our ArticulationPointMap {} has both articulation points, {} and {}", self.name, x, y);
            }
            false => {
                debug!("our ArticulationPointMap {} does not have both articulation points, {} and {}", self.name, x, y);
            }
        }

        result
    }

    pub fn has_either_articulation_point(&self, x: NodeId, y: NodeId) -> bool {

        let result = self.has_articulation_point(x) || self.has_articulation_point(y);

        match result {
            true => {
                debug!("our ArticulationPointMap {} has either articulation point, {} or {}", self.name, x, y);
            }
            false => {
                debug!("our ArticulationPointMap {} does not have either articulation point, {} or {}", self.name, x, y);
            }
        }

        result
    }

    pub fn has_articulation_point(&self, x: NodeId) -> bool {
        self.data.contains(x)
    }

    pub fn map_articulation_point(&mut self, 
        x:     NodeId, 
        sizes: &Vec<usize>) 
    {

        debug!("mapping articulation point {} into {}", x, self.name);

        self.data.set(x, sizes.to_vec());
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("reinitializing ArticulationPointMap {} to len: {}", self.name, len);

        self.data.refill(len, UNCONNECTED_ARTICULATION_POINT);
    }

    pub fn clear(&mut self) {

        debug!("clearing ArticulationPointMap {} which contained {} items", self.name, self.len());

        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
