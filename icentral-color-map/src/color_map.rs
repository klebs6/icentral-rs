crate::ix!();

/// 1 means grey
///
#[derive(Clone,Debug)]
pub struct ColorMap {
    name: String,
    data: MaybeIndexedMap<Color>,
}

impl ColorMap {

    pub fn new_from_nodes(nodes: Vec<NodeId>, name: &str) -> Self {

        debug!("creating new ColorMap named {} from {} nodes", name, nodes.len());

        let mut builder = Self::empty_mapped(name);

        for node in nodes {

            builder.set_color_for_node(
                node,
                Color::None
            );
        }

        builder
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed ColorMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped ColorMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new ColorMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,Color::None),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_colored(&self, node: NodeId) -> bool {
        self.color_for_node(node) == Color::Grey
    }

    pub fn color_for_node(&self, node: NodeId) -> Color {
        self.data[node]
    }

    pub fn color_ref(&self, node: NodeId)
    -> &Color 
    {
        &self.data[node]
    }

    pub fn color_mut(&mut self, node: NodeId)
    -> &mut Color 
    {
        &mut self.data[node]
    }

    pub fn set_color_for_node(&mut self, node: NodeId, val: Color) {

        debug!("in {}, setting color {} for node {}", self.name, val, node);

        self.data.set(node, val);
    }

    /// convenience method for the common case
    /// where we want to set the color to *Color::Grey*
    pub fn set_color_for_node_grey(&mut self, node: NodeId) {
        self.set_color_for_node(node, Color::Grey);
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("in {}, reinitializing ColorMap to len: {}", self.name, len);

        self.data.refill(len, Color::None);
    }
}
