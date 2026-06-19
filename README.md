# 🦀 RustyKrab

<p align="center">
  <img src="docs/RustyKrab.png" width="240" alt="RustyKrab Logo">
</p>

# Rust-First Native UI Compiler for Mobile Multiplatform Development

RustyKrab is an experimental Rust-first framework for building mobile applications using a declarative UI architecture.

Unlike Flutter, React Native, or other runtime-based frameworks, RustyKrab does not ship a rendering engine.

Instead, RustyKrab compiles a platform-independent UI representation into fully native source code.

Current Targets:

- iOS → SwiftUI
- Android → Jetpack Compose

Future Targets:

- Web
- Desktop
- WASM
- Embedded UI

---

# Vision

> Write UI once in Rust. Generate native applications everywhere.

RustyKrab aims to become a complete application platform capable of generating native applications for multiple environments without introducing runtime rendering layers.

---

# Why RustyKrab?

Modern cross-platform frameworks often introduce:

- Rendering engines
- Runtime abstraction layers
- Platform bridges
- Performance overhead
- Increased application size

RustyKrab takes a different approach.

Developer Code:

```rust
App::new(
    VStack::new()
        .child(
            Text::new("Welcome")
        )
        .child(
            Button::new("Login")
        )
)
```

Generated SwiftUI:

```swift
VStack {
    Text("Welcome")

    Button("Login") {
    }
}
```

Generated Jetpack Compose:

```kotlin
Column {
    Text("Welcome")

    Button(
        onClick = {}
    ) {
        Text("Login")
    }
}
```

No rendering engine.

No bridge.

No UI runtime.

Only native applications.

---

# Core Principles

## Native First

Generated applications use platform-native technologies.

- SwiftUI
- Jetpack Compose

---

## AST Driven

All generators consume AST.

```text
DSL
 ↓
AST
 ↓
Generator
```

Generators never read DSL structures directly.

---

## Platform Independent

The AST never contains:

- SwiftUI-specific concepts
- Compose-specific concepts
- Android-specific concepts
- iOS-specific concepts

---

## Compile-Time Generation

Most work happens during compilation.

Runtime complexity is minimized.

---

## Extensible Architecture

RustyKrab is designed to support:

- New widgets
- New generators
- New platforms
- Plugin systems

without redesigning the AST.

---

# Architecture

```text
Developer Code
       │
       ▼
   Widget DSL
       │
       ▼
   Widget Tree
       │
       ▼
       AST
       │
       ▼
  Validation
       │
       ▼
   Generator
   ┌────┴────┐
   ▼         ▼
SwiftUI   Compose
```

The AST acts as the single source of truth for every generator.

---

# Workspace Structure

```text
rustykrab/

├── Cargo.toml

├── crates/
│
├── rustykrab-core/
├── rustykrab-widget/
├── rustykrab-ast/
├── rustykrab-parser/
├── rustykrab-generator/
│
├── rustykrab-swiftui/
├── rustykrab-compose/
│
├── rustykrab-cli/
│
├── examples/
├── docs/
├── templates/
└── tests/
```

---

# Workspace Modules

## rustykrab-core

Shared framework primitives.

Responsibilities:

- Error handling
- Shared traits
- Utilities
- Configuration

---

## rustykrab-widget

Declarative UI DSL.

Example:

```rust
Text::new("Hello")
```

---

## rustykrab-ast

Platform-independent UI representation.

Responsibilities:

- AST Root
- Node System
- Property System
- Modifier System
- Event System
- Binding System
- Metadata System

---

## rustykrab-parser

Transforms widget trees into AST.

Responsibilities:

- Parsing
- Validation Entry Point
- AST Construction

---

## rustykrab-generator

Shared generator abstractions.

Responsibilities:

- Generator Contracts
- Visitor APIs
- Shared Utilities

---

## rustykrab-swiftui

SwiftUI backend.

Responsibilities:

- SwiftUI Generation
- Xcode Project Generation

---

## rustykrab-compose

Jetpack Compose backend.

Responsibilities:

- Compose Generation
- Android Project Generation

---

## rustykrab-cli

Command Line Interface.

Responsibilities:

- Project Creation
- Build
- Run
- Environment Diagnostics

---

# AST Foundation

RustyKrab uses a generic AST architecture.

## AST Root

```rust
pub struct Ast {
    pub version: AstVersion,
    pub root: Node,
}
```

---

## Node Model

