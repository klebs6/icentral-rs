crate::ix!();

#[derive(Clone,Debug)]
pub struct WorkspaceMap {
    name: String,
    data: MaybeIndexedMap<ICentralWorkspace>,
}

impl Default for WorkspaceMap {

    fn default() -> Self {
        Self::empty_indexed("default_workspace_map")
    }
}

impl WorkspaceMap {

    pub fn empty_indexed(name: &str) -> Self {

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(graph_len: usize, name: &str) -> Self {

        let mut data = MaybeIndexedMap::empty_indexed();

        data.reserve(graph_len);

        for idx in 0..graph_len {

            let name = format!["workspace_for_{}",nodeid![idx]];

            let workspace = ICentralWorkspace::new_init_all(graph_len, &name);

            data.set(nodeid![idx], workspace);
        }

        Self { 
            name: name.to_owned(),
            data 
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn has_workspace(&self, node: NodeId) -> bool {
        self.data.contains(node)
    }

    pub fn workspace_ref(&self, node: NodeId)
    -> &ICentralWorkspace 
    {
        &self.data[node]
    }

    pub fn workspace_mut(&mut self, node: NodeId)
    -> &mut ICentralWorkspace 
    {
        &mut self.data[node]
    }

    pub fn set_workspace_for_node(&mut self, node: NodeId, val: ICentralWorkspace) {
        self.data[node] = val;
    }

    /*
    pub fn reinit(&mut self, graph_len: usize) {
        self.data.reserve(graph_len);
        self.data.refill(graph_len,ICentralWorkspace::new_init_all(graph_len));
    }
    */

    /// this could possibly be parallelized in the
    /// future
    ///
    pub fn bbfs(&mut self, 
        config:    Option<BBFSConfig>, 
        component: &Component)
    {
        for node in NodeIdRange::new(0,self.len()) {

            let workspace = self.workspace_mut(node);

            bbfs(
                None, 
                workspace, 
                component, 
                node 
            );
        }
    }

    /// this could possibly be parallelized in the
    /// future
    ///
    pub fn bbfs_rbfs(&mut self, 
        scores:    &mut BetweennessScores,
        config:    Option<BBFSConfig>, 
        component: &mut Component)
    {
        for node in NodeIdRange::new(0,self.len()) {

            let workspace = self.workspace_mut(node);

            bbfs(
                None, 
                workspace, 
                component, 
                node 
            );

            rbfs(
                scores, 
                component, 
                node, 
                workspace, 
                None,
            );
        }
    }
}
