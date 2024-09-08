crate::ix!();

/**
  | Stores the information used in one iteration
  | of iCentral
  | 
  | Namely, \sigma \parents \delta \D \S both before
  | edge insertion and after edge insertion
  | 
  | TODO study the effectiveness of storing
  | old/new
  | 
  | TODO reconsider what to store and what
  | not to (after the iCentral is implemented)
  | 
  | TODO reconsider the names of the structures
  |
  */
#[derive(Clone)]
pub struct ICentralWorkspace {
    parents:            ParentsMap,
    distances:          DistanceMap,
    deltas:             DeltaMap,
    new_deltas:         DeltaMap,
    capital_deltas:     DeltaMap,
    new_capital_deltas: DeltaMap,
    sigmas:             SigmaMap,
    new_sigmas:         SigmaMap,
    inc_sigmas:         SigmaMap,
    visit_markers:      VisitMarkers,
    stack:              NodeIdStack,
}

impl Default for ICentralWorkspace {

    fn default() -> Self {
        Self::empty("default_icentral_workspace")
    }
}

delegate_to_parents![ICentralWorkspace];
delegate_to_visit_markers![ICentralWorkspace];
delegate_to_bfs_stack![ICentralWorkspace];

delegate_to_distances![ICentralWorkspace];

delegate_to_sigmas![ICentralWorkspace];
delegate_to_sigmas![ICentralWorkspace; new_sigmas];
delegate_to_sigmas![ICentralWorkspace; inc_sigmas];

delegate_to_deltas![ICentralWorkspace];
delegate_to_deltas![ICentralWorkspace; new_deltas];
delegate_to_deltas![ICentralWorkspace; capital_deltas];
delegate_to_deltas![ICentralWorkspace; new_capital_deltas];

impl ICentralWorkspace {

    pub fn new_sigmas(&self) -> &SigmaMap {
        &self.new_sigmas
    }

    pub fn sigmas(&self) -> &SigmaMap {
        &self.sigmas
    }

    pub fn set_new_sigmas_from(&mut self, other: &SigmaMap) {
        self.new_sigmas = other.clone();
    }

    /// fix order of workspace.stack
    ///
    /// IMP::THIS CAN BE MADE much BETTER!
    ///
    /// HEAP FOR EXAMPLE
    ///
    /// EVEN THE SWAPPING CAN BE DONE MORE EFFICIENTLY
    ///
    /// for now it's not a bottleneck
    ///
    pub fn fix_order_of_workspace_stack(&mut self) 
    {
        for i in 1..self.stack.len() {

            let im1 = self.stack_node_at_index(i - 1);
            let i0  = self.stack_node_at_index(i);

            if self.distance(im1) > self.distance(i0) {

                let mut j: usize = i;

                let jm1 = self.stack_node_at_index(j - 1);
                let j0  = self.stack_node_at_index(j);

                while self.distance(jm1) > self.distance(j0) {

                    let tmp: NodeId = self.stack_node_at_index(j - 1);

                    self.stack_set_node_at_index(
                        j - 1, 
                        self.stack_node_at_index(j)
                    );

                    self.stack_set_node_at_index(j, tmp);

                    j -= 1;
                }
            }
        }
    }

    pub fn compute_new_path_counts_and_paths(
        &mut self, 
        src: NodeId, 
        dst: NodeId)
    {
        if !self.distance_is_one_step_away(dst,src) {

            self.clear_node_parents(dst);

            self.add_parent(dst,src);

            self.set_sigma_value_for_node(
                dst,
                self.sigma_value_for_node(src)
            );

        } else {

            self.add_parent(dst,src);

            self.increment_sigma_value_for_node(
                dst, 
                self.sigma_value_for_node(src)
            );
        }
    }

