# Modifier System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the Modifier System used by the RustyKrab AST.

The Modifier System provides a platform-independent mechanism for applying visual styling, layout configuration, appearance customization, and behavioral transformations to Nodes.

Instead of embedding style information directly into widget properties, modifiers are represented as independent entities attached to a Node.

Example:

```rust
Text::new("Welcome")
    .padding(16)
    .foreground(Color::Blue)
```

AST:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Welcome"
  },
  "modifiers": [
    {
      "type": "Padding",
      "value": 16
    },
    {
      "type": "Foreground",
      "value": "#0000FF"
    }
  ]
}
```

---

# 2. Design Goals

## DG-001 Platform Independence

Modifiers must not contain platform-specific concepts.

Invalid:

```text
SwiftUIPadding
ComposeModifier
UIKitConstraint
```

Valid:

```text
Padding
Background
Width
Height
```

---

## DG-002 Reusable Styling

Modifiers should be reusable across all NodeKinds.

Example:

```text
Text
Button
Image
Card
```

can all use:

```text
Padding
Background
CornerRadius
```

---

## DG-003 Generator Friendly

Generators should map modifiers directly to platform implementations.

Example:

```text
Padding
```

Maps to:

```text
SwiftUI
.padding()

Compose
.padding()
```

---

## DG-004 Extensibility

New modifiers should be added without changing Node structure.

---

## DG-005 Order Preservation

Modifier execution order must be preserved.

Example:

```rust
Text::new("Hello")
    .padding(16)
    .background(Color::Blue)
```

must remain:

```text
Padding
↓
Background
```

during generation.

---

# 3. Architecture Overview

```text
Node
 │
 ├── Properties
 ├── Modifiers
 ├── Events
 └── Children
```

Modifiers represent visual transformations applied to a Node.

---

# 4. Modifier Model

## Modifier Enum

```rust
pub enum Modifier {
    Layout(LayoutModifier),

    Style(StyleModifier),

    Effect(EffectModifier),

    Accessibility(AccessibilityModifier),

    Animation(AnimationModifier),
}
```

This acts as the root modifier abstraction.

---

# 5. Layout Modifiers

Layout modifiers control size and positioning.

---

## LayoutModifier

```rust
pub enum LayoutModifier {
    Width(f32),

    Height(f32),

    MinWidth(f32),

    MinHeight(f32),

    MaxWidth(f32),

    MaxHeight(f32),

    Padding(EdgeInsets),

    Margin(EdgeInsets),

    Alignment(Alignment),

    Offset(Position),
}
```

---

## Example

```rust
Text::new("Login")
    .width(200)
    .padding(16)
```

AST:

```json
{
  "modifiers": [
    {
      "type": "Width",
      "value": 200
    },
    {
      "type": "Padding",
      "value": 16
    }
  ]
}
```

---

# 6. Style Modifiers

Style modifiers control appearance.

---

## StyleModifier

```rust
pub enum StyleModifier {
    Foreground(ColorValue),

    Background(ColorValue),

    FontSize(f32),

    FontWeight(FontWeight),

    CornerRadius(f32),

    Border(BorderStyle),

    Opacity(f32),
}
```

---

## Example

```rust
Button::new("Login")
    .foreground(Color::White)
    .background(Color::Blue)
```

---

# 7. Effect Modifiers

Effect modifiers provide visual enhancements.

---

## EffectModifier

```rust
pub enum EffectModifier {
    Shadow(ShadowStyle),

    Blur(f32),

    Scale(f32),

    Rotation(f32),
}
```

---

## Example

```rust
Card::new()
    .shadow(8)
```

---

# 8. Accessibility Modifiers

Accessibility support must be first-class.

---

## AccessibilityModifier

```rust
pub enum AccessibilityModifier {
    Label(String),

    Hint(String),

    Hidden(bool),
}
```

---

## Example

```rust
Button::new("Login")
    .accessibility_label(
        "Login Button"
    )
```

---

# 9. Animation Modifiers

Animations are represented separately.

---

## AnimationModifier

```rust
pub enum AnimationModifier {
    Fade,

