#[cfg(test)]
mod test_math {
    use approx::abs_diff_eq;
    use rmathlib::*;

    mod c {
        extern "C" {
            pub fn cospi(x: f64) -> f64;
            pub fn sinpi(x: f64) -> f64;
            pub fn tanpi(x: f64) -> f64;
            pub fn pnorm5(x: f64, mu: f64, sigma: f64, lower_tail: i32, log_p: i32) -> f64;
            pub fn qnorm5(p: f64, mu: f64, sigma: f64, lower_tail: i32, log_p: i32) -> f64;
            // pub fn pgamma(x: f64, alph: f64, scale: f64, lower_tail: bool, log_p: bool) -> f64; // TODO: tests
            pub fn lgammafn(x: f64) -> f64;
            pub fn lgammafn_sign(x: f64, sgn: Option<&mut i32>) -> f64;
            pub fn gammafn(x: f64) -> f64;
            // pub fn lgammacor(x: f64) -> f64; // FIXME: Linking in CC broken
            // pub fn chebyshev_init(dos: *mut f64, nos: i32, eta: f64) -> i32; // FIXME: Linking in CC broken
            // pub fn chebyshev_eval(x: f64, a: *mut f64, n: i32) -> f64; // FIXME: Linking in CC broken
            pub fn dnorm4(x: f64, mu: f64, sigma: f64, give_log: bool) -> f64;
            pub fn stirlerr(n: f64) -> f64;
        }
    }

    #[test]
    fn test_cospi() {
        assert_eq!(cospi(0.0), unsafe { c::cospi(0.0) });
        assert_eq!(cospi(0.234), unsafe { c::cospi(0.234) });
        assert_eq!(sinpi(0.0), unsafe { c::sinpi(0.0) });
        assert_eq!(sinpi(0.234), unsafe { c::sinpi(0.234) });
        assert_eq!(tanpi(0.0), unsafe { c::tanpi(0.0) });
        assert_eq!(tanpi(0.25), unsafe { c::tanpi(0.25) });
        assert_eq!(tanpi(0.234), unsafe { c::tanpi(0.234) });
    }

    #[test]
    fn test_pnorm() {
        assert_eq!(pnorm(0.0, 0.0, 1.0, true, false), 0.5);
        assert_eq!(pnorm(0.0, 0.0, 1.0, false, false), 0.5);
        assert_eq!(pnorm(0.0, 0.0, 1.0, false, false), unsafe {
            c::pnorm5(0.0, 0.0, 1.0, 0, 0)
        });
        assert_eq!(pnorm(0.65, 0.2, 0.34, false, false), unsafe {
            c::pnorm5(0.65, 0.2, 0.34, 0, 0)
        });
        assert_eq!(pnorm(3.21, 0.2, 0.34, false, false), unsafe {
            c::pnorm5(3.21, 0.2, 0.34, 0, 0)
        });
        assert_eq!(pnorm(3.21, 0.2, 0.34, false, true), unsafe {
            c::pnorm5(3.21, 0.2, 0.34, 0, 1)
        });
        assert_eq!(pnorm(123.0, 0.2, 0.34, false, true), unsafe {
            c::pnorm5(123.0, 0.2, 0.34, 0, 1)
        });
    }

    #[test]
    fn test_qnorm() {
        assert_eq!(qnorm5(0.0, 0.5, 1.0, true, false), unsafe {
            c::qnorm5(0.0, 0.5, 1.0, 1, 0)
        });
        assert_eq!(qnorm5(0.4, 0.5, 1.0, true, false), unsafe {
            c::qnorm5(0.4, 0.5, 1.0, 1, 0)
        });
        assert_eq!(qnorm5(0.4, 0.5, 1.0, false, false), unsafe {
            c::qnorm5(0.4, 0.5, 1.0, 0, 0)
        });
        assert!(abs_diff_eq!(
            qnorm5(-2.3, 0.5, 1.0, false, true),
            unsafe { c::qnorm5(-2.3, 0.5, 1.0, 0, 1) },
            epsilon = 1e-15
        ));
    }

    #[test]
    fn test_lgammafn() {
        assert_eq!(lgammafn(0.0), unsafe { c::lgammafn(0.0) });
        assert_eq!(lgammafn(-1.0), unsafe { c::lgammafn(-1.0) });
        assert_eq!(lgammafn(1.0), unsafe { c::lgammafn(1.0) });
    }

    #[test]
    fn test_lgammafn_sign() {
        assert_eq!(lgammafn_sign(0.0, Some(&mut -1)), unsafe {
            c::lgammafn_sign(0.0, Some(&mut -1))
        });
        assert_eq!(lgammafn_sign(-1.0, Some(&mut -1)), unsafe {
            c::lgammafn_sign(-1.0, Some(&mut -1))
        });
        assert_eq!(lgammafn_sign(1.0, Some(&mut -1)), unsafe {
            c::lgammafn_sign(1.0, Some(&mut -1))
        });
        assert_eq!(lgammafn_sign(0.0, Some(&mut 1)), unsafe {
            c::lgammafn_sign(0.0, Some(&mut 1))
        });
        assert_eq!(lgammafn_sign(-1.0, Some(&mut 1)), unsafe {
            c::lgammafn_sign(-1.0, Some(&mut 1))
        });
        assert_eq!(lgammafn_sign(1.0, Some(&mut 1)), unsafe {
            c::lgammafn_sign(1.0, Some(&mut 1))
        });
    }