    pub fn update_for_partial_bbfs_addition(&mut self, 
        queue: &mut NodeIdQueue, 
        w:     NodeId, 
        v:     NodeId) 
    {
        if self.distance_is_farther_than_one_away(w,v) {

            self.set_distance_one_step_away(w,v);

            self.clear_node_parents(w);

            self.add_parent(w,v);

            self.sigma_set_node_to_zero(w);

            self.inc_sigmas_set_sigma_value_for_node(
                w, 
                self.inc_sigmas_sigma_value_for_node(v)
            );

            self.increment_sigma_value_for_node(
                w, 
                self.inc_sigmas_sigma_value_for_node(w)
            );

            if self.unvisited(w) {

                self.visit(w);

                queue.enqueue(w);
            }

        } else {

            if self.distance_is_one_step_away(w,v) {

                self.inc_sigmas_set_sigma_value_for_node(
                    w, 
                    self.inc_sigmas_sigma_value_for_node(v)
                );

                self.increment_sigma_value_for_node(
                    w, 
                    self.inc_sigmas_sigma_value_for_node(v)
                );

                if !self.has_parent(w,v) {
                    self.add_parent(w,v);
                }

                if self.unvisited(w) {
                    self.visit(w);
                    queue.enqueue(w);
                }
            }
        }
    }

    pub fn deltas(&self) -> &DeltaMap {
        &self.deltas
    }

    pub fn get_new_delta_ratio(&self, 
        v_p: NodeId, 
        v_n: NodeId) -> f64 
    {
        self.new_deltas_delta_ratio(v_p, v_n)
    }

    pub fn set_deltas_from_new_deltas(&mut self) {
        self.deltas = self.new_deltas.clone();
    }

    pub fn set_sigmas_from_other(&mut self, new: &SigmaMap) {
        self.sigmas = new.clone();
    }

    pub fn get_sigma_ratio(&self, 
        v_p: NodeId, 
        v_n: NodeId) -> f64 
    {
        self.sigma_ratio(v_p, v_n)
    }

    pub fn get_new_sigma_ratio(&self, 
        v_p: NodeId, 
        v_n: NodeId) -> f64 
    {
        self.new_sigmas_sigma_ratio(v_p, v_n)
    }

    pub fn update_new_capital_deltas_with_new_delta_ratio(
        &mut self, 
        v_p: NodeId, 
        v_n: NodeId) 
    {
        let new_sp_sn = self.get_new_delta_ratio(v_p,v_n);

        let t0 = self.new_capital_deltas_delta_value_for_node(v_p);
        let t1 = self.new_capital_deltas_delta_value_for_node(v_n);

        self.new_capital_deltas_set_delta_value_for_node(
            v_p, 
            t0 + t1 * new_sp_sn
        );
    }

    pub fn update_new_capital_deltas_with_new_sigma_ratio(
        &mut self, 
        v_p: NodeId,
        v_n: NodeId) 
    {
        let sp_sn = self.get_new_sigma_ratio(v_p,v_n);

        let ndp   = self.new_capital_deltas_delta_value_for_node(v_p);
        let ndn   = self.new_capital_deltas_delta_value_for_node(v_n);

        self.new_capital_deltas_set_delta_value_for_node(
            v_p, 
            ndp + ndn * sp_sn
        );
    }

    pub fn update_new_deltas_with_new_delta_ratio(
        &mut self, 
        v_p: NodeId, 
        v_n: NodeId) 
    {
        let new_sp_sn = self.get_new_delta_ratio(v_p,v_n);

        let ndp = self.new_deltas_delta_value_for_node(v_p);
        let ndn = self.new_deltas_delta_value_for_node(v_n);
        let one_p_ndn = 1.0 + ndn;

        self.new_deltas_set_delta_value_for_node(
            v_p, 
            ndp + new_sp_sn * one_p_ndn
        );
    }

    pub fn update_new_deltas_with_new_sigma_ratio(
        &mut self, 
        v_p: NodeId, 
        v_n: NodeId) 
    {
        let sp_sn     = self.get_new_sigma_ratio(v_p, v_n);
        let ndp       = self.new_deltas_delta_value_for_node(v_p);
        let ndn       = self.new_deltas_delta_value_for_node(v_n);
        let one_p_ndn = 1.0 + ndn;

        self.new_deltas_set_delta_value_for_node(
            v_p, 
            ndp + sp_sn * one_p_ndn
        );
    }

