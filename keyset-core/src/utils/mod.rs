mod round_rect;

pub(crate) use round_rect::RoundRect;

// Shim to allow us to use assert_approx_eq with Kurbo types
#[cfg(test)]
mod kurbo_shim {
    pub trait KurboAbs {
        fn abs(&self) -> f64;
    }

    impl KurboAbs for kurbo::Vec2 {
        fn abs(&self) -> f64 {
            self.length()
        }
    }

    impl KurboAbs for kurbo::Size {
        fn abs(&self) -> f64 {
            self.to_vec2().length()
        }
    }
}

#[cfg(test)]
pub(crate) use kurbo_shim::*;

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use assert_approx_eq::assert_approx_eq;
    use kurbo::{Size, Vec2};

    use super::*;

    #[test]
    fn test_kurbo_shim() {
        let vec = Vec2::new(1., 1.);
        let size = Size::new(1., 1.);

        assert_approx_eq!(vec.abs(), SQRT_2);
        assert_approx_eq!(size.abs(), SQRT_2);
    }
}
