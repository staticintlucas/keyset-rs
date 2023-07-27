mod color;

pub(crate) use color::Color;

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
