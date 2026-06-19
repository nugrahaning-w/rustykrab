use crate::{
    modifier::{ModifierKind, ModifierValue},
    node::Node,
};

pub trait ModifierBuilders {
    fn padding(self, value: i64) -> Self;

    fn background(self, color: impl Into<String>) -> Self;

    fn corner_radius(self, radius: i64) -> Self;

    fn opacity(self, value: f64) -> Self;

    fn width(self, value: i64) -> Self;

    fn height(self, value: i64) -> Self;
}

impl ModifierBuilders for Node {
    fn padding(self, value: i64) -> Self {
        self.modifier(ModifierKind::Padding, ModifierValue::Integer(value))
    }

    fn background(self, color: impl Into<String>) -> Self {
        self.modifier(ModifierKind::Background, ModifierValue::Color(color.into()))
    }

    fn corner_radius(self, radius: i64) -> Self {
        self.modifier(ModifierKind::CornerRadius, ModifierValue::Integer(radius))
    }

    fn opacity(self, value: f64) -> Self {
        self.modifier(ModifierKind::Opacity, ModifierValue::Float(value))
    }

    fn width(self, value: i64) -> Self {
        self.modifier(ModifierKind::Width, ModifierValue::Integer(value))
    }

    fn height(self, value: i64) -> Self {
        self.modifier(ModifierKind::Height, ModifierValue::Integer(value))
    }
}
