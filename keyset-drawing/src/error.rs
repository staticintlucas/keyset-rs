use std::fmt;

use geom::{Mm, Unit as _, Vector};

use indoc::writedoc;

#[cfg(feature = "png")]
use crate::png::Pixel;

/// A drawing creation error
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// The drawing is larger than the maximum PNG dimensions
    #[cfg(feature = "png")]
    PngDimensionsError(Vector<Pixel>),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(feature = "png")]
            Self::PngDimensionsError(dims) => writedoc!(
                f,
                "
                    drawing dimensions too large for exporting a PNG image
                      maximum PNG size: {max} x {max} pixels
                      drawing size: {dim_x} x {dim_y} pixels
                    try reducing the PNG's DPI when exporting
                ",
                max = (u32::MAX as usize) / tiny_skia::BYTES_PER_PIXEL,
                dim_x = dims.x.get(),
                dim_y = dims.y.get(),
            ),
        }
    }
}

impl std::error::Error for Error {}

/// A drawing creation warning
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Warning {
    /// The legend is wider than the available area
    LegendTooWide {
        /// The legend text
        text: String,
        /// The width of the legend
        legend: Mm,
        /// The available width for legends
        bounds: Mm,
    },
    /// The legend is taller than the available area
    LegendTooTall {
        /// The legend text
        text: String,
        /// The height of the legend
        legend: Mm,
        /// The available height for legends
        bounds: Mm,
    },
}

impl fmt::Display for Warning {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LegendTooWide {
                ref text,
                legend,
                bounds,
            } => {
                writedoc!(
                    f,
                    r#"
                        legend "{text}" is too wide
                          available width: {bounds:.3} mm
                          legend width: {legend:.3} mm ({percent:.3}% too wide)
                        legend is being squished to fit
                    "#,
                    bounds = bounds.get(),
                    legend = legend.get(),
                    percent = 100.0 * ((legend / bounds) - 1.0),
                )
            }
            Self::LegendTooTall {
                ref text,
                legend,
                bounds,
            } => {
                writedoc!(
                    f,
                    r#"
                        legend "{text}" is too tall
                          available height: {bounds:.3} mm
                          legend height: {legend:.3} mm ({percent:.3}% too tall)
                        legend is being squished to fit
                    "#,
                    bounds = bounds.get(),
                    legend = legend.get(),
                    percent = 100.0 * ((legend / bounds) - 1.0),
                )
            }
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn error_fmt() {
        if cfg!(feature = "png") {
            let error = Error::PngDimensionsError(Vector::new(Pixel(20.0), Pixel(30.0)));
            assert_eq!(
                error.to_string(),
                indoc! { "
                    drawing dimensions too large for exporting a PNG image
                      maximum PNG size: 1073741823 x 1073741823 pixels
                      drawing size: 20 x 30 pixels
                    try reducing the PNG's DPI when exporting
                " },
            );
        }
    }

    #[test]
    fn warning_fmt() {
        let warning = Warning::LegendTooWide {
            text: "legend lmao".to_string(),
            legend: Mm(15.0),
            bounds: Mm(12.0),
        };
        assert_eq!(
            warning.to_string(),
            indoc! { r#"
                legend "legend lmao" is too wide
                  available width: 12.000 mm
                  legend width: 15.000 mm (25.000% too wide)
                legend is being squished to fit
            "# }
        );

        let warning = Warning::LegendTooTall {
            text: "legend lmao".to_string(),
            legend: Mm(15.0),
            bounds: Mm(12.0),
        };
        assert_eq!(
            warning.to_string(),
            indoc! { r#"
                legend "legend lmao" is too tall
                  available height: 12.000 mm
                  legend height: 15.000 mm (25.000% too tall)
                legend is being squished to fit
            "# }
        );
    }
}
