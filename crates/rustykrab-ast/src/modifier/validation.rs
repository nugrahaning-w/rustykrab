use crate::modifier::{Modifier, ModifierKind, ModifierValue};

/// Validation error.
pub type ValidationResult = Result<(), String>;

/// Validates a modifier.
pub fn validate_modifier(modifier: &Modifier) -> ValidationResult {
    match modifier.kind() {
        ModifierKind::Padding => validate_padding(modifier.value()),

        ModifierKind::Opacity => validate_opacity(modifier.value()),

        ModifierKind::CornerRadius => validate_corner_radius(modifier.value()),

        _ => Ok(()),
    }
}

/// Padding must be >= 0.
pub fn validate_padding(value: &ModifierValue) -> ValidationResult {
    match value {
        ModifierValue::Integer(v) if *v >= 0 => Ok(()),

        ModifierValue::Integer(_) => Err("Padding cannot be negative".into()),

        _ => Err("Padding requires Integer value".into()),
    }
}

/// Opacity must be 0.0 ..= 1.0.
pub fn validate_opacity(value: &ModifierValue) -> ValidationResult {
    match value {
        ModifierValue::Float(v) if *v >= 0.0 && *v <= 1.0 => Ok(()),

        ModifierValue::Float(_) => Err("Opacity must be between 0 and 1".into()),

        _ => Err("Opacity requires Float value".into()),
    }
}

/// Corner radius must be >= 0.
pub fn validate_corner_radius(value: &ModifierValue) -> ValidationResult {
    match value {
        ModifierValue::Integer(v) if *v >= 0 => Ok(()),

        ModifierValue::Integer(_) => Err("Corner radius cannot be negative".into()),

        _ => Err("Corner radius requires Integer value".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_padding() {
        let result = validate_padding(&ModifierValue::Integer(16));

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_padding() {
        let result = validate_padding(&ModifierValue::Integer(-1));

        assert!(result.is_err());
    }

    #[test]
    fn valid_opacity() {
        let result = validate_opacity(&ModifierValue::Float(0.8));

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_opacity() {
        let result = validate_opacity(&ModifierValue::Float(1.5));

        assert!(result.is_err());
    }

    #[test]
    fn valid_corner_radius() {
        let result = validate_corner_radius(&ModifierValue::Integer(8));

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_corner_radius() {
        let result = validate_corner_radius(&ModifierValue::Integer(-8));

        assert!(result.is_err());
    }
}
