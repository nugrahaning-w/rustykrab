# Modifier Examples

## Basic Text

```rust
let text = Node::new(
    NodeId::new("title"),
    NodeKind::Text,
)
.property(
    PropertyKeys::VALUE,
    PropertyValue::String(
        "Welcome".into(),
    ),
)
.padding(16);
```

---

## Background

```rust
let text = Node::new(
    NodeId::new("title"),
    NodeKind::Text,
)
.background("#FF0000");
```

---

## Opacity

```rust
let image = Node::new(
    NodeId::new("logo"),
    NodeKind::Image,
)
.opacity(0.8);
```

---

## Corner Radius

```rust
let card = Node::new(
    NodeId::new("card"),
    NodeKind::Container,
)
.corner_radius(12);
```

---

## Multiple Modifiers

```rust
let title = Node::new(
    NodeId::new("title"),
    NodeKind::Text,
)
.padding(16)
.background("#FF0000")
.corner_radius(8)
.opacity(0.8);
```

---

## Manual Modifier Creation

```rust
let modifier = Modifier::new(
    ModifierKind::Padding,
    ModifierValue::Integer(16),
);
```

---

## Validation

```rust
let modifier = Modifier::new(
    ModifierKind::Opacity,
    ModifierValue::Float(0.8),
);

assert!(
    validate_modifier(&modifier)
        .is_ok()
);
```

---

## Accessing Modifiers

```rust
let modifiers =
    node.modifiers.modifiers();

for modifier in modifiers {
    println!(
        "{:?}",
        modifier.kind(),
    );
}
```
