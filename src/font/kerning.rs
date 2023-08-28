use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Kerning {
    pairs: HashMap<(char, char), f64>,
}

impl Kerning {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get(&self, lhs: char, rhs: char) -> f64 {
        self.pairs.get(&(lhs, rhs)).copied().unwrap_or(0.)
    }

    pub fn set(&mut self, lhs: char, rhs: char, kern: f64) {
        // Kern should be an integer here
        if kern.abs() > 0.5 {
            self.pairs.insert((lhs, rhs), kern);
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

impl Default for Kerning {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_get() {
        let kerning = Kerning {
            pairs: hashmap! {('A', 'V') => -50.},
        };

        assert_eq!(kerning.get('A', 'V'), -50.);
        assert_eq!(kerning.get('A', 'B'), 0.);
    }

    #[test]
    fn test_set() {
        let mut kerning = Kerning::new();
        kerning.set('A', 'V', -50.);
        kerning.set('A', 'B', 0.);

        assert!(kerning.pairs.contains_key(&('A', 'V')));
        assert!(!kerning.pairs.contains_key(&('A', 'B')));
    }

    #[test]
    fn test_len() {
        let mut kerning = Kerning::new();

        assert_eq!(kerning.len(), 0);

        kerning.set('A', 'V', -50.);
        kerning.set('A', 'B', 0.);

        assert_eq!(kerning.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let mut kerning = Kerning::new();

        assert!(kerning.is_empty());

        kerning.set('A', 'V', -50.);
        kerning.set('A', 'B', 0.);

        assert!(!kerning.is_empty());
    }

    #[test]
    fn test_default() {
        let kerning = Kerning::default();
        assert_eq!(kerning.pairs, hashmap! {});
    }
}