    pub fn maybe_update_capital_deltas(&mut self, 
        component: &Component, 
        source:    NodeId, 
        v_p:       NodeId, 
        v_n:       NodeId) 
    {
        if component.has_articulation_point(source) {

            self.update_capital_deltas(v_p,v_n);
        }
    }

    pub fn maybe_update_new_capital_deltas(&mut self, 
        component:  &Component, 
        source:     NodeId,
        v_p:        NodeId,
        v_n:        NodeId) 
    {
        if component.has_articulation_point(source) {

            ///TODO: is this sigma or delta ratio?
            self.update_new_capital_deltas_with_new_sigma_ratio(v_p,v_n);
        }
    }

    pub fn update_capital_deltas(&mut self, v_p: NodeId, v_n: NodeId) {

        let sp_sn = self.get_sigma_ratio(v_p,v_n);

        let dp = self.capital_deltas_delta_value_for_node(v_p);
        let dn = self.capital_deltas_delta_value_for_node(v_n);

        let dn_by_sp_sn = dn * sp_sn;

        self.capital_deltas_set_delta_value_for_node(
            v_p, 
            dp + dn_by_sp_sn
        );
    }

    pub fn update_deltas_for_each_p(
        &mut self, 
        component: &Component, 
        source:    NodeId, 
        v_n:       NodeId)
    {
        for &v_p in self.parents_for_node(v_n).iter() {

            self.update_all_deltas_for_component(
                component,
                source,
                v_p,
                v_n
            );
        }
    }

    pub fn update_all_deltas_for_component(
        &mut self, 
        component: &Component, 
        source:    NodeId, 
        v_p:       NodeId, 
        v_n:       NodeId)
    {
        self.update_deltas(v_p,v_n);

        self.maybe_update_capital_deltas(component,source,v_p,v_n);

        self.update_new_deltas_with_new_sigma_ratio(v_p,v_n);

        self.maybe_update_new_capital_deltas(component, source, v_p, v_n);
    }

    pub fn update_deltas(&mut self, v_p: NodeId, v_n: NodeId) {

        let sp_sn    = self.get_sigma_ratio(v_p,v_n);
        let dn       = self.delta_value_for_node(v_n);
        let dp       = self.delta_value_for_node(v_p);
        let one_p_dn = 1.0 + dn;

        self.set_delta_value_for_node(
            v_p, 
            dp + sp_sn * one_p_dn
        );
    }

    pub fn maybe_update_capital_deltas_for_component(
        &mut self, 
        component: &Component, 
        source:    NodeId, 
        v_n:       NodeId) 
    {
        if component.has_both_articulation_points(source, v_n)
        {
            self.update_both_capital_deltas_for_component(
                component,
                source,
                v_n
            );
        }
    }

    pub fn update_both_capital_deltas_for_component(
        &mut self, 
        component: &Component, 
        source:    NodeId, 
        v_n:       NodeId) 
    {
        let c_t = component
            .subgraphs_product_through_articulation_points(
                source,
                v_n
            );

        self.capital_deltas_increment_delta_value_for_node(
            v_n, 
            c_t
        );

        self.new_capital_deltas_increment_delta_value_for_node(
            v_n, 
            c_t
        );
    }

    pub fn process_outer_layer_neighbor(&mut self,
        bc_mem:    &mut BcMemWorkspace,
        neighbor:  NodeId,
        queue:     &mut NodeIdQueue,
        v:         NodeId) 
    -> Result<(), BetweennessCentralityError>  
    {
        self.set_distance_one_step_away(neighbor,v);

        // lost_parents[neighbor] = parents[neighbor];
        bc_mem.new_parents_add_parent(neighbor, v);

        let parents = self.parents_for_node(neighbor);

        for &parent in parents.iter() {

            self.attenuate_deltas(
                parent,
                neighbor
            );
        }

        self.clear_node_parents(neighbor);

        // parents[neighbor].push_back(v);
        bc_mem.new_sigmas_sigma_set_node_to_zero(neighbor);

        self.inc_sigmas_set_sigma_value_for_node(
            neighbor, 
            self.inc_sigmas_sigma_value_for_node(v)
        );

        bc_mem.new_sigmas_increment_sigma_value_for_node(
            neighbor, 
            self.inc_sigmas_sigma_value_for_node(neighbor)
        );

        if self.unvisited(neighbor) {

            self.visit(neighbor);

            // if(new_sigmas_sigma_value_for_node(&neighbor) != self.sigma_value_for_node(&neighbor))
            queue.enqueue(neighbor);
        }

        Ok(())
    }

