use crate::modifier::Modifier;

/// Ordered collection of modifiers.
///
/// Modifier order is preserved because
/// SwiftUI and Compose apply modifiers
/// sequentially.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierChain {
    modifiers: Vec<Modifier>,
}

impl ModifierChain {
    /// Creates an empty modifier chain.
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    /// Adds a modifier.
    pub fn add(&mut self, modifier: Modifier) {
        self.modifiers.push(modifier);
    }

    /// Removes modifier by index.
    pub fn remove(&mut self, index: usize) -> Option<Modifier> {
        if index >= self.modifiers.len() {
            return None;
        }

        Some(self.modifiers.remove(index))
    }

    /// Number of modifiers.
    pub fn len(&self) -> usize {
        self.modifiers.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Returns all modifiers.
    pub fn modifiers(&self) -> &[Modifier] {
        &self.modifiers
    }

    /// Returns iterator.
    pub fn iter(&self) -> std::slice::Iter<'_, Modifier> {
        self.modifiers.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::modifier::{ModifierKind, ModifierValue};

    #[test]
    fn empty_chain() {
        let chain = ModifierChain::new();

        assert!(chain.is_empty());
    }

    #[test]
    fn add_modifier() {
        let mut chain = ModifierChain::new();

        chain.add(Modifier::new(
            ModifierKind::Padding,
            ModifierValue::Integer(16),
        ));

        assert_eq!(chain.len(), 1);
    }

    #[test]
    fn remove_modifier() {
        let mut chain = ModifierChain::new();

        chain.add(Modifier::new(
            ModifierKind::Padding,
            ModifierValue::Integer(16),
        ));

        let removed = chain.remove(0);

        assert!(removed.is_some());

        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn preserves_order() {
        let mut chain = ModifierChain::new();

        chain.add(Modifier::new(
            ModifierKind::Padding,
            ModifierValue::Integer(16),
        ));

        chain.add(Modifier::new(
            ModifierKind::Background,
            ModifierValue::Color("#FF0000".into()),
        ));

        let modifiers = chain.modifiers();

        assert_eq!(modifiers[0].kind(), &ModifierKind::Padding,);

        assert_eq!(modifiers[1].kind(), &ModifierKind::Background,);
    }
}
