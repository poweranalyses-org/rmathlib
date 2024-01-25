//! Chebyshev functions.
//!
//! ## DESCRIPTION
//!
//! - `chebyshev_init` determines the number of terms for the
//!   double precision orthogonal series `dos` needed to insure
//!   the error is no larger than `eta`.  Ordinarily eta will be
//!   chosen to be one-tenth machine precision.
//! - `chebyshev_eval` evaluates the n-term Chebyshev series
//!   `a` at `x`.
//!
//! ## NOTES
//!
//! These routines are translations into C of Fortran routines
//! by W. Fullerton of Los Alamos Scientific Laboratory.
//! Based on the Fortran routine dcsevl by W. Fullerton.
//! Adapted from R. Broucke, Algorithm 446, CACM., 16, 254 (1973).

use crate::ml_warn_return_nan;

/// `chebyshev_init` determines the number of terms for the
/// double precision orthogonal series `dos` needed to insure
/// the error is no larger than `eta`.  Ordinarily eta will be
/// chosen to be one-tenth machine precision.
pub fn chebyshev_init(dos: &[f64], eta: f64) -> Option<usize> {
    if dos.is_empty() {
        return None;
    }

    let mut err = 0.0;
    for (i, &d) in dos.iter().enumerate().rev() {
        err += d.abs();
        if err > eta {
            return Some(i);
        }
    }
    Some(0)
}

/// `chebyshev_eval` evaluates the n-term Chebyshev series
/// `a` at `x`.
pub fn chebyshev_eval(x: f64, a: &[f64], n: usize) -> f64 {
    if n < 1 || n > a.len() || n > 1000 {
        return ml_warn_return_nan();
    }

    if !(-1.1..=1.1).contains(&x) {
        return ml_warn_return_nan();
    }

    let twox = x * 2.0;
    let (mut b2, mut b1, mut b0) = (0.0, 0.0, 0.0);

    for &coeff in a.iter().take(n).rev() {
        b2 = b1;
        b1 = b0;
        b0 = twox * b1 - b2 + coeff;
    }

    (b0 - b2) * 0.5
}