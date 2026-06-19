# Modifier System

## Overview

The Modifier System provides platform-independent styling, layout, accessibility, and visual effects for RustyKrab AST nodes.

Modifiers are inspired by:

* SwiftUI Modifiers
* Jetpack Compose Modifiers
* Flutter Widget Modifiers

The goal is to allow UI definitions to remain platform-agnostic while still supporting rich styling capabilities.

---

## Architecture

A Node can contain:

```text
Node
├── Properties
├── Modifiers
├── Metadata
└── Children
```

Example:

```rust
let title = Node::new(
    NodeId::new("title"),
    NodeKind::Text,
)
.property(
    PropertyKeys::VALUE,
    PropertyValue::String(
        "Hello RustyKrab".into(),
    ),
)
.padding(16)
.background("#FF0000")
.corner_radius(8)
.opacity(0.8);
```

---

## Core Components

### ModifierKind

Represents the type of modifier.

Examples:

```rust
ModifierKind::Padding
ModifierKind::Background
ModifierKind::Opacity
ModifierKind::CornerRadius
```

---

### ModifierValue

Stores modifier data.

Examples:

```rust
ModifierValue::Integer(16)

ModifierValue::Float(0.8)

ModifierValue::Color(
    "#FF0000".into(),
)
```

---

### Modifier

Represents a single modifier.

```rust
Modifier {
    kind: ModifierKind::Padding,
    value: ModifierValue::Integer(16),
}
```

---

### ModifierChain

Stores an ordered collection of modifiers.

Order is preserved because many UI frameworks apply modifiers sequentially.

Example:

```rust
[
    Padding(16),
    Background("#FF0000"),
    Opacity(0.8),
]
```

---

## Categories

Modifiers are grouped into categories.

### Layout

* Padding
* Margin
* Width
* Height
* Frame
* Alignment

### Style

* Background
* ForegroundColor
* Border
* CornerRadius

### Visual Effect

* Opacity
* Shadow
* Blur

### Accessibility

* AccessibilityLabel
* AccessibilityIdentifier

### Animation

* Animation
* Transition

---

## Validation Rules

### Padding

Must be:

```text
>= 0
```

### Corner Radius

Must be:

```text
>= 0
```

### Opacity

Must be:

```text
0.0 <= value <= 1.0
```

---

## Future Roadmap

Planned modifier additions:

* Offset
* Rotation
* Scale
* Overlay
* ClipShape
* SafeArea
* ZIndex
* Gesture Modifiers
* Navigation Modifiers

These will be added without breaking existing AST structures.
