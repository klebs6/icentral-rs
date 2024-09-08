crate::ix!();

#[macro_export] macro_rules! delegate_to_articulation_point_map {

    () => {
        delegate!{
            to self.articulation_point_map {

                pub fn has_both_articulation_points(&self, x: NodeId, y: NodeId) -> bool;

                pub fn has_either_articulation_point(&self, x: NodeId, y: NodeId) -> bool;

                pub fn has_articulation_point(&self, x: NodeId) -> bool;

                #[call(reinit)]
                pub fn articulation_point_map_reinit(&mut self, len: usize);

                #[call(clear)]
                pub fn articulation_point_map_clear(&mut self);

                #[call(len)]
                pub fn num_mapped_articulation_points(&self) -> usize;

                pub fn subgraph_micentraltude_through_articulation_point(&self, source: NodeId) -> f64;

                pub fn subgraphs_product_through_articulation_points(&self, 
                    s: NodeId, 
                    n: NodeId) -> f64;

                pub fn map_articulation_point(&mut self, 
                    x:     NodeId, 
                    sizes: &Vec<usize>);

                #[call(iter)]
                pub fn articulation_point_map_iter(&self) -> std::collections::hash_map::Iter<'_,NodeId,Vec<usize>>;
            }
        }
    };
    ($tag:ident) => {
        paste::item!{
            delegate!{
                to self.[<$tag _articulation_point_map>] {

                    #[call(has_both_articulation_points)]
                    pub fn [<has_both_ $tag _articulation_points>](&self, x: NodeId, y: NodeId) -> bool;

                    #[call(has_either_articulation_point)]
                    pub fn [<has_either_ $tag _articulation_point>](&self, x: NodeId, y: NodeId) -> bool;

                    #[call(has_articulation_point)]
                    pub fn [<has_ $tag _articulation_point>](&self, x: NodeId) -> bool;

                    #[call(reinit)]
                    pub fn [<$tag _articulation_point_map_reinit>](&mut self, len: usize);

                    #[call(clear)]
                    pub fn [<$tag _articulation_point_map_clear>](&mut self);

                    #[call(len)]
                    pub fn [<num_mapped_ $tag _articulation_points>](&self) -> usize;

                    #[call(subgraph_micentraltude_through_articulation_point)]
                    pub fn [<subgraph_micentraltude_through_ $tag _articulation_point>](&self, source: NodeId) -> f64;

                    #[call(subgraphs_product_through_articulation_points)]
                    pub fn [<subgraphs_product_through_ $tag _articulation_points>](&self, 
                        s: NodeId, 
                        n: NodeId) -> f64;

                    #[call(map_articulation_point)]
                    pub fn [<map_ $tag _articulation_point>](&mut self, 
                        x:     NodeId, 
                        sizes: &Vec<usize>);

                    #[call(iter)]
                    pub fn [<$tag _articulation_point_map_iter>](&self) -> std::collections::hash_map::Iter<'_,NodeId,Vec<usize>>;
                }
            }
        }
    };
}
