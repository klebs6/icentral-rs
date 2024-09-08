crate::ix!();

//-------------------------------------------[icentral/src/utility.cc]
//-------------------------------------------[icentral/src/experiments.h]
#[derive(Debug)]
pub struct SimpleStats {
    pub mean:   Duration,
    pub stddev: Duration,
    pub median: Duration,
    pub min:    Duration,
    pub max:    Duration,
}

impl From<&mut Vec<Duration>> for SimpleStats {

    fn from(x: &mut Vec<Duration>) -> Self {

        //x.sort_by(|a,b| FloatOrd(*a).cmp(&FloatOrd(*b)));
        x.sort();

        let n = x.len();

        let min = x[0];
        let max = *x.iter().last().unwrap();

        let sum: Duration = x.iter().sum();

        let mean   = sum / n as u32;
        let median = x[n / 2];

        let mut sum_2: f64 = 0.0;

        for i in 0..n {

            sum_2 += {

                let t0 = x[i] - mean;
                let t1 = x[i] - mean;

                t0.as_secs_f64() * t1.as_secs_f64()
            };
        }

        let stddev = Duration::from_secs_f64((sum_2 / n as f64).sqrt());

        Self { min, max, mean, median, stddev }
    }
}



