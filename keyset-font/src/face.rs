// TODO these warnings originate from within ouroboros
#![allow(clippy::future_not_send, clippy::redundant_pub_crate)]

use std::fmt::Debug;

use log::warn;
use ouroboros::self_referencing;
use ttf_parser::GlyphId;

use crate::Result;

#[self_referencing]
pub(crate) struct Face {
    data: Vec<u8>,
    #[borrows(data)]
    #[covariant]
    face: ttf_parser::Face<'this>,
    #[borrows(face)]
    #[covariant]
    cmap: Vec<ttf_parser::cmap::Subtable<'this>>,
    #[borrows(face)]
    #[covariant]
    kern: Vec<ttf_parser::kern::Subtable<'this>>,
}

impl Clone for Face {
    fn clone(&self) -> Self {
        Self::from_ttf(self.borrow_data().clone())
            .unwrap_or_else(|_| unreachable!("face was already parsed"))
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Face").finish()
    }
}

impl Face {
    pub fn from_ttf(data: Vec<u8>) -> Result<Self> {
        FaceTryBuilder {
            data,
            face_builder: |data| {
                let face = ttf_parser::Face::parse(data, 0)?;

                if let Some(os2) = face.tables().os2 {
                    use crate::error::PermissionError;

                    if matches!(os2.permissions(), Some(ttf_parser::Permissions::Restricted)) {
                        return Err(PermissionError::RestrictedLicense.into());
                    } else if !os2.is_subsetting_allowed() {
                        return Err(PermissionError::NoSubsetting.into());
                    } else if !os2.is_outline_embedding_allowed() {
                        return Err(PermissionError::BitmapEmbeddingOnly.into());
                    }
                }

                Ok(face)
            },
            cmap_builder: |face| {
                Ok(face.tables().cmap.map_or_else(
                    || {
                        warn!("no CMAP table in font");
                        Vec::new()
                    },
                    |cmap| {
                        cmap.subtables
                            .into_iter()
                            .filter(ttf_parser::cmap::Subtable::is_unicode) // Filter out non-unicode subtables
                            .collect()
                    },
                ))
            },
            kern_builder: |face| {
                Ok(face.tables().kern.map_or_else(Vec::new, |kern| {
                    kern.subtables
                        .into_iter()
                        .filter(|st| {
                            st.horizontal // We only support LTR for the moment
                                && !st.variable // We don't support variable fonts
                                && !st.has_cross_stream // TODO support this?
                                && !st.has_state_machine // Not supported by ttf-parser
                        })
                        .collect()
                }))
            },
        }
        .try_build()
    }

    pub fn names(&self) -> ttf_parser::name::Names<'_> {
        self.borrow_face().names()
    }

    pub fn italic_angle(&self) -> Option<f32> {
        self.borrow_face().italic_angle()
    }

    pub fn ascender(&self) -> i16 {
        self.borrow_face().ascender()
    }

    pub fn descender(&self) -> i16 {
        self.borrow_face().descender()
    }

    pub fn line_gap(&self) -> i16 {
        self.borrow_face().line_gap()
    }

    pub fn units_per_em(&self) -> u16 {
        self.borrow_face().units_per_em()
    }

    pub fn x_height(&self) -> Option<i16> {
        self.borrow_face().x_height()
    }

    pub fn capital_height(&self) -> Option<i16> {
        self.borrow_face().capital_height()
    }

    pub fn number_of_glyphs(&self) -> u16 {
        self.borrow_face().number_of_glyphs()
    }

    pub fn glyph_index(&self, char: char) -> Option<GlyphId> {
        self.borrow_cmap()
            .iter()
            .find_map(|cmap| cmap.glyph_index(u32::from(char)))
    }

    pub fn glyph_hor_advance(&self, glyph_id: GlyphId) -> Option<u16> {
        self.borrow_face().glyph_hor_advance(glyph_id)
    }

    pub fn glyphs_kerning(&self, lhs: GlyphId, rhs: GlyphId) -> Option<i16> {
        self.borrow_kern()
            .iter()
            .find_map(|kern| kern.glyphs_kerning(lhs, rhs))
    }

    pub fn outline_glyph(
        &self,
        gid: GlyphId,
        builder: &mut dyn ttf_parser::OutlineBuilder,
    ) -> Option<ttf_parser::Rect> {
        self.borrow_face().outline_glyph(gid, builder)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn face_clone() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        #[allow(clippy::redundant_clone)] // We want to test clone
        let face2 = face.clone();

        assert_eq!(face.borrow_data(), face2.borrow_data());
        assert_eq!(
            face.borrow_face().number_of_glyphs(),
            face2.borrow_face().number_of_glyphs()
        );
        assert_eq!(face.borrow_cmap().len(), face2.borrow_cmap().len());
        assert_eq!(face.borrow_kern().len(), face2.borrow_kern().len());
    }

    #[test]
    fn face_debug() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(format!("{face:?}"), "Face");
    }

    #[test]
    fn face_from_ttf() {
        let data = std::fs::read(env!("DEMO_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(face.borrow_cmap().len(), 1);
        assert_eq!(face.borrow_kern().len(), 1);

        let data = std::fs::read(env!("NULL_TTF")).unwrap();
        let face = Face::from_ttf(data).unwrap();

        assert_eq!(face.borrow_cmap().len(), 0);
        assert_eq!(face.borrow_kern().len(), 0);
    }

    #[test]
    fn face_permissions() {
        use crate::error::PermissionError::*;
        use crate::Error::PermissionError;

        let demo = Face::from_ttf(std::fs::read(env!("DEMO_TTF")).unwrap());
        assert!(demo.is_ok());

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
        assert_eq!(face.italic_angle(), None);
        assert_eq!(face.ascender(), 1024);
        assert_eq!(face.descender(), -400);
        assert_eq!(face.line_gap(), 0);
        assert_eq!(face.units_per_em(), 1000);
        assert_eq!(face.x_height(), Some(450));
        assert_eq!(face.capital_height(), Some(650));
        assert_eq!(face.number_of_glyphs(), 3);
        assert_eq!(face.glyph_index('A'), Some(GlyphId(1)));
        assert_eq!(face.glyph_hor_advance(GlyphId(1)), Some(540));
        assert_eq!(face.glyphs_kerning(GlyphId(1), GlyphId(2)), Some(-70));
        // Font::outline_glyph() only tested as part of Glyph::parse_from
    }
}
