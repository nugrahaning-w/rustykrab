use crate::modifier::{Modifier, ModifierKind, ModifierValue};

#[test]
fn create_padding_modifier() {
    let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

    assert_eq!(modifier.kind(), &ModifierKind::Padding,);

    assert_eq!(modifier.value(), &ModifierValue::Integer(16),);
}

#[test]
fn create_background_modifier() {
    let modifier = Modifier::new(
        ModifierKind::Background,
        ModifierValue::Color("#FF0000".into()),
    );

    assert_eq!(modifier.kind(), &ModifierKind::Background,);
}

#[test]
fn create_opacity_modifier() {
    let modifier = Modifier::new(ModifierKind::Opacity, ModifierValue::Float(0.8));

    assert_eq!(modifier.value(), &ModifierValue::Float(0.8),);
}

#[test]
fn custom_modifier() {
    let modifier = Modifier::new(
        ModifierKind::Custom("glass".into()),
        ModifierValue::Boolean(true),
    );

    assert!(matches!(modifier.kind(), ModifierKind::Custom(_)));
}
