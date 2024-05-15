use crate::Scale;

/// Keyboard Unit, usually 19.05 mm or 0.75 in
#[derive(Clone, Copy, Debug, Default)]
pub struct Unit;

/// Dot, a.k.a. drawing unit
#[derive(Clone, Copy, Debug, Default)]
pub struct Dot;

/// Millimeter
#[derive(Clone, Copy, Debug, Default)]
pub struct Mm;

/// Inch
#[derive(Clone, Copy, Debug, Default)]
pub struct Inch;

/// Conversion factor for Keyboard Units to Drawing Units
pub const DOT_PER_UNIT: Scale<Unit, Dot> = Scale::new(1000.0);
/// Conversion factor for Keyboard Units to Millimeters
pub const MM_PER_UNIT: Scale<Unit, Mm> = Scale::new(19.05);
/// Conversion factor for Keyboard Units to Inches
pub const INCH_PER_UNIT: Scale<Unit, Inch> = Scale::new(0.75);

/// Conversion factor for Millimeters to Drawing Units
pub const DOT_PER_MM: Scale<Mm, Dot> = Scale::new(DOT_PER_UNIT.0 / MM_PER_UNIT.0);
/// Conversion factor for Inches to Drawing Units
pub const DOT_PER_INCH: Scale<Inch, Dot> = Scale::new(DOT_PER_UNIT.0 / INCH_PER_UNIT.0);