    #[test]
    fn test_gammafn() {
        assert!(gammafn(-1.0).is_nan());
        assert!(gammafn(0.0).is_nan());
        assert_eq!(gammafn(0.1), unsafe { c::gammafn(0.1) });
        assert_eq!(gammafn(1.0), unsafe { c::gammafn(1.0) });
    }

    // #[test]
    // fn test_lgammacor() {
    //     assert!(lgammacor(-1.0).is_nan());
    //     assert!(lgammacor(0.0).is_nan());
    //     assert_eq!(lgammacor(10.0), unsafe { c::lgammacor(10.0) });
    //     assert_eq!(lgammacor(20.0), unsafe { c::lgammacor(20.0) });
    //     assert_eq!(lgammacor(3.745194030963158e306 + 1.0), unsafe {
    //         c::lgammacor(3.745194030963158e306 + 1.0)
    //     });
    //     assert_eq!(lgammacor(94906265.62425156 + 1.0), unsafe {
    //         c::lgammacor(94906265.62425156 + 1.0)
    //     });
    // }

    // #[test]
    // fn test_chebyshev_init() {
    //     assert_eq!(chebyshev_init(&[1.0, 2.0, 3.0], 3, 0.5), unsafe {
    //         c::chebyshev_init([1.0, 2.0, 3.0].as_mut_ptr(), 3, 0.5)
    //     });
    //     assert_eq!(chebyshev_init(&[], 0, 0.5), unsafe {
    //         c::chebyshev_init([].as_mut_ptr(), 0, 0.5)
    //     });
    //     assert_eq!(chebyshev_init(&[1.0, 2.0, 3.0], 3, -0.5), unsafe {
    //         c::chebyshev_init([1.0, 2.0, 3.0].as_mut_ptr(), 3, -0.5)
    //     });
    // }

    // #[test]
    // fn test_chebyshev_eval() {
    //     assert_eq!(chebyshev_eval(0.6, &[1.0, 2.0, 3.0], 2), unsafe {
    //         c::chebyshev_eval(0.6, [1.0, 2.0, 3.0].as_mut_ptr(), 2)
    //     });
    //     assert_eq!(chebyshev_eval(0.6, &[1.0, 2.0, 3.0], 0), unsafe {
    //         c::chebyshev_eval(0.6, [1.0, 2.0, 3.0].as_mut_ptr(), 0)
    //     });
    //     assert_eq!(chebyshev_eval(0.6, &[1.0, 2.0, 3.0], 5), unsafe {
    //         c::chebyshev_eval(0.6, [1.0, 2.0, 3.0].as_mut_ptr(), 5)
    //     });
    //     assert_eq!(chebyshev_eval(-0.6, &[1.0, 2.0, 3.0], 2), unsafe {
    //         c::chebyshev_eval(-0.6, [1.0, 2.0, 3.0].as_mut_ptr(), 2)
    //     });
    //     assert_eq!(chebyshev_eval(0.6, &[], 2), unsafe {
    //         c::chebyshev_eval(0.6, [].as_mut_ptr(), 2)
    //     });
    // }

    #[test]
    fn test_dnorm4() {
        assert_eq!(dnorm4(0.0, 0.0, 1.0, false), unsafe {
            c::dnorm4(0.0, 0.0, 1.0, false)
        });
        assert_eq!(dnorm4(0.0, 0.0, 1.0, true), unsafe {
            c::dnorm4(0.0, 0.0, 1.0, true)
        });
        assert_eq!(dnorm4(1.0, 0.0, 1.0, false), unsafe {
            c::dnorm4(1.0, 0.0, 1.0, false)
        });
        assert_eq!(dnorm4(1.0, 0.0, 1.0, true), unsafe {
            c::dnorm4(1.0, 0.0, 1.0, true)
        });
        assert_eq!(dnorm4(-1.0, 0.0, 1.0, false), unsafe {
            c::dnorm4(-1.0, 0.0, 1.0, false)
        });
        assert_eq!(dnorm4(-1.0, 0.0, 1.0, true), unsafe {
            c::dnorm4(-1.0, 0.0, 1.0, true)
        });
    }

    #[test]
    fn test_stilerr() {
        // assert_eq!(stirlerr(1.0), unsafe { c::stirlerr(1.0) });
        // assert_eq!(stirlerr(2.0), unsafe { c::stirlerr(2.0) });
        // assert_eq!(stirlerr(25.0), unsafe { c::stirlerr(25.0) });
        // assert_eq!(stirlerr(50.0), unsafe { c::stirlerr(50.0) });
    }
}
