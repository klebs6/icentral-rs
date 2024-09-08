crate::ix!();

#[macro_export] macro_rules! delegate_to_subgraph {
    ($tag:ident) => {
        paste::item!{
            delegate!{
                to self.$tag {

                    #[call(label_map_outin)]
                    pub fn [<$tag _label_map_outin>](&self, n: NodeId) -> NodeId;
                }
            }
        }
    }
}
