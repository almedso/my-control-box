//! # Random Time Signals
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use my_control_box::time_signal::random::RandomSignal;
//! use control_box::signal::{TimeRange, TimeSignal};
//!
//! fn main () {
//!   let time: Array<f64, Ix1> = TimeRange::default().collect();
//!   let step_fn = RandomSignal::default().set_minimum(2.0).set_maximum(3.0);
//!   let signal: Array<f64, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();
//!   assert!(signal[0] > 2.0);
//!   assert!(signal[0] < 3.0);
//! }
//! ```

pub use super::*;



use rand::prelude::*;
use rand::distr::Uniform;
use rand::distr::uniform::SampleUniform;

use num_traits::{Num, one, zero};

use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomSignal<S: Debug + Display + Clone + Copy + PartialEq  + SampleUniform> {
    pub minimum: S,
    pub maximum: S,
}

impl<S: Debug + Display + Clone + Copy + PartialEq  + SampleUniform>  RandomSignal<S> {
    pub fn set_minimum(self, minimum: S) -> Self {
        RandomSignal::<S> {minimum, ..self }
    }

    pub fn set_maximum(self, maximum: S) -> Self {
        RandomSignal::<S> { maximum, ..self }
    }

}


impl<S: Debug + Display + Clone + Copy + PartialEq  + SampleUniform + Num> Default for RandomSignal<S> {
    fn default() -> Self {
        RandomSignal::<S> {
            minimum: zero(),
            maximum: one(),
        }
    }
}

impl <S: Debug + Display + Clone + Copy + PartialEq  + SampleUniform + 'static> TimeSignal<S> for RandomSignal<S>
{
    fn time_to_signal(&self, _time: f64) -> S {
        let mut rnd = rand::rng();
        let uniform = Uniform::new(self.minimum, self.maximum).expect("Invalid range for random signal");
        let choice: S = uniform.sample(&mut rnd);
        choice
    }

    fn short_type_name(&self) -> &'static str {
        "Random"
    }
}

impl<S: Debug + Display + Clone + Copy + PartialEq  + SampleUniform + 'static> fmt::Display for RandomSignal<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}(minimum={}, maximum={}",
            self.short_type_name(),self.minimum, self.maximum,
        )
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_random_build() {
        let sut = RandomSignal::<f64>::default()
            .set_minimum(2.0)
            .set_maximum(3.0);
        let expected = RandomSignal::<f64> {
            minimum: 2.0,
            maximum: 3.0,
        };
        assert_eq!(expected, sut)
    }

}
