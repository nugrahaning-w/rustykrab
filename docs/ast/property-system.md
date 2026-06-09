# Property System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the Property System used by the RustyKrab AST.

The Property System provides a flexible and extensible mechanism for storing widget configuration data without requiring widget-specific AST structures.

Instead of creating dedicated node types such as:

```rust
TextNode
ButtonNode
ImageNode
TextFieldNode
```

all widget-specific data is stored using a generic property system.

This approach significantly improves:

* Extensibility
* Serialization
* Generator implementation
* Plugin support
* Future widget expansion

---

# 2. Design Goals

## DG-001 Generic Configuration Storage

All widget data must be stored using a common property model.

Example:

```rust
Text::new("Hello")
```

becomes:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello"
  }
}
```

---

## DG-002 Platform Independence

Properties must not contain platform-specific concepts.

Invalid:

```text
swift_font
compose_modifier
android_padding
```

Valid:

```text
font_size
padding
color
```

---

## DG-003 Extensibility

Adding new widgets must not require changes to the Node structure.

Future widgets:

```text
Chart
Map
WebView
VideoPlayer
Camera
```

must reuse the same property system.

---

## DG-004 Serialization Friendly

Properties must be easily serializable.

Supported formats:

* JSON
* YAML
* Binary

---

## DG-005 Generator Friendly

Generators must be able to access properties in a predictable way.

Example:

```rust
node.get_property("value")
```

---

# 3. Architecture Overview

The Property System is attached to every Node.

```text
Node
│
├── NodeKind
├── Properties
├── Modifiers
├── Events
├── Bindings
└── Children
```

Properties store widget-specific data.

---

# 4. Property Model

## PropertyMap

```rust
pub type PropertyMap =
    HashMap<String, PropertyValue>;
```

Every node contains:

```rust
properties: PropertyMap
```

---

# 5. PropertyValue

Property values support multiple primitive and structured types.

```rust
pub enum PropertyValue {
    String(String),

    Integer(i64),

    Float(f64),

    Boolean(bool),

    Enum(String),

    Color(ColorValue),

    Expression(String),

    Array(Vec<PropertyValue>),

    Object(PropertyObject),

    Null,
}
```

---

# 6. PropertyObject

Complex objects are represented using PropertyObject.

```rust
pub type PropertyObject =
    HashMap<String, PropertyValue>;
```

Example:

```json
{
  "shadow": {
    "radius": 8,
    "opacity": 0.3
  }
}
```

---

# 7. Property Categories

Properties are grouped into categories.

---

## Content Properties

Represent widget content.

Examples:

```text
value
title
placeholder
source
```

---

## Layout Properties

Represent layout configuration.

Examples:

```text
width
height
min_width
max_width
```

---

## Style Properties

Represent appearance.

Examples:

```text
font_size
font_weight
color
background
```

---

## State Properties

Represent state bindings.

Examples:

```text
is_enabled
is_loading
is_visible
```

---

## Data Properties

Represent collections and data sources.

Examples:

```text
items
sections
data_source
```

---

# 8. Standard Property Names

To ensure consistency across generators, property names must follow a standard vocabulary.

---

## Text

```json
{
  "value": "Hello"
}
```

---

## Button

```json
{
  "title": "Login"
}
```

---

## Image

```json
{
  "source": "logo.png"
}
```

---

## TextField

```json
{
  "placeholder": "Email"
}
```

---

# 9. Property Examples

## Text Widget

DSL:

```rust
Text::new("Welcome")
```

AST:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Welcome"
  }
}
```

---

## Button Widget

DSL:

```rust
Button::new("Login")
```

AST:

```json
{
  "kind": "Button",
  "properties": {
    "title": "Login"
  }
}
```

---

## Image Widget

DSL:

```rust
Image::new("logo.png")
```

AST:

```json
{
  "kind": "Image",
  "properties": {
    "source": "logo.png"
  }
}
```

---

# 10. Future Property Examples

Future widgets require no AST redesign.

Example:

```rust
Map::new()
```

AST:

```json
{
  "kind": "Map",
  "properties": {
    "latitude": -6.200,
    "longitude": 106.816
  }
}
```

---

Example:

```rust
VideoPlayer::new()
```

AST:

```json
{
  "kind": "VideoPlayer",
  "properties": {
    "url": "video.mp4",
    "autoplay": true
  }
}
```

---

# 11. Property Access API

Recommended API:

```rust
pub trait PropertyAccess {
    fn get(
        &self,
        key: &str,
    ) -> Option<&PropertyValue>;

    fn set(
        &mut self,
        key: impl Into<String>,
        value: PropertyValue,
    );
}
```

---

Example:

```rust
node.set(
    "value",
    PropertyValue::String(
        "Hello".into()
    )
);
```

---

# 12. Validation Rules

## PS-001

Property names must be unique.

Valid:

```json
{
  "title": "Login"
}
```

Invalid:

```json
{
  "title": "Login",
  "title": "Submit"
}
```

---

## PS-002

Property values must match expected types.

Valid:

```json
{
  "font_size": 16
}
```

Invalid:

```json
{
  "font_size": "large"
}
```

---

## PS-003

Required properties must exist.

Example:

Image:

```json
{
  "source": "logo.png"
}
```

Required:

```text
source
```

---

# 13. Serialization Example

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello World",
    "font_size": 16,
    "color": "#000000"
  }
}
```

---

# 14. Why Not Widget-Specific Nodes?

Rejected Design:

```rust
pub struct TextNode {
    value: String
}

pub struct ButtonNode {
    title: String
}
```

Problems:

* Large number of node types
* High maintenance cost
* Difficult plugin support
* Difficult future expansion

---

Selected Design:

```rust
Node {
    kind: Text,

    properties: {
        "value": "Hello"
    }
}
```

Benefits:

* Smaller AST
* Easier generators
* Easier serialization
* Easier plugin ecosystem

---

# 15. Future Expansion Strategy

The Property System must support:

```text
Theme System

Animation System

Accessibility

Localization

Forms

Data Binding

Networking
```

without requiring changes to:

```text
Node

NodeKind

Generator API
```

---

# 16. Architectural Decisions

| Decision                    | Reason                       |
| --------------------------- | ---------------------------- |
| HashMap-based properties    | Flexible storage             |
| Generic PropertyValue       | Supports all widgets         |
| Property categories defined | Improves consistency         |
| Standard property names     | Generator compatibility      |
| Object support included     | Enables complex widgets      |
| Array support included      | Supports collections         |
| Platform-independent naming | Multi-platform compatibility |

---

# 17. Acceptance Criteria

This design is considered complete when:

* PropertyMap is defined
* PropertyValue is defined
* Property categories are documented
* Standard property names are documented
* Validation rules are documented
* Serialization examples are provided
* Future expansion strategy is documented

---