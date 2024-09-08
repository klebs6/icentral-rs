crate::ix!();

#[derive(Debug)]
pub struct SpeedupStats {
    pub mean:   f64,
    pub stddev: f64,
    pub median: f64,
    pub min:    f64,
    pub max:    f64,
}

impl From<&mut Vec<f64>> for SpeedupStats {

    fn from(x: &mut Vec<f64>) -> Self {

        x.sort_by(|a,b| FloatOrd(*a).cmp(&FloatOrd(*b)));

        let n = x.len();

        let min = x[0];
        let max = *x.iter().last().unwrap();

        let sum: f64 = x.iter().sum();

        let mean   = sum / n as f64;
        let median = x[n / 2];

        let mut sum_2: f64 = 0.0;

        for i in 0..n {

            sum_2 += {

                let t0 = x[i] - mean;
                let t1 = x[i] - mean;

                t0 * t1
            };
        }

        let stddev = (sum_2 / n as f64).sqrt();

        Self { min, max, mean, median, stddev }
    }
}
