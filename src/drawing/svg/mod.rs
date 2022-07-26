mod key;
mod layout;

use crate::drawing::Drawing;

pub trait ToSvg<T> {
    fn to_svg(&self) -> T;
}

impl Drawing {
    #[must_use]
    pub fn to_svg(&self) -> String {
        self.layout.to_svg().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::Layout;
    use crate::utils::Size;

    use super::*;

    #[test]
    fn test_to_svg() {
        let layout = Layout {
            size: Size::new(1., 1.),
            keys: vec![],
        };
        let drawing = Drawing {
            layout,
            // profile,
            dpi: 96.,
        };

        assert_eq!(
            drawing.to_svg(),
            r#"<svg height="72" viewBox="0 0 1000 1000" width="72" xmlns="http://www.w3.org/2000/svg"/>"#
        );
    }
}
