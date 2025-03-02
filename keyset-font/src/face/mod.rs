// TODO these warnings originate from within ouroboros
#![allow(clippy::future_not_send, clippy::redundant_pub_crate)]

mod mac_roman;

use std::fmt;
use std::sync::Arc;

use ouroboros::self_referencing;
use rustybuzz::ttf_parser::{self, GlyphId};

use geom::{PathBuilder, Point, Vector};

use self::mac_roman::{is_mac_roman_encoding, mac_roman_decode};
use crate::error::PermissionError;
use crate::{FontUnit, Result};

#[self_referencing]
pub struct Face {
    data: Arc<[u8]>,
    #[borrows(data)]
    #[covariant]
    inner: rustybuzz::Face<'this>,
}

impl Clone for Face {
    fn clone(&self) -> Self {
        FaceBuilder {
            data: self.borrow_data().clone(),
            inner_builder: |data| {
                rustybuzz::Face::from_slice(data, 0)
                    .unwrap_or_else(|| unreachable!("face was already parsed"))
            },
        }
        .build()
    }
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.borrow_inner().fmt(f)
    }
}

impl Face {
    pub fn from_ttf(data: Vec<u8>) -> Result<Self> {
        FaceTryBuilder {
            data: data.into_boxed_slice().into(),
            inner_builder: |data| {
                let face = ttf_parser::Face::parse(data, 0)?;

                // Check font permissions (if set). Use getters on the OS/2 table rather than on
                // Face since Face defaults to not allowed if there is no OS/2 table. We assume the
                // user has read the font's license and knows what they're doing unless the font
                // tells us otherwise.
                if let Some(os2) = face.tables().os2 {
                    if matches!(os2.permissions(), Some(ttf_parser::Permissions::Restricted)) {
                        return Err(PermissionError::RestrictedLicense.into());
                    } else if !os2.is_subsetting_allowed() {
                        return Err(PermissionError::NoSubsetting.into());
                    } else if !os2.is_outline_embedding_allowed() {
                        return Err(PermissionError::BitmapEmbeddingOnly.into());
                    }
                }

                Ok(rustybuzz::Face::from_face(face))
            },
        }
        .try_build()
    }

