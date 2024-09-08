crate::ix!();

impl<GH> GetShortestPath for Graph<GH> {

    fn get_shortest_path(&mut self, 
        src:      NodeId,
        dst:      NodeId)  
    -> Result<Vec<NodeId>,BetweennessCentralityError> 
    {
        debug!("finding the shortest path between src={} and dst={}", src, dst);

        let len = self.num_nodes();

        let distances_name  = name![self.name(), "get_shortest_path::distances"];
        let parent_vec_name = name![self.name(), "get_shortest_path::parent_vec"];

        // do a BFS from dst to src and store path
        //
        let mut distances  = DistanceMap::new(len, distances_name);
        let mut parent_vec = PredecessorMap::new(len, parent_vec_name);

        let queue_name = name![self.name(), "get_shortest_path::queue"];

        let mut queue = NodeIdQueue::empty(queue_name);

        distances.set_zero_distance(dst);

        queue.enqueue(dst);

        while let Some(node) = queue.dequeue() {

            if node == src {
                break;
            }

            let nbr_vec = self.neighbors(node);

            for &nbr_id in nbr_vec.iter() {

                if distances.is_infinite(nbr_id) {

                    distances.set_distance_for_node(
                        nbr_id, 
                        distances.distance(node) + 1.0
                    );

                    parent_vec.set_predecessor_for_node(nbr_id, node);

                    queue.enqueue(nbr_id);
                }
            }
        }

        // fill the output path vector:
        let mut node_path = vec![];

        let mut nd: NodeId = src;

        while parent_vec.has_predecessor(nd) {

            node_path.push(nd);

            nd = parent_vec.predecessor_for_node(nd);
        }

        // insert dst
        node_path.push(nd);

        Ok(node_path)
    }
}