```rust
pub struct Node {
    pub id: NodeId,

    pub kind: NodeKind,

    pub properties: PropertyMap,

    pub modifiers: ModifierChain,

    pub events: Vec<EventHandler>,

    pub bindings: Vec<Binding>,

    pub metadata: Metadata,

    pub children: Vec<Node>,
}
```

---

# AST Architecture Layers

```text
AST
│
├── Node System
├── Property System
├── Modifier System
├── Event System
├── Binding System
└── Metadata System
```

---

# Node System

Provides:

- Node
- NodeId
- NodeKind
- Tree Management
- Children Management

Example:

```rust
let root = Node::new(
    NodeId::new("root"),
    NodeKind::Container,
);
```

---

# Property System

Properties provide flexible widget configuration.

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
);
```

AST Representation:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello RustyKrab"
  }
}
```

Current Features:

- PropertyValue
- PropertyObject
- PropertyMap
- PropertyAccess
- Property Validation
- Standard Property Keys
- Property Builder APIs

---

# Modifier System

Inspired by:

- SwiftUI Modifiers
- Jetpack Compose Modifiers

Current Features:

- ModifierKind
- ModifierValue
- Modifier
- ModifierChain
- Validation Rules
- Categories
- Builder APIs

Example:

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

AST:

```text
Text
└── Modifiers
     ├── Padding(16)
     ├── Background("#FF0000")
     ├── CornerRadius(8)
     └── Opacity(0.8)
```

Supported Categories:

## Layout

- Padding
- Margin
- Width
- Height
- Alignment

## Style

- Background
- Foreground Color
- Border
- Corner Radius

## Visual Effects

- Opacity
- Shadow
- Blur

## Accessibility

- Accessibility Label
- Accessibility Identifier

## Animation

- Animation
- Transition

---

# Event System

🚧 Planned

Examples:

```rust
Button::new("Login")
    .on_click("login")
```

Future AST:

```json
{
  "event": "click",
  "action": "login"
}
```

---

# State Binding System

🚧 Planned

Examples:

```rust
Text::new(counter)
```

Future AST:

```json
{
  "binding": {
    "property": "value",
    "state": "counter"
  }
}
```

---

# Example AST

```rust
let screen = Node::new(
    NodeId::new("screen"),
    NodeKind::Container,
)
.child(
    Node::new(
        NodeId::new("title"),
        NodeKind::Text,
    )
    .property(
        PropertyKeys::VALUE,
        PropertyValue::String(
            "Welcome".into(),
        ),
    )
    .padding(16)
    .background("#FF0000")
);
```

Conceptual AST:

```text
Container
└── Text
     ├── Properties
     │
     │    value = "Welcome"
     │
     └── Modifiers
          ├── Padding(16)
          └── Background("#FF0000")
```

---

# CLI

Planned commands:

```bash
krab create MyApp

krab build ios

krab build android

krab run ios

krab run android

krab doctor
```

---

# Current Development Status

## Completed

### Workspace & Tooling

- Cargo Workspace
- Rustfmt
- Clippy
- Makefile
- EditorConfig

### AST Foundation

- AST Root
- Node System
- Metadata System
- Property System
- Modifier System

---

## In Progress

- Event System
- Binding System

---

## Planned

- Validation Framework
- Visitor Pattern
- Serialization
- Widget DSL
- Parser
- SwiftUI Generator
- Compose Generator
- CLI

---

# Roadmap

## Milestone 1

Compiler Core

- Workspace
- Tooling
- AST Foundation
- Parser Foundation
- Generator Contracts

---

## Milestone 2

Widget DSL

- Widget APIs
- Layout Widgets
- Property Integration
- Modifier Integration

---

## Milestone 3

SwiftUI Generator

- SwiftUI Visitor
- Swift Source Generation
- Xcode Project Generation

---

## Milestone 4

Compose Generator

- Compose Visitor
- Kotlin Source Generation
- Android Project Generation

---

## Milestone 5

Reactive State

- State Management
- Binding Runtime
- Update Propagation

---

## Milestone 6

Navigation

- Navigator API
- Navigation AST Integration

---

## Milestone 7

Plugin System

- Custom Widgets
- Custom Generators
- Extension APIs

---

# Contributing

RustyKrab is currently in active early development.

Contributors are welcome.

Areas where help is needed:

- AST Development
- Widget DSL
- Parser
- SwiftUI Generation
- Compose Generation
- Documentation
- Testing
- Examples

Please check project issues and discussions before starting work.

---

# License

License information will be added before the first public release.

---

# Status

🚧 Early Development

Current Focus:

- AST Foundation
- Event System
- Binding System

The project is not production-ready yet.