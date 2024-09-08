crate::ix!();

pub type BCMaybeSuccess = Result<(),BetweennessCentralityError>;

error_tree!{

    pub enum BetweennessCentralityError {

        DataMismatch {
            msg: String,
        },

        DuplicateEdgeInsertion {
            edge: Edge,
        },

        DataMismatches {
            mismatches: Vec<f64>,
        },

        NoKey {
            key: usize 
        },

        Poisoned {
            msg: String 
        },

        TryFromIntError(std::num::TryFromIntError),
        IoError(std::io::Error),
        TextIoError(text_io::Error),
        ParseIntError(ParseIntError),
        NoMPI,
        LockError {
            msg: String,
        },
    }

    pub enum ParallelICentralError {
        Io(std::io::Error),
        TryFromIntError(std::num::TryFromIntError),
    }
}

impl BetweennessCentralityError {

    pub fn no_key(x: usize) -> Self {
        BetweennessCentralityError::NoKey { key: x }
    }

    pub fn mismatch_diff(diff: f64, msg: Option<&str>) -> Self {

        BetweennessCentralityError::DataMismatch { 
            msg: format!("data mismatch diff: {}, msg: {:?}", diff, msg) 
        }
    }
}

pub trait PoisonMessage {

    fn poison_message(&self) -> String;
}

impl<'a, T: PoisonMessage> 
    From<std::sync::PoisonError<MutexGuard<'a,T>>> 
    for BetweennessCentralityError 
{
    fn from(x: std::sync::PoisonError<MutexGuard<'a,T>>) -> Self {

        BetweennessCentralityError::Poisoned { 
            msg: x.into_inner().poison_message() 
        }
    }
}

impl<T: core::fmt::Debug> PoisonMessage for T {
    fn poison_message(&self) -> String {
        format!("poisoned {:?}!",self)
    }
}

pub type BCError = BetweennessCentralityError;

