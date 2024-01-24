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
        assert_eq!(qnorm(0.0, 0.5, 1.0, true, false), unsafe {
            c::qnorm5(0.0, 0.5, 1.0, 1, 0)
        });
        assert_eq!(qnorm(0.4, 0.5, 1.0, true, false), unsafe {
            c::qnorm5(0.4, 0.5, 1.0, 1, 0)
        });
        assert_eq!(qnorm(0.4, 0.5, 1.0, false, false), unsafe {
            c::qnorm5(0.4, 0.5, 1.0, 0, 0)
        });
        abs_diff_eq!(
            qnorm(-2.3, 0.5, 1.0, false, true),
            unsafe { c::qnorm5(-2.3, 0.5, 1.0, 0, 1) },
            epsilon = 1e-10
        );
    }
}