    pub fn process_first_layer_neighbor(&mut self,
        bc_mem:    &mut BcMemWorkspace,
        neighbor:  NodeId,
        queue:     &mut NodeIdQueue,
        v:         NodeId) 
    -> Result<(), BetweennessCentralityError>  
    {
        self.inc_sigmas_increment_sigma_value_for_node(
            neighbor, 
            self.inc_sigmas_sigma_value_for_node(v)
        );

        bc_mem.new_sigmas_increment_sigma_value_for_node(
            neighbor, 
            self.inc_sigmas_sigma_value_for_node(v)
        );

        if !self.has_parent(neighbor, v) {

            bc_mem.new_parents_add_parent(neighbor, v);
        }

        if self.unvisited(neighbor) {

            self.visit(neighbor);

            // if(new_sigmas_sigma_value_for_node(&neighbor) != sigma_value_for_node(&neighbor))
            queue.enqueue(neighbor);
        }

        Ok(())
    }

    pub fn attenuate_deltas(&mut self, src: NodeId, dst: NodeId)
    {
        let sigma_ratio = self.sigma_ratio(src, dst);

        let t1 = 1.0 + self.delta_value_for_node(dst);

        let attenuation = sigma_ratio * t1;

        self.attenuate_delta_value_for_node(
            src, 
            attenuation
        );
    }

    pub fn calculate_deltas_step_with_sigma_ratio(
        &self, 
        sigma_ratio: f64,
        w:           NodeId) -> f64 
    {
        let one_p_delta_w = 1.0 + self.delta_value_for_node(w);

        let step = sigma_ratio * one_p_delta_w;

        step
    }

    pub fn calculate_deltas_step(
        &self, 
        v: NodeId, 
        w: NodeId) -> f64 
    {
        let sigma_ratio = self.sigma_ratio(v,w);

        self.calculate_deltas_step_with_sigma_ratio(
            sigma_ratio,
            w
        )
    }

    pub fn update_delta_bc_of_vertices_for_node(
        &self,
        w:                    NodeId,
        delta_bc_of_vertices: &mut BetweennessScores)
    {
        delta_bc_of_vertices.decrease_score_for_node(
            w, 
            self.delta_value_for_node(w) / 2.0
        );

        delta_bc_of_vertices.increase_score_for_node(
            w, 
            self.new_deltas_delta_value_for_node(w) / 2.0
        );
    }

    pub fn refill_deltass(&mut self, len: usize) {
        self.deltas.reinit(len);
        self.capital_deltas.reinit(len);
        self.new_capital_deltas.reinit(len);
        self.new_deltas.reinit(len);
    }
}

impl fmt::Debug for ICentralWorkspace {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ICentralWorkspace")
            .field("capital_deltas",      &self.capital_deltas)
            .field("capital_deltas_len",  &self.capital_deltas.len())
            .field("deltas",              &self.deltas)
            .field("deltas_len",          &self.deltas.len())
            .field("distances",           &self.distances)
            .field("distances_len",       &self.distances.len())
            .field("new_capital_deltas",  &self.new_capital_deltas)
            .field("new_deltas",          &self.new_deltas)
            .field("new_sigmas",          &self.new_sigmas)
            .field("new_sigmas_len",      &self.new_sigmas.len())
            .field("parents",             &self.parents)
            .field("parents_len",         &self.parents.len())
            .field("inc_sigmas",          &self.inc_sigmas)
            .field("sigmas_len",          &self.sigmas.len())
            .field("stack",               &self.stack)
            .field("visit_markers",       &self.visit_markers)
            .finish_non_exhaustive()
    }
}

impl CreateNamedEmpty for ICentralWorkspace {

