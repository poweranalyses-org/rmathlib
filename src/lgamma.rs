//!
//!  Mathlib : A C Library of Special Functions
//!  Copyright (C) 2000-2020 The R Core Team
//!  Copyright (C) 1998 Ross Ihaka
//!
//!  This program is free software; you can redistribute it and/or modify
//!  it under the terms of the GNU General Public License as published by
//!  the Free Software Foundation; either version 2 of the License, or
//!  (at your option) any later version.
//!
//!  This program is distributed in the hope that it will be useful,
//!  but WITHOUT ANY WARRANTY; without even the implied warranty of
//!  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!  GNU General Public License for more details.
//! 
//!  You should have received a copy of the GNU General Public License
//!  along with this program; if not, a copy is available at
//!  https://www.R-project.org/Licenses/
//!
//!  SYNOPSIS
//!
//!    #include <Rmath.h>
//!    double lgammafn_sign(double x, int *sgn);
//!    double lgammafn(double x);
//! 
//!  DESCRIPTION
//! 
//!    The function lgammafn computes log|gamma(x)|.  The function
//!    lgammafn_sign in addition assigns the sign of the gamma function
//!    to the address in the second argument if this is not NULL.
//! 
//!  NOTES
//! 
//!    This routine is a translation into C of a Fortran subroutine
//!    by W. Fullerton of Los Alamos Scientific Laboratory.
//! 
//!    The accuracy of this routine compares (very) favourably
//!    with those of the Sun Microsystems portable mathematical
//!    library.
//! 
//!  ./toms708.c  has  gamln()

use crate::dpq::*;
use crate::libc::*;
use crate::nmath::*;
use crate::rmath::*;
use crate::d1mach::*;
use libm::*;

fn lgammafn_sign(x: f64, sgn: &mut i32) -> f64 {
    let ans: f64;
    let y: f64;
    let sinpiy: f64;

    d1mach(2)
}
