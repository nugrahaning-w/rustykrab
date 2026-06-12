use super::PropertyValue;

/// Standard interface for property containers.
///
/// This trait provides a unified API for
/// reading and writing properties.
///
/// Implementors:
/// - Node (Story 2.3.7)
/// - Future Modifier objects
/// - Future Event objects
pub trait PropertyAccess {
    /// Returns a property value by key.
    fn get(&self, key: &str) -> Option<&PropertyValue>;

    /// Sets or replaces a property value.
    fn set(&mut self, key: impl Into<String>, value: PropertyValue);

    /// Returns true if the property exists.
    fn contains(&self, key: &str) -> bool;

    /// Removes a property.
    fn remove(&mut self, key: &str) -> Option<PropertyValue>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time verification that the trait
    /// can be implemented by a custom type.
    struct MockPropertyContainer;

    impl PropertyAccess for MockPropertyContainer {
        fn get(&self, _key: &str) -> Option<&PropertyValue> {
            None
        }

        fn set(&mut self, _key: impl Into<String>, _value: PropertyValue) {}

        fn contains(&self, _key: &str) -> bool {
            false
        }

        fn remove(&mut self, _key: &str) -> Option<PropertyValue> {
            None
        }
    }

    #[test]
    fn trait_can_be_implemented() {
        let container = MockPropertyContainer;

        assert!(!container.contains("value"));
    }
}
