use crate::pnorm::pnorm5;

use std::f64::consts::{LN_2, PI, SQRT_2};

pub const M_PI: f64 = PI;
pub const M_SQRT2: f64 = SQRT_2;
pub const M_SQRT_32: f64 = 5.656_854_249_492_381;
pub const M_1_SQRT_2PI: f64 = 0.398_942_280_401_432_7; // 1/sqrt(2pi)
pub const ML_LN2: f64 = LN_2;


pub fn pnorm(x: f64, mu: f64, sigma: f64, lower_tail: bool, log_p: bool) -> f64 {
    pnorm5(x, mu, sigma, lower_tail, log_p)
}
