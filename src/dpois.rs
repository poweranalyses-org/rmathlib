use std::f64::MIN;

use crate::{lgammafn, ml_warn_return_nan, r_d_nonint_check, r_forceint, rmath::M_PI, stirlerr};

#[allow(non_upper_case_globals)]
const x_LRG: f64 = 2.86111748575702815380240589208115399625e+307; // = 2^1023 / pi

/// Computes the Poisson probability lb^x exp(-lb) / x!.
///
/// This does not check that x is an integer, since dgamma() may
/// call this with a fractional x argument. Any necessary argument
/// checks should be done in the calling function.
pub fn dpois_raw(x: f64, lambda: f64, give_log: bool) -> f64 {
    if lambda == 0.0 {
        return if x == 0.0 { 1.0 } else { 0.0 };
    }
    if !lambda.is_finite() {
        return 0.0;
    }
    if x < 0.0 {
        return 0.0;
    }
    if x <= lambda * MIN {
        return (-lambda).exp();
    }
    if lambda < x * MIN {
        if !x.is_finite() {
            return 0.0;
        }
        return (-lambda + x * lambda.ln() - lgammafn(x + 1.0)).exp();
    }

    // TODO: implement ebd0
    let (yh, yl) = ebd0(x, lambda);
    let yl = yl + stirlerr(x);
    let lrg_x = x >= x_LRG;

    let r = if lrg_x {
        (2.0 * M_PI * x).sqrt()
    } else {
        2.0 * M_PI * x
    };

    if give_log {
        -yl - yh - if lrg_x { r.ln() } else { 0.5 * r.ln() }
    } else {
        (-yl).exp() * (-yh).exp() / if lrg_x { r } else { r.sqrt() }
    }
}

/// checks argument validity and calls dpois_raw().
pub fn dpois(x: f64, lambda: f64, give_log: bool) -> f64 {
    if x.is_nan() || lambda.is_nan() {
        return x + lambda;
    }
    if lambda < 0.0 {
        return ml_warn_return_nan();
    }
    r_d_nonint_check(x, give_log);
    if x < 0.0 || !x.is_finite() {
        return 0.0;
    }

    let x = r_forceint(x);

    dpois_raw(x, lambda, give_log)
}
