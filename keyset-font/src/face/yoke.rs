use rustybuzz::ttf_parser::Face as TtfpFace;
use rustybuzz::Face as RbFace;
use yoke::{Yoke, Yokeable};

/// Wrapper for [`rustybuzz::Face`] that allows us to derive [`Yokeable`]
#[derive(Yokeable, Clone)]
pub struct RbFaceWrapper<'a>(RbFace<'a>);

/// Type alias for Yoke with our yokeable and backing cart
pub type RbFaceYoke = Yoke<RbFaceWrapper<'static>, Box<[u8]>>;

impl<'a> AsRef<TtfpFace<'a>> for RbFaceWrapper<'a> {
    #[inline]
    fn as_ref(&self) -> &TtfpFace<'a> {
        &self.0
    }
}

impl<'a> AsMut<TtfpFace<'a>> for RbFaceWrapper<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut TtfpFace<'a> {
        &mut self.0
    }
}

impl<'a> AsRef<RbFace<'a>> for RbFaceWrapper<'a> {
    #[inline]
    fn as_ref(&self) -> &RbFace<'a> {
        &self.0
    }
}

impl<'a> AsMut<RbFace<'a>> for RbFaceWrapper<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut RbFace<'a> {
        &mut self.0
    }
}

impl<'a> core::ops::Deref for RbFaceWrapper<'a> {
    type Target = RbFace<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for RbFaceWrapper<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> RbFaceWrapper<'a> {
    /// Creates a new [`RbFace`] from a [`TtfpFace`]
    pub fn from_face(face: TtfpFace<'a>) -> Self {
        Self(rustybuzz::Face::from_face(face))
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use std::ops::{Deref as _, DerefMut as _};

    use super::*;

    #[test]
    #[expect(clippy::explicit_deref_methods, reason = "we want to test them")]
    fn wrapper_ref_deref() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let mut wrapper = RbFaceWrapper(RbFace::from_slice(&data, 0).unwrap());

        let _: &TtfpFace = wrapper.as_ref();
        let _: &mut TtfpFace = wrapper.as_mut();
        let _: &RbFace = wrapper.as_ref();
        let _: &mut RbFace = wrapper.as_mut();
        let _: &RbFace = wrapper.deref();
        let _: &mut RbFace = wrapper.deref_mut();
    }

    #[test]
    fn wrapper_from_face() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = TtfpFace::parse(&data, 0).unwrap();

        let _wrapper = RbFaceWrapper::from_face(face);
    }
}
