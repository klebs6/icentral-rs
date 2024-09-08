use icentral_graph_hash::*;
use icentral_alg::*;
use icentral_deps::*;

//-------------------------------------------[icentral/src/main.cc]

pub fn main() -> Result<(),()> {

    let mut i: i32 = 0;

    let mut status: MPI_Status = unsafe { std::mem::zeroed() };

    let mut str_message: [i8; 100] = [0; 100];

    let universe = mpi::initialize().unwrap();
    let world    = universe.world();
    let size     = world.size();
    let rank     = world.rank();

    //    sprintf(str_message, "Hello world from process %d of %d", rank, size);
    //    int cnt=strlen(str_message)+1;
    //    if(rank!=0) MPI_Send(str_message, cnt, MPI_CHAR, 0, 0, MPI_COMM_WORLD);
    //    if(rank==0)
    //    {
    //        for(i=1;i<size;i++)
    //        {
    //            MPI_Recv(str_message, 100, MPI_CHAR, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD, &status);
    //
    //            // If we need to receive messages in order, we should do this instead:
    //            //MPI_Recv(str_message, 100, MPI_CHAR, i, MPI_ANY_TAG, MPI_COMM_WORLD, &status);
    //
    //            printf("%s", str_message);
    //        }
    //    }

    //    if(rank == 0) {
    //        vector<double> vv;
    //        vv.push_back(10);
    //        vv.push_back(20);
    //        MPI_Send(&vv[0], vv.len(), MPI_DOUBLE, 1, 0, MPI_COMM_WORLD);
    //        
    //        vector<pair<unsigned int, unsigned int> > vvp;
    //        vvp.push_back(make_pair(20, 17));
    //        vvp.push_back(make_pair(25, 27));
    //        MPI_Send(&vvp[0], vvp.len()*sizeof(pair<unsigned int, unsigned int>), MPI_CHAR, 1, 0, MPI_COMM_WORLD);
    //    }
    //    if(rank == 1) {
    //        vector<double> vv;
    //        vv.resize(2);
    //        MPI_Recv(&vv[0], vv.len(), MPI_DOUBLE, 0, MPI_ANY_TAG, MPI_COMM_WORLD, &status);
    //        printf("%f, %f", vv[0], vv[1]);
    //        vector<pair<unsigned int, unsigned int> > vvp;
    //        vvp.resize(2);
    //        MPI_Recv(&vvp[0], vvp.len()*sizeof(pair<unsigned int, unsigned int>), MPI_CHAR, 0, MPI_ANY_TAG, MPI_COMM_WORLD, &status);
    //        printf("%u %u %u %u", vvp[0].first, vvp[0].second, vvp[1].first, vvp[1].second);
    //        
    //    }

    kdd_exp_main::<GraphHash>(rank, size, &world);

   Ok(()) 
}
