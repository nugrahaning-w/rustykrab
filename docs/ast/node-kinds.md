# NodeKind System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the NodeKind System used by the RustyKrab AST.

NodeKind identifies the semantic type of a node and determines how generators, validators, and tooling interpret that node.

Every Node must contain exactly one NodeKind.

---

# 2. Design Goals

## DG-001 Platform Independence

NodeKind must not contain platform-specific concepts.

Invalid:

```text id="q1t5k2"
SwiftUIText
ComposeText
UIKitLabel
```

Valid:

```text id="fx7n4m"
Text
Button
Image
```

---

## DG-002 Extensibility

New widgets must be added without redesigning the AST structure.

Examples:

```text id="4wpn4t"
Grid
Map
Chart
VideoPlayer
```

---

## DG-003 Generator Compatibility

Every generator must understand NodeKind values.

Example:

```text id="b6l3sm"
Text
```

Generated as:

```text id="t1u4gq"
SwiftUI -> Text

Compose -> Text
```

---

## DG-004 Future Platform Support

The same NodeKind must support:

* SwiftUI
* Compose
* Web
* Desktop
* WASM

---

# 3. NodeKind Architecture

NodeKind defines the behavior of a Node.

Example:

```rust id="k5p7e1"
Node {
    kind: Text
}
```

The node structure remains identical.

Only NodeKind changes.

---

# 4. NodeKind Enumeration

```rust id="g8r9c4"
pub enum NodeKind {
    Text,
    Button,
    Image,
    TextField,

    VStack,
    HStack,
    ZStack,

    ScrollView,

    Spacer,

    NavigationView,
    NavigationLink,

    List,
    Grid,

    Custom(String),
}
```

---

# 5. Node Categories

NodeKinds are grouped into categories.

```text id="q7n4hs"
Leaf Widgets

Container Widgets

Navigation Widgets

Collection Widgets

Custom Widgets
```

---

# 6. Leaf Widgets

Leaf widgets cannot contain children.

---

## Text

Displays textual content.

Example:

```rust id="n3j7s5"
Text::new("Hello")
```

---

## Button

Displays interactive button content.

Example:

```rust id="u2x8k1"
Button::new("Login")
```

---

## Image

Displays images.

Example:

```rust id="r5v2n9"
Image::new("logo.png")
```

---

## TextField

Displays editable text input.

Example:

```rust id="h4s9c6"
TextField::new()
```

---

## Spacer

Represents flexible layout spacing.

Example:

```rust id="m8w3d2"
Spacer::new()
```

---

# 7. Container Widgets

Container widgets may contain children.

---

## VStack

Vertical layout container.

Example:

```text id="a5n7p4"
VStack
 ├── Text
 └── Button
```

---

## HStack

Horizontal layout container.

Example:

```text id="v9j6x1"
HStack
 ├── Text
 └── Button
```

---

## ZStack

Layered layout container.

Example:

```text id="d8m2s7"
ZStack
 ├── Image
 └── Text
```

---

## ScrollView

Scrollable container.

Example:

```text id="l7f4q8"
ScrollView
 └── VStack
```

---

# 8. Navigation Widgets

Navigation widgets describe screen transitions.

---

## NavigationView

Navigation root container.

Example:

```rust id="t4h8j3"
NavigationView
```

---

## NavigationLink

Navigation trigger.

Example:

```rust id="s6x9p1"
NavigationLink
```

---

# 9. Collection Widgets

Collection widgets display multiple items.

---

## List

Displays vertical collections.

Example:

```rust id="c3m8k7"
List
```

---

## Grid

Displays grid collections.

Example:

```rust id="z1f7w5"
Grid
```

---

# 10. Custom Widgets

Custom widgets enable plugin support.

---

## Design

```rust id="y5n2h8"
Custom(String)
```

---

## Example

```rust id="g9v4k6"
Custom(
    "ChartView"
)
```

---

## Purpose

Supports:

* Plugin ecosystem
* Third-party widgets
* Company-specific widgets

---

# 11. Widget Hierarchy

```text id="w4x7j2"
NodeKind

├── Leaf Widgets
│
│   ├── Text
│   ├── Button
│   ├── Image
│   ├── TextField
│   └── Spacer
│
├── Container Widgets
│
│   ├── VStack
│   ├── HStack
│   ├── ZStack
│   └── ScrollView
│
├── Navigation Widgets
│
│   ├── NavigationView
│   └── NavigationLink
│
├── Collection Widgets
│
│   ├── List
│   └── Grid
│
└── Custom Widgets
```

---

# 12. Child Support Matrix

| NodeKind       | Children Allowed  |
| -------------- | ----------------- |
| Text           | No                |
| Button         | No                |
| Image          | No                |
| TextField      | No                |
| Spacer         | No                |
| VStack         | Yes               |
| HStack         | Yes               |
| ZStack         | Yes               |
| ScrollView     | Yes               |
| NavigationView | Yes               |
| NavigationLink | Yes               |
| List           | Yes               |
| Grid           | Yes               |
| Custom         | Depends on plugin |

---

# 13. Generator Mapping

## SwiftUI

| NodeKind       | SwiftUI         |
| -------------- | --------------- |
| Text           | Text            |
| Button         | Button          |
| Image          | Image           |
| VStack         | VStack          |
| HStack         | HStack          |
| ZStack         | ZStack          |
| ScrollView     | ScrollView      |
| NavigationView | NavigationStack |
| List           | List            |
| Grid           | LazyVGrid       |

---

## Compose

| NodeKind       | Compose          |
| -------------- | ---------------- |
| Text           | Text             |
| Button         | Button           |
| Image          | Image            |
| VStack         | Column           |
| HStack         | Row              |
| ZStack         | Box              |
| ScrollView     | Column + Scroll  |
| NavigationView | NavHost          |
| List           | LazyColumn       |
| Grid           | LazyVerticalGrid |

---

# 14. Validation Rules

## NK-001

Every node must have a NodeKind.

---

## NK-002

NodeKind cannot be null.

---

## NK-003

Leaf widgets cannot have children.

Example:

Invalid:

```text id="j8n5v1"
Text
 └── Button
```

---

## NK-004

Container widgets may contain any valid Node.

---

## NK-005

Custom widgets must define a unique identifier.

Example:

```rust id="u4h9p7"
Custom(
    "ChartView"
)
```

---

# 15. Future Expansion Strategy

Future NodeKinds may include:

```text id="f7x2m8"
TabView

Sheet

Alert

Dialog

BottomSheet

Map

VideoPlayer

WebView

Canvas

Chart

LazyVStack

LazyHStack
```

These additions must not require changes to:

```text id="d5w8q1"
Node

Property System

Modifier System

Binding System
```

---

# 16. Architectural Decisions

| Decision                       | Reason                     |
| ------------------------------ | -------------------------- |
| NodeKind enum used             | Strong typing              |
| Categories introduced          | Simplifies validation      |
| Custom widget support included | Plugin readiness           |
| Navigation widgets included    | Future navigation system   |
| Collection widgets included    | Scalable data presentation |
| Platform independence enforced | Multi-generator support    |

---

# 17. Acceptance Criteria

This design is considered complete when:

* NodeKind enumeration is defined
* Widget categories are documented
* Child support rules are documented
* Generator mappings are documented
* Validation rules are documented
* Future expansion strategy is documented
* Custom widget support is documented

---