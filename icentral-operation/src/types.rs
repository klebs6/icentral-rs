crate::ix!();

//-------------------------------------------[icentral/src/types.h]

pub const EPS: f32   = 0.0001;

#[derive(Copy,Clone,Default,Debug,PartialEq,Eq)]
pub enum Operation {
    #[default] Insertion, 
    Deletion
}

impl Operation {

    pub fn is_deletion(&self) -> bool {
        *self == Operation::Deletion
    }

    pub fn is_insertion(&self) -> bool {
        *self == Operation::Insertion
    }
}
