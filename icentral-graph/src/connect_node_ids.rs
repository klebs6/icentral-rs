crate::ix!();

impl<GH> ConnectNodeIds for Graph<GH> {

    fn connect_nodeids(&mut self, src: NodeId, dst: NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("in {}, connecting nodeids {} and {}", self.name(), src, dst);

        self.insert_edge(&Edge::new(src,dst));

        Ok(())
    }
}