    pub fn inner(&self) -> &rustybuzz::Face<'_> {
        self.borrow_inner()
    }

    pub fn names(&self) -> ttf_parser::name::Names<'_> {
        self.borrow_inner().names()
    }

    // TODO: can we delete these altogether? Or should we expose these in the public API
    // pub fn is_regular(&self) -> bool {
    //     self.borrow_inner().is_regular()
    // }

    // pub fn is_italic(&self) -> bool {
    //     self.borrow_inner().is_italic()
    // }

    // pub fn is_bold(&self) -> bool {
    //     self.borrow_inner().is_bold()
    // }

    // pub fn is_oblique(&self) -> bool {
    //     self.borrow_inner().is_oblique()
    // }

    // pub fn style(&self) -> ttf_parser::Style {
    //     self.borrow_inner().style()
    // }

    // pub fn is_monospaced(&self) -> bool {
    //     self.borrow_inner().is_monospaced()
    // }

    // pub fn is_variable(&self) -> bool {
    //     self.borrow_inner().is_variable()
    // }

    // pub fn weight(&self) -> ttf_parser::Weight {
    //     self.borrow_inner().weight()
    // }

    // pub fn width(&self) -> ttf_parser::Width {
    //     self.borrow_inner().width()
    // }

    pub fn italic_angle(&self) -> f32 {
        self.borrow_inner().italic_angle()
    }

    pub fn ascender(&self) -> i16 {
        self.borrow_inner().ascender()
    }

    pub fn descender(&self) -> i16 {
        self.borrow_inner().descender()
    }

    // pub fn height(&self) -> i16 {
    //     self.borrow_inner().height()
    // }

    pub fn line_gap(&self) -> i16 {
        self.borrow_inner().line_gap()
    }

    pub fn units_per_em(&self) -> u16 {
        self.borrow_inner().as_ref().units_per_em()
    }

    pub fn x_height(&self) -> Option<i16> {
        self.borrow_inner().x_height()
    }

    pub fn capital_height(&self) -> Option<i16> {
        self.borrow_inner().capital_height()
    }

    pub fn number_of_glyphs(&self) -> u16 {
        self.borrow_inner().number_of_glyphs()
    }

    pub fn glyph_index(&self, code_point: char) -> Option<u16> {
        self.borrow_inner().glyph_index(code_point).map(|gid| gid.0)
    }

    pub fn outline_length(&self, glyph_id: u16) -> usize {
        struct LengthBuilder(usize);
        impl ttf_parser::OutlineBuilder for LengthBuilder {
            fn move_to(&mut self, _x: f32, _y: f32) {
                self.0 += 1;
            }

            fn line_to(&mut self, _x: f32, _y: f32) {
                self.0 += 1;
            }

            fn quad_to(&mut self, _x1: f32, _y1: f32, _x: f32, _y: f32) {
                self.0 += 1;
            }

            fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
                self.0 += 1;
            }

            fn close(&mut self) {
                self.0 += 1;
            }
        }

        let mut builder = LengthBuilder(0);
        let _ = self.inner().outline_glyph(GlyphId(glyph_id), &mut builder);
        builder.0
    }

    pub fn outline_glyph(
        &self,
        glyph_id: u16,
        builder: &mut PathBuilder<FontUnit>,
        offset: Vector<FontUnit>,
    ) {
        struct OutlineBuilder<'a> {
            builder: &'a mut PathBuilder<FontUnit>,
            offset: Vector<FontUnit>,
        }

        impl ttf_parser::OutlineBuilder for OutlineBuilder<'_> {
            fn move_to(&mut self, x: f32, y: f32) {
                self.builder.abs_move(Point::new(x, y) + self.offset);
            }

            fn line_to(&mut self, x: f32, y: f32) {
                self.builder.abs_line(Point::new(x, y) + self.offset);
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.builder.abs_quadratic_bezier(
                    Point::new(x1, y1) + self.offset,
                    Point::new(x, y) + self.offset,
                );
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.builder.abs_cubic_bezier(
                    Point::new(x1, y1) + self.offset,
                    Point::new(x2, y2) + self.offset,
                    Point::new(x, y) + self.offset,
                );
            }

            fn close(&mut self) {
                self.builder.close();
            }
        }

        let mut builder = OutlineBuilder { builder, offset };
        // outline_glyph returns None if there is no outline (e.g. for space)
        let _ = self.inner().outline_glyph(GlyphId(glyph_id), &mut builder);
    }

    // Utility function to get bounds for the glyph for a given char. Returns none if there is no
    // glyph in the font for the given character, or the glyph has no bounding box
    pub(crate) fn glyph_bounds(&self, glyph_id: u16) -> Option<ttf_parser::Rect> {
        self.inner().glyph_bounding_box(GlyphId(glyph_id))
    }

    // Finds a string for the given name id in the names table
    pub(crate) fn name(&self, name_id: u16) -> Option<String> {
        let mut names = self.names().into_iter().filter(|n| n.name_id == name_id);

        names.clone().find_map(|n| n.to_string()).or_else(|| {
            names
                .find(|n| is_mac_roman_encoding(n.platform_id, n.encoding_id, n.language_id))
                .map(|name| mac_roman_decode(name.name))
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use isclose::assert_is_close;
    use ttf_parser::name_id;

    use super::*;

    #[test]
    fn face_clone() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        #[allow(clippy::redundant_clone)] // We want to test clone
        let face2 = face.clone();

        assert_eq!(face.borrow_data(), face2.borrow_data());
        assert_eq!(
            face.borrow_inner().number_of_glyphs(),
            face2.borrow_inner().number_of_glyphs()
        );
    }

    #[test]
    fn face_debug() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(format!("{face:?}"), "Face()");
    }

    #[test]
    fn face_from_ttf() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let cmap = face.borrow_inner().tables().cmap.unwrap();
        let kern = face.borrow_inner().tables().kern.unwrap();
        assert_eq!(cmap.subtables.len(), 1);
        assert_eq!(kern.subtables.len(), 1);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert!(face.borrow_inner().tables().cmap.is_none());
        assert!(face.borrow_inner().tables().kern.is_none());
    }

    #[test]
    fn face_permissions() {
        use crate::error::Error::PermissionError;
        use crate::error::PermissionError::{BitmapEmbeddingOnly, NoSubsetting, RestrictedLicense};

        let null = Face::from_ttf(std::fs::read(env!("NULL_TTF")).unwrap());
        assert!(null.is_ok()); // no OS/2 table

        let demo = Face::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap());
        assert!(demo.is_ok()); // permissive permissions in OS/2

        let restricted = Face::from_ttf(std::fs::read(env!("RESTRICTED_TTF")).unwrap());
        assert!(restricted.is_err());
        let err = restricted.unwrap_err();
        assert_matches!(err, PermissionError(RestrictedLicense));

        let no_subsetting = Face::from_ttf(std::fs::read(env!("NO_SUBSET_TTF")).unwrap());
        assert!(no_subsetting.is_err());
        let err = no_subsetting.unwrap_err();
        assert_matches!(err, PermissionError(NoSubsetting));

        let bitmap_embedding_only =
            Face::from_ttf(std::fs::read(env!("BITMAP_EMBED_ONLY_TTF")).unwrap());
        assert!(bitmap_embedding_only.is_err());
        let err = bitmap_embedding_only.unwrap_err();
        assert_matches!(err, PermissionError(BitmapEmbeddingOnly));
    }

    #[test]
    fn face_properties() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(face.names().len(), 4);
        // assert!(face.is_regular());
        // assert!(!face.is_italic());
        // assert!(!face.is_bold());
        // assert!(!face.is_oblique());
        // assert_eq!(face.style(), Style::Normal);
        // assert!(!face.is_monospaced());
        // assert!(!face.is_variable());
        // assert_eq!(face.weight(), Weight::Normal);
        // assert_eq!(face.width(), Width::Normal);
        assert_is_close!(face.italic_angle(), 0.0);
        assert_eq!(face.ascender(), 1024);
        assert_eq!(face.descender(), -400);
        // assert_eq!(face.height(), 1424);
        assert_eq!(face.line_gap(), 0);
        assert_eq!(face.units_per_em(), 1000);
        assert_eq!(face.x_height(), Some(450));
        assert_eq!(face.capital_height(), Some(650));
        assert_eq!(face.number_of_glyphs(), 3);
    }

    #[test]
    fn face_glyph_index() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(face.glyph_index('A').unwrap(), 1);
        assert_eq!(face.glyph_index('V').unwrap(), 2);
        assert!(face.glyph_index('P').is_none());
    }

    #[test]
    #[allow(non_snake_case)]
    fn face_outline_length() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let glyph_A = face.glyph_index('A').unwrap();
        let glyph_V = face.glyph_index('V').unwrap();
        let glyph_notdef = 0;

        assert_eq!(face.outline_length(glyph_A), 15);
        assert_eq!(face.outline_length(glyph_V), 9);
        assert_eq!(face.outline_length(glyph_notdef), 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn face_outline_glyph() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let glyph_A = face.glyph_index('A').unwrap();
        let glyph_V = face.glyph_index('V').unwrap();
        let glyph_notdef = 0;

        let mut outline_A = PathBuilder::new();
        let mut outline_V = PathBuilder::new();
        let mut outline_notdef = PathBuilder::new();

        face.outline_glyph(glyph_A, &mut outline_A, Vector::zero());
        face.outline_glyph(glyph_V, &mut outline_V, Vector::zero());
        face.outline_glyph(glyph_notdef, &mut outline_notdef, Vector::zero());

        let outline_A = outline_A.build();
        let outline_V = outline_V.build();
        let outline_notdef = outline_notdef.build();

        assert_eq!(outline_A.len(), 15);
        assert_eq!(outline_V.len(), 9);
        assert_eq!(outline_notdef.len(), 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn face_glyph_bounds() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        let glyph_A = face.glyph_index('A').unwrap();
        let glyph_V = face.glyph_index('V').unwrap();
        let glyph_notdef = 0;

        assert_eq!(face.glyph_bounds(glyph_A).unwrap().height(), 656);
        assert_eq!(face.glyph_bounds(glyph_A).unwrap().width(), 535);
        assert_eq!(face.glyph_bounds(glyph_V).unwrap().height(), 656);
        assert_eq!(face.glyph_bounds(glyph_V).unwrap().width(), 535);
        assert_eq!(face.glyph_bounds(glyph_notdef).unwrap().height(), 700);
        assert_eq!(face.glyph_bounds(glyph_notdef).unwrap().width(), 500);
    }

    #[test]
    #[allow(non_snake_case)]
    fn face_name() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(face.name(name_id::FAMILY).unwrap(), "demo");
        assert_eq!(face.name(name_id::FULL_NAME).unwrap(), "demo regular");
    }
}
