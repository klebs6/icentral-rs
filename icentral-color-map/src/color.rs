crate::ix!();

#[derive(Clone,Default,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum Color {

    #[default] 
    None,
    Grey,
}

impl fmt::Display for Color {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::None => {
                write!(f, "Color::None")
            }
            Color::Grey => {
                write!(f, "Color::Grey")
            }
        }
    }
}