    Scale,

    Slide,

    Custom(String),
}
```

---

## Example

```rust
Text::new("Loading")
    .animation(Animation::Fade)
```

---

# 10. Supporting Models

## EdgeInsets

```rust
pub struct EdgeInsets {
    pub top: f32,

    pub right: f32,

    pub bottom: f32,

    pub left: f32,
}
```

---

## Alignment

```rust
pub enum Alignment {
    Start,

    Center,

    End,

    Top,

    Bottom,
}
```

---

## Position

```rust
pub struct Position {
    pub x: f32,

    pub y: f32,
}
```

---

## ShadowStyle

```rust
pub struct ShadowStyle {
    pub radius: f32,

    pub opacity: f32,

    pub offset_x: f32,

    pub offset_y: f32,
}
```

---

# 11. Modifier Ordering

Modifier order must always be preserved.

Example:

```rust
Text::new("Hello")
    .padding(16)
    .background(Color::Blue)
    .corner_radius(8)
```

AST:

```text
Padding
↓
Background
↓
CornerRadius
```

The generator must process modifiers in order.

---

# 12. Generator Mapping

## SwiftUI

| Modifier   | SwiftUI            |
| ---------- | ------------------ |
| Padding    | .padding()         |
| Width      | .frame(width:)     |
| Height     | .frame(height:)    |
| Background | .background()      |
| Foreground | .foregroundStyle() |
| Shadow     | .shadow()          |
| Opacity    | .opacity()         |

---

## Compose

| Modifier   | Compose               |
| ---------- | --------------------- |
| Padding    | Modifier.padding()    |
| Width      | Modifier.width()      |
| Height     | Modifier.height()     |
| Background | Modifier.background() |
| Shadow     | Modifier.shadow()     |
| Opacity    | Modifier.alpha()      |

---

# 13. Validation Rules

## MS-001

Modifier values must be valid.

Valid:

```text
Padding(16)
```

Invalid:

```text
Padding(-50)
```

---

## MS-002

Opacity must be between:

```text
0.0
and
1.0
```

---

## MS-003

Width and Height must be positive values.

---

## MS-004

Modifier order must be preserved.

---

# 14. Serialization Example

```json
{
  "kind": "Button",
  "modifiers": [
    {
      "type": "Padding",
      "value": 16
    },
    {
      "type": "Background",
      "value": "#0000FF"
    },
    {
      "type": "CornerRadius",
      "value": 8
    }
  ]
}
```

---

# 15. Future Expansion

Future modifiers may include:

```text
Gradient

GlassEffect

Material

Transform

Elevation

SafeArea

KeyboardAvoidance
```

without requiring changes to:

```text
Node

NodeKind

Property System
```

---

# 16. Why Use a Modifier System?

Rejected Design:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello",
    "padding": 16,
    "color": "blue",
    "cornerRadius": 8
  }
}
```

Problems:

* Style mixed with content
* Difficult validation
* Difficult generator mapping
* Poor scalability

---

Selected Design:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello"
  },
  "modifiers": [
    {
      "type": "Padding"
    }
  ]
}
```

Benefits:

* Cleaner separation
* Easier generator implementation
* Better extensibility
* Consistent architecture

---

# 17. Architectural Decisions

| Decision                                    | Reason                 |
| ------------------------------------------- | ---------------------- |
| Modifiers stored separately from properties | Separation of concerns |
| Modifier ordering preserved                 | Consistent rendering   |
| Platform-independent modifiers              | Multi-platform support |
| Accessibility included                      | Production readiness   |
| Animation included                          | Future-proof design    |
| Enum-based modifier model                   | Strong typing          |

---

# 18. Acceptance Criteria

This design is considered complete when:

* Modifier architecture is defined
* Modifier categories are documented
* Layout modifiers are documented
* Style modifiers are documented
* Effect modifiers are documented
* Accessibility modifiers are documented
* Animation modifiers are documented
* Generator mappings are documented
* Validation rules are documented

---