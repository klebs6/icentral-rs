crate::ix!();

impl<GH> ReadGraph for Graph<GH> 
where GH: BccGraphHashInterface
{
    type Error = BetweennessCentralityError;

    /*
     | #vertices    #edges
     | src1         dst1
     | .
     | .
     | src#edges    dst#edges
     */
    fn read_graph(&mut self, path: &str) 
    -> Result<(),Self::Error>  
    {
        debug!("reading graph from filename: {}", path);

        let fin = File::open(path)?;

        let mut bytes = fin.bytes().map(|ch| ch.unwrap());

        let n: usize = read!("{} ",bytes);
        let m: usize = read!("{}",bytes);

        self.init_size(n);

        for i in 0..m {

            let src: usize = read!("{} ",bytes);
            let dst: usize = read!("{}",bytes);

            self.insert_edge(&Edge::new_with_ids(src,dst))?;
        }

        self.init_internals()?;

        debug!("finished reading graph from file:\n{:#?}", self);

        Ok(())
    }
}
