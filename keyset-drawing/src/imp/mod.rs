mod key;
mod legend;

use ::key::{Homing, Key, Shape as KeyShape};
use color::Color;
use geom::{ConvertInto as _, Dot, KeyUnit, Path, Point, Rect, Scale, Vector};
use isclose::IsClose as _;

use crate::{Template, Warning};

#[derive(Debug, Clone, Copy)]
pub struct Outline {
    pub color: Color,
    pub width: Dot,
}

#[derive(Debug, Clone)]
pub struct KeyPath {
    pub data: Path<Dot>,
    pub outline: Option<Outline>,
    pub fill: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct KeyDrawing {
    pub origin: Point<KeyUnit>,
    pub paths: Box<[KeyPath]>,
}

impl KeyDrawing {
    pub fn new(key: &Key, template: &Template, warnings: &mut Vec<Warning>) -> Self {
        let cap = Self::required_capacity(key, template);
        let mut paths = Vec::with_capacity(cap);

        let Template {
            ref profile,
            ref font,
            show_keys,
            show_margin,
            ..
        } = *template;

        match key.shape {
            _ if !show_keys => {}
            KeyShape::None(..) => {}
            KeyShape::Normal(size) | KeyShape::Space(size) => {
                paths.push(key::bottom(template, key.color, size));
                paths.push(key::top(template, key.color, size));
            }
            KeyShape::Homing(homing) => {
                paths.push(key::bottom(
                    template,
                    key.color,
                    Vector::splat(KeyUnit(1.0)),
                ));
                paths.push(key::top(template, key.color, Vector::splat(KeyUnit(1.0))));

                match homing.unwrap_or(profile.homing.default) {
                    Homing::Scoop => {}
                    Homing::Bar => {
                        paths.push(key::homing_bar(template, key.color));
                    }
                    Homing::Bump => {
                        paths.push(key::homing_bump(template, key.color));
                    }
                }
            }
            KeyShape::SteppedCaps => {
                paths.push(key::bottom(
                    template,
                    key.color,
                    Vector::new(KeyUnit(1.75), KeyUnit(1.0)),
                ));
                paths.push(key::top(
                    template,
                    key.color,
                    Vector::new(KeyUnit(1.25), KeyUnit(1.0)),
                ));
                paths.push(key::step(template, key.color));
            }
            KeyShape::IsoVertical | KeyShape::IsoHorizontal => {
                paths.push(key::iso_bottom(template, key.color));
                paths.push(key::iso_top(template, key.color));
            }
        }

        let top_rect = {
            let Rect { min, max } = key.shape.inner_rect();
            let (dmin, dmax) = (min - Point::origin(), max - Point::splat(KeyUnit(1.0)));
            let Rect { min, max } = profile.top.to_rect();
            Rect::new(min + dmin.convert_into(), max + dmax.convert_into())
        };

        if show_margin {
            let mut boundses = Vec::<Rect<Dot>>::with_capacity(key.legends.len());
            for leg in key.legends.iter().flatten() {
                let size = leg.size_idx;
                let text_bounds = top_rect - profile.legend_geom.for_kle_size(size).margin;
                if !boundses.iter().any(|rect| rect.is_close(&text_bounds)) {
                    boundses.push(text_bounds);
                }
            }

            let data = boundses.into_iter().map(Rect::to_path).collect();

            paths.push(KeyPath {
                data,
                outline: Some(Outline {
                    color: Color::new(1.0, 0.0, 0.0),
                    width: Dot(5.0),
                }),
                fill: None,
            });
        }

        let legends = key
            .legends
            .iter()
            .enumerate()
            .filter_map(|(i, l)| {
                l.as_ref().map(|legend| {
                    #[expect(clippy::cast_precision_loss, reason = "i <= 9")]
                    let align = Scale::new(0.5 * ((i % 3) as f32), 0.5 * ((i / 3) as f32));
                    legend::draw(legend, font, profile, top_rect, align, warnings)
                })
            })
            .collect::<Vec<_>>();

        paths.extend_from_slice(&legends);

        Self {
            origin: key.position,
            paths: paths.into_boxed_slice(),
        }
    }

    fn required_capacity(key: &Key, template: &Template) -> usize {
        let Template {
            ref profile,
            show_keys,
            show_margin,
            ..
        } = *template;

        let key_paths = match key.shape {
            _ if !show_keys => 0,
            KeyShape::None(..) => 0,
            KeyShape::Normal(..)
            | KeyShape::Space(..)
            | KeyShape::IsoVertical
            | KeyShape::IsoHorizontal => 2,
            KeyShape::Homing(homing) => match homing.unwrap_or(profile.homing.default) {
                Homing::Scoop => 2,
                Homing::Bar | Homing::Bump => 3,
            },
            KeyShape::SteppedCaps => 3,
        };

        let margin = usize::from(show_margin);

        let legends = key.legends.iter().flatten().count();

        key_paths + margin + legends
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use geom::{ConvertInto as _, Translate};
    use isclose::assert_is_close;

    use super::*;

    #[test]
    fn test_key_drawing_new() {
        // Regular 1u
        let key = Key::example();
        let template = Template::default();
        let mut warnings = Vec::new();
        let drawing = KeyDrawing::new(&key, &template, &mut warnings);

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 6); // top, bottom, 4x legends
        assert!(warnings.is_empty());

        // Stepped caps
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::SteppedCaps;
            key
        };
        let template = Template::default();
        let mut warnings = Vec::new();
        let drawing = KeyDrawing::new(&key, &template, &mut warnings);

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, step, 4x legends
        assert!(warnings.is_empty());

        // ISO H
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoHorizontal;
            key
        };
        let template = Template {
            show_margin: true,
            ..Template::default()
        };
        let mut warnings = Vec::new();
        let drawing = KeyDrawing::new(&key, &template, &mut warnings);

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounds;
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let top_rect = {
            let Rect { min, max } = template.profile.top.to_rect();
            let max = max + Vector::new(KeyUnit(0.5), KeyUnit(0.0)).convert_into();
            Rect::new(min, max)
        };
        let margin_rect = top_rect - template.profile.legend_geom.margin_for_kle_size(font_size);
        assert_is_close!(bounding_box, margin_rect);
        assert!(warnings.is_empty());

        // ISO V
        let key = {
            let mut key = Key::example();
            key.shape = ::key::Shape::IsoVertical;
            key
        };
        let template = Template {
            show_margin: true,
            ..Template::default()
        };
        let mut warnings = Vec::new();
        let drawing = KeyDrawing::new(&key, &template, &mut warnings);

        assert_is_close!(drawing.origin, key.position);
        assert_eq!(drawing.paths.len(), 7); // top, bottom, margin, 4x legends
        let bounding_box = drawing.paths[2].data.bounds;
        let font_size = key.legends[0].as_ref().unwrap().size_idx;
        let top_rect = {
            let Rect { min, max } = template.profile.top.to_rect();
            let max = max + Vector::new(KeyUnit(0.25), KeyUnit(1.0)).convert_into();
            Rect::new(min, max)
        };
        let margin_rect = top_rect
            * Translate::<Dot>::new(KeyUnit(0.25).convert_into(), KeyUnit(0.0).convert_into())
            - template.profile.legend_geom.margin_for_kle_size(font_size);
        assert_is_close!(bounding_box, margin_rect);
        assert!(warnings.is_empty());
    }
}