    /// TODO: try to eliminate this if possible
    fn empty(name: &str) -> Self  {

        let parents_name            = name![name, "parents"];
        let distances_name          = name![name, "distances"];
        let deltas_name             = name![name, "deltas"];
        let new_deltas_name         = name![name, "new_deltas"];
        let capital_deltas_name     = name![name, "capital_deltas"];
        let new_capital_deltas_name = name![name, "new_capital_deltas"];
        let sigmas_name             = name![name, "sigmas"];
        let new_sigmas_name         = name![name, "new_sigmas"];
        let inc_sigmas_name         = name![name, "inc_sigmas"];
        let visit_markers_name      = name![name, "visit_markers"];
        let stack_name              = name![name, "stack"];

        Self {
            parents:            ParentsMap::empty_mapped(parents_name),
            distances:          DistanceMap::empty_mapped(distances_name),
            deltas:             DeltaMap::empty_mapped(deltas_name),
            new_deltas:         DeltaMap::empty_mapped(new_deltas_name),
            capital_deltas:     DeltaMap::empty_mapped(capital_deltas_name),
            new_capital_deltas: DeltaMap::empty_mapped(new_capital_deltas_name),
            sigmas:             SigmaMap::empty_mapped(sigmas_name),
            new_sigmas:         SigmaMap::empty_mapped(new_sigmas_name),
            inc_sigmas:         SigmaMap::empty_mapped(inc_sigmas_name),
            visit_markers:      VisitMarkers::empty_mapped(visit_markers_name),
            stack:              NodeIdStack::empty(stack_name),
        }
    }
}

impl ICentralWorkspace {
    
    pub fn new_init_all(n: usize, name: &str) -> Self  {

        let parents_name            = name![name, "parents"];
        let distances_name          = name![name, "distances"];
        let deltas_name             = name![name, "deltas"];
        let new_deltas_name         = name![name, "new_deltas"];
        let capital_deltas_name     = name![name, "capital_deltas"];
        let new_capital_deltas_name = name![name, "new_capital_deltas"];
        let sigmas_name             = name![name, "sigmas"];
        let new_sigmas_name         = name![name, "new_sigmas"];
        let inc_sigmas_name         = name![name, "inc_sigmas"];
        let visit_markers_name      = name![name, "visit_markers"];
        let stack_name              = name![name, "stack"];

        Self {
            parents:            ParentsMap::new(n,parents_name),
            distances:          DistanceMap::new(n,distances_name),
            deltas:             DeltaMap::new(n,deltas_name),
            new_deltas:         DeltaMap::new(n,new_deltas_name),
            capital_deltas:     DeltaMap::new(n,capital_deltas_name),
            new_capital_deltas: DeltaMap::new(n,new_capital_deltas_name),
            sigmas:             SigmaMap::new(n,sigmas_name),
            new_sigmas:         SigmaMap::new(n,new_sigmas_name),
            inc_sigmas:         SigmaMap::new(n,inc_sigmas_name),
            visit_markers:      VisitMarkers::new(n,visit_markers_name),
            stack:              NodeIdStack::empty(stack_name),
        }
    }

    pub fn init_all(&mut self, n: usize) {
        debug!("ICentralWorkspace init_all!, n={}", n);

        self.parents.reinit(n);
        self.sigmas.reinit(n);
        self.distances.reinit(n);
        self.inc_sigmas.reinit(n);
        self.deltas.reinit(n);
        self.capital_deltas.reinit(n);
        self.visit_markers.reinit(n);

        self.stack.clear();

        debug!("workspace initialization *complete*");

        //    fill_vec<int>(new_sigmas, N, 0);
        //    fill_vec<double>(new_deltas, N, 0);
        //    fill_vec<double>(new_capital_deltas, N, 0);
            
        //    fill_vec<vector<node_id_t> >(old_P, N, vector<node_id_t>());
        //    fill_vec<int>(old_sigmas, N, 0);
        //    fill_vec<int>(old_distances, N, -1);
        //    fill_vec<double>(old_deltas, N, 0);
        //    fill_vec<double>(old_capital_deltas, N, 0);
    }

    /**
      | needed for d=1 only
      |
      */
    pub fn init_new(&mut self, n: usize)  {
        self.new_sigmas.reinit(n);
        self.new_deltas.reinit(n);
        self.new_capital_deltas.reinit(n);
    }
}
