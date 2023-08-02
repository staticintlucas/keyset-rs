mod key;

use kurbo::Point;

use crate::key::Type as KeyType;
use crate::{DrawingOptions, Key};

pub(crate) use self::key::KeyPath;

// TODO move this somewhere?
const ARC_TOL: f64 = 1.; // Tolerance for converting Arc->Bézier with Kurbo

#[derive(Debug, Clone)]
pub(crate) struct KeyDrawing {
    pub origin: Point,
    pub paths: Vec<KeyPath>,
}

impl KeyDrawing {
    pub fn new(key: &Key, options: &DrawingOptions) -> Self {
        let show_key = options.show_keys && !matches!(key.typ, KeyType::None);

        let bottom = show_key.then(|| KeyPath::bottom(key, options));
        let top = show_key.then(|| KeyPath::top(key, options));
        let step = show_key.then(|| KeyPath::step(key, options)).flatten();
        let homing = show_key.then(|| KeyPath::homing(key, options)).flatten();

        // Do a bunch of chaining here rather than using [...].iter().filter_map(|it| it). This
        // gives iterator a known size so it will allocate the required size when collecting to a
        // Vec<_>
        let paths = bottom.into_iter().chain(top).chain(step).chain(homing);

        Self {
            origin: key.position,
            paths: paths.collect(),
        }
    }
}
