crate::ix!();

#[derive(Debug)]
pub struct BBFSConfig {
    edge:              Option<Edge>,
    handle_new_sigmas: bool,
}

impl Default for BBFSConfig {

    fn default() -> Self {
        Self {
            edge:              None,
            handle_new_sigmas: false,
        }
    }
}

impl BBFSConfig {

    pub fn edge(&self) -> Edge {
        self.edge.unwrap()
    }

    pub fn handle_new_sigmas(&self) -> bool {
        self.handle_new_sigmas
    }

    pub fn new_rbfs_d1(edge: Edge) -> Self {
        Self {
            edge:              Some(edge),
            handle_new_sigmas: true,
        }
    }
}
