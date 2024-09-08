crate::ix!();

pub struct BcMemWorkspace {
    name:        String,
    new_parents: ParentsMap,
    new_sigmas:  SigmaMap,
    level_vec:   Vec<Vec<NodeId>>,
}

impl CreateNamedEmpty for BcMemWorkspace {

    fn empty(name: &str) -> Self {

        let new_parents_name = name![name, "new_parents"];
        let new_sigmas_name  = name![name, "new_sigmas"];

        Self {
            name:        name.to_owned(),
            new_parents: ParentsMap::empty_mapped(new_parents_name),
            new_sigmas:  SigmaMap::empty_mapped(new_sigmas_name),
            level_vec:   vec![],
        }
    }
}

impl BcMemWorkspace {

    pub fn push_level(&mut self, lvl: Vec<NodeId>) {
        self.level_vec.push(lvl);
    }

    pub fn level_push(&mut self, idx: usize, id: NodeId) {
        self.level_vec[idx].push(id);
    }

    pub fn level_pop(&mut self, idx: usize) 
        -> Option<NodeId> 
    {
        self.level_vec[idx].pop()
    }

    pub fn level_vec_len(&self) -> usize {
        self.level_vec.len()
    }

    pub fn level_vec_resize_default(&mut self,n: usize) {
        self.level_vec.resize(n,default!());
    }

    pub fn set_new_sigmas_from(&mut self, other: &SigmaMap) {
        self.new_sigmas = other.clone();
    }

    pub fn new_sigmas(&self) -> &SigmaMap {
        &self.new_sigmas
    }
}

delegate_to_sigmas!{BcMemWorkspace; new_sigmas}
delegate_to_parents!{BcMemWorkspace; new_parents}
