crate::ix!();

pub struct LargestBiconnectedComponents<'g,'v,GH> 
where GH: BccGraphHashInterface
{
    graph: &'g Graph<GH>,

    bcc_vec: &'v Vec<GH>,

    /// index of the lbcc, # of nodes
    lbcc_i: usize,

    /// index of the lbcc, # of edges
    lbcc_j: usize,

    /// highest # of nodes
    lbcc_n: usize,

    /// highest # of edges
    lbcc_m: usize,
}

impl<'g,'v,GH> fmt::Debug for LargestBiconnectedComponents<'g,'v,GH> 
where GH: BccGraphHashInterface
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding     = f.debug_struct("LargestBiconnectedComponents");

        let mut builder = binding;

        if self.lbcc_i != self.lbcc_j {
            builder.field("warning", &"LargestBiconnectedComponents_m is not LargestBiconnectedComponents_n!");
        }

        builder.field("num_nodes:   {}", &self.num_nodes());
        builder.field("num_edges:   {}", &self.num_edges());
        builder.field("frac_nodes:  {}", &self.frac_nodes());
        builder.field("frac_edges:  {}", &self.frac_edges());

        builder.finish()
    }
}

impl<'g,'v,GH> NumNodes for LargestBiconnectedComponents<'g,'v,GH> 
where GH: BccGraphHashInterface
{
    fn num_nodes(&self) -> usize {
        self.lbcc_n
    }
}

impl<'g,'v,GH> NumEdges for LargestBiconnectedComponents<'g,'v,GH> 
where GH: BccGraphHashInterface
{

    fn num_edges(&self) -> usize {
        self.bcc_vec[self.lbcc_i as usize].num_edges()
    }
}

impl<'g,'v,GH> LargestBiconnectedComponents<'g,'v,GH> 
where GH: BccGraphHashInterface
{
    pub fn new(bcc_vec: &'v Vec<GH>, graph: &'g Graph<GH>) -> Self {

        let mut lbcc_i: usize = 0; // index of the lbcc, # of nodes
        let mut lbcc_j: usize = 0; // index of the lbcc, # of edges
        let mut lbcc_n: usize = 0; // highest # of nodes
        let mut lbcc_m: usize = 0; // highest # of edges

        if bcc_vec.len() > 1 {

            for i in 0..bcc_vec.len() {

                if bcc_vec[i].num_nodes() > lbcc_n {

                    lbcc_n = bcc_vec[i].num_nodes();

                    lbcc_i = i;
                }

                if bcc_vec[i].num_edges() > lbcc_m {

                    lbcc_m = bcc_vec[i].num_edges();

                    lbcc_j = i;
                }
            }
        }

        Self {
            graph,
            bcc_vec,
            lbcc_i,
            lbcc_j,
            lbcc_n,
            lbcc_m,
        }
    }

    pub fn frac_nodes(&self) -> f64 {
        self.num_nodes() as f64 / self.graph.num_nodes() as f64
    }

    pub fn frac_edges(&self) -> f64 {
        self.num_edges() as f64 / self.graph.num_edges() as f64
    }
}

/**
  | finds the fraction of nodes and edges
  | in the largest bcc, largest bcc in terms
  | of number of nodes
  |
  */
pub fn lbcc_stat<GH>(graph: &mut Graph<GH>) 
-> Result<(),BetweennessCentralityError>  
where GH: BccGraphHashInterface 
{
    graph.print_header();

    let mut bcc_vec: Vec<GH> = vec![];

    graph.find_bicon_component(&mut bcc_vec);

    let stat = LargestBiconnectedComponents::new(&bcc_vec,&graph);

    debug!("{:#?}", stat);

    Ok(())
}
