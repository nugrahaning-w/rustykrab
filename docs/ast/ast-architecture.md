# AST Architecture

**Project:** RustyKrab
**Document Type:** Architecture Design
**Version:** 1.0
**Status:** Draft
**Author:** Aji Nugrahaning Widhi

---

# 1. Purpose

This document defines the high-level architecture of the RustyKrab Abstract Syntax Tree (AST) system.

The AST acts as the central intermediate representation between Widget DSL and platform-specific code generators.

All platform generators must consume AST instead of directly processing Widget DSL structures.

This architecture enables:

* Platform independence
* Extensibility
* Validation
* Serialization
* Tooling support
* Future platform support

---

# 2. Architectural Goals

The AST architecture is designed to satisfy the following goals.

## AG-001 Platform Independence

The AST must not contain any platform-specific concepts.

Invalid:

```text
SwiftUITextNode
ComposeTextNode
AndroidButtonNode
```

Valid:

```text
Text
Button
Image
```

---

## AG-002 Generator Agnostic

The same AST must be usable by multiple generators.

Examples:

* SwiftUI Generator
* Compose Generator
* Web Generator
* Desktop Generator
* WASM Generator

---

## AG-003 Extensibility

New widgets must be added without redesigning the AST architecture.

Example:

```text
List
Grid
LazyVStack
TabView
MapView
VideoPlayer
```

---

## AG-004 Validation Support

The AST must support validation before generation.

Examples:

* Text cannot have children
* Image must have source
* Root node must exist

---

## AG-005 Serialization Support

The AST must support serialization and deserialization.

Supported formats:

* JSON
* YAML (future)
* Binary (future)

---

## AG-006 Tooling Support

The AST must support:

* Debugging
* Error reporting
* Code analysis
* IDE integration

---

# 3. High-Level Architecture

RustyKrab follows a compiler architecture.

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

The AST serves as the single source of truth for all platform generators.

---

# 4. Compiler Flow

## Step 1 — Widget DSL

Developer writes UI using RustyKrab DSL.

Example:

```rust
App::new(
    VStack::new()
        .child(Text::new("Welcome"))
        .child(Button::new("Login"))
)
```

---

## Step 2 — Widget Tree

The DSL constructs a Widget Tree.

Example:

```text
VStack
├── Text
└── Button
```

The Widget Tree remains platform-independent.

---

## Step 3 — AST Generation

The parser converts the Widget Tree into AST.

Example:

```text
Widget Tree
    │
    ▼
Parser
    │
    ▼
AST
```

---

## Step 4 — Validation

The AST is validated before generation.

Validation Examples:

```text
Text cannot contain children

Image must have source

NodeKind must be valid
```

Invalid AST structures stop generation.

---

## Step 5 — Code Generation

Generators traverse the AST using the Visitor Pattern.

Example:

```text
AST
 │
 ▼
Visitor
 │
 ▼
SwiftUI Generator
```

or

```text
AST
 │
 ▼
Visitor
 │
 ▼
Compose Generator
```

---

## Step 6 — Native Project Generation

Generated source code is inserted into native project templates.

Output:

```text
build/

├── ios/
│   └── SwiftUI Project

└── android/
    └── Compose Project
```

---

# 5. Ownership Architecture

Each crate has a clearly defined responsibility.

```text
rustykrab-widget
        │
        ▼
rustykrab-parser
        │
        ▼
rustykrab-ast
        │
        ▼
rustykrab-generator
       / \
      /   \
     ▼     ▼
swiftui compose
```

---

## rustykrab-widget

Responsibilities:

* Widget DSL
* Widget Composition
* Widget Builder APIs

Outputs:

```text
Widget Tree
```

---

## rustykrab-parser

Responsibilities:

* Widget Tree Analysis
* AST Construction
* Validation Entry Point

Outputs:

```text
AST
```

---

## rustykrab-ast

Responsibilities:

* AST Models
* Property System
* Modifier System
* Event System
* Binding System
* Metadata System

Outputs:

```text
Platform-independent AST
```

---

## rustykrab-generator

Responsibilities:

* Generator Contracts
* Visitor Interfaces
* Shared Generation Utilities

Outputs:

```text
Generator APIs
```

---

## rustykrab-swiftui

Responsibilities:

* SwiftUI Code Generation

Outputs:

```text
SwiftUI Source Code
```

---

## rustykrab-compose

Responsibilities:

* Jetpack Compose Code Generation

Outputs:

```text
Compose Source Code
```

---

# 6. AST as Single Source of Truth

The AST is the only structure that generators may consume.

Forbidden:

```text
Widget DSL
   │
   ▼
SwiftUI Generator
```

Forbidden:

```text
Widget Tree
   │
   ▼
Compose Generator
```

Required:

```text
Widget DSL
   │
   ▼
Widget Tree
   │
   ▼
AST
   │
   ▼
Generator
```

This guarantees consistent behavior across platforms.

---

# 7. Future Expansion Strategy

The architecture must support future features without redesign.

Future Widgets:

```text
List
Grid
LazyVStack
LazyHStack
TabView
NavigationStack
Map
VideoPlayer
WebView
```

Future Platforms:

```text
Web
Desktop
WASM
```

Future Systems:

```text
Theme System
Animation System
Accessibility System
Localization System
Plugin System
```

The AST architecture must remain stable while supporting these additions.

---

# 8. Architectural Decisions

| Decision                            | Reason                                   |
| ----------------------------------- | ---------------------------------------- |
| AST is platform-independent         | Enables multiple generators              |
| AST is generator-agnostic           | Supports future platforms                |
| Validation occurs before generation | Prevents invalid source output           |
| Visitor Pattern used for traversal  | Simplifies generator implementation      |
| Metadata included in AST            | Improves diagnostics                     |
| Generic Node Model used             | Supports future extensibility            |
| Property System used                | Reduces AST redesign risk                |
| Modifier System used                | Aligns with SwiftUI and Compose concepts |

---

# 9. Success Criteria

This architecture is considered successful when:

* Widget DSL can be transformed into AST
* AST can be validated
* AST can be serialized
* AST can be traversed
* SwiftUI Generator consumes AST
* Compose Generator consumes AST
* New widgets can be added without AST redesign
* New generators can be implemented without AST changes

---
