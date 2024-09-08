crate::ix!();

#[macro_export] macro_rules! allow_twolevel_named_members {

    ($($outer_type:ident [ $fn_name:ident ] $inner_type:ident [ $inner_type_constructor:ident ]);*) => {
        $(
            allow_twolevel_named_member!($outer_type [ $fn_name ] $inner_type [ $inner_type_constructor ] );
        )*
    }
}

/// $outer_type and $inner_type need to be
/// a variant of
/// betweenness_json_builder::ParentAddress and
/// betweenness_json_builder::ChildAddress
/// respectively
///
/// $inner_type_constructor needs to be
/// implemented for $inner_type and accept &str as
/// input
///
/// $fn_name is ultimately what we can use to
/// call this functionality as a struct method
/// from OuterType
#[macro_export] macro_rules! allow_twolevel_named_member {

    ($outer_type:ident [ $fn_name:ident ] $inner_type:ident [ $inner_type_constructor:ident ]) => {

        impl $outer_type {

            pub fn $fn_name(parent: &str, child: &str) -> $inner_type {

                let parent_address = betweenness_json_builder::ParentAddress::$outer_type { 
                    name: parent.to_owned() 
                };

                let child_address = betweenness_json_builder::ChildAddress::$inner_type { 
                    name: child.to_owned() 
                };

                let name = BetweennessJsonBuilder::new(
                    parent_address,
                    child_address
                ).to_json();

                $inner_type::$inner_type_constructor(&name)
            }
        }
    }
}

pub mod betweenness_json_builder {

    use super::*;

    #[derive(Serialize)]
    pub enum ParentAddress {
        GraphHash         { name: String },
        Graph             { name: String },
        SubGraph          { name: String },
        MinimumUnionCycle { name: String },
    }

    #[derive(Serialize)]
    pub enum ChildAddress {
        PathCounts        { name: String },
        Edges             { name: String },
        NeighborsMap      { name: String },
        ParentsMap        { name: String },
        SigmaMap          { name: String },
        DeltaMap          { name: String },
        VisitMarkers      { name: String },
        PairDependencies  { name: String },
        DistanceMap       { name: String },
        NodeIdStack       { name: String },
        NodeIdQueue       { name: String },
    }
}

#[derive(Serialize)]
pub struct BetweennessJsonBuilder {
    parent_address: betweenness_json_builder::ParentAddress,
    child_address:  betweenness_json_builder::ChildAddress,
}

impl BetweennessJsonBuilder {

    pub fn new(
        parent_address: betweenness_json_builder::ParentAddress, 
        child_address:  betweenness_json_builder::ChildAddress) -> Self 
    {
        BetweennessJsonBuilder { 
            parent_address,
            child_address
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[test] fn test_betweenness_json_builder() {

    setup_test_logger![];

    let parent  = betweenness_json_builder::ParentAddress::GraphHash { name: "gh".to_owned() };
    let child   = betweenness_json_builder::ChildAddress::PathCounts { name: "pc".to_owned() };

    let builder = BetweennessJsonBuilder::new(parent,child);

    debug!("{}", builder.to_json());
}
