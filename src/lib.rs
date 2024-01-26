//! A Rust port of R's C Library of Special Functions.

// Avoid converting `if n < 1 || n > 1000` to `if !(1..=1000).contains(&n)`.
#![allow(clippy::manual_range_contains)]

mod chebyshev;
mod cospi;
mod dnorm;
// mod dpois;
mod dpq;
mod gamma;
mod lgamma;
mod lgammacor;
mod libc;
mod nmath;
// mod pgamma;
mod i1mach;
mod pnorm;
mod qnorm;
mod rmath;
mod stirlerr;

// Use only explicit exports and no wildcard exports to avoid accidentally
// exporting symbols that should not be exported.
pub use chebyshev::chebyshev_eval;
pub use chebyshev::chebyshev_init;
pub use cospi::cospi;
pub use cospi::sinpi;
pub use cospi::tanpi;
// pub use dpois::*;
pub use gamma::gammafn;
pub use lgamma::lgammafn;
pub use lgamma::lgammafn_sign;
pub use lgammacor::lgammacor;
pub use rmath::dnorm;
pub use rmath::pnorm;
pub use rmath::qnorm;
// pub use pgamma::*;
pub use i1mach::i1mach;
pub use stirlerr::stirlerr;

// TODO: Remove these exports later; once they are used by lgamma.
pub use nmath::r_d_nonint_check;
pub use nmath::r_log1_exp;
pub use nmath::r_nonint;