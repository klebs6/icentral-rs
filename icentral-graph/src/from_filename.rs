crate::ix!();

impl<GH> FromFilename for Graph<GH> where GH: BccGraphHashInterface {

    fn from_filename(filename: &str) -> Self {

        let mut g = Graph::empty(&extract_graph_name(filename));

        g.read_graph(filename);

        g
    }
}
