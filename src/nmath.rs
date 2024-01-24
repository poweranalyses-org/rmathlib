use std::f64::{consts::LN_2, EPSILON, INFINITY, MAX, MIN, NAN, NEG_INFINITY};

pub const ML_POSINF: f64 = INFINITY;
pub const ML_NEGINF: f64 = NEG_INFINITY;
pub const ML_DBL_MAX: f64 = MAX;
pub const ML_DBL_MIN: f64 = MIN;
pub const ML_DBL_EPSILON: f64 = EPSILON;

pub fn ml_warn_return_nan() -> f64 {
    println!("argument out of domain");
    ML_NAN
}

pub fn r_finite(x: f64) -> bool {
    x.is_finite()
}

pub const ML_NAN: f64 = NAN;

/// log(sqrt(2*pi)) == log(2*pi)/2
pub const M_LN_SQRT_2PI: f64 = 0.918_938_533_204_672_8;

/// log(1 - exp(x)) in more stable form than log1p(- r_d_qiv(x)) :
pub fn r_log1_exp(x: f64) -> f64 {
    if x > -LN_2 {
        (-x.exp_m1()).ln()
    } else {
        (-x.exp()).ln_1p()
    }
}
