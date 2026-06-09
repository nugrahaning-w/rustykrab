# 🦀 RustyKrab
<img src="docs/RustyKrab.png" alt="Icon Description" width="240" height="240" align="center">

**Rust-First Native UI Compiler for Mobile Multiplatform Development**

RustyKrab is an experimental cross-platform application framework built with Rust that compiles a declarative UI DSL into fully native mobile applications.

Unlike Flutter, React Native, or other runtime-based frameworks, RustyKrab does not ship a rendering engine.

Instead, RustyKrab generates native source code for each target platform.

Current Targets:

* iOS → SwiftUI
* Android → Jetpack Compose

Future Targets:

* Web
* Desktop
* WASM

---

# Why RustyKrab?

Modern cross-platform frameworks often introduce:

* Additional runtime layers
* Rendering engines
* Platform bridges
* Performance overhead

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

Generated Output:

### iOS

```swift
VStack {
    Text("Welcome")

    Button("Login") {
    }
}
```

### Android

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

No runtime UI abstraction.

Just native applications.

---

# Architecture

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

The AST acts as the single source of truth for all platform generators.

---

# Core Principles

## Native First

RustyKrab always generates native platform code.

Generated applications use:

* SwiftUI on iOS
* Jetpack Compose on Android

---

## AST Driven

All generators consume AST.

Generators never read DSL structures directly.

```text
DSL
 ↓
AST
 ↓
Generator
```

---

## Platform Independent

The AST does not contain:

* SwiftUI-specific concepts
* Compose-specific concepts
* Android-specific concepts
* iOS-specific concepts

---

## Compile-Time Generation

Most work is performed during compilation.

Runtime complexity is minimized.

---

## Extensibility First

RustyKrab is designed to support:

* New widgets
* New generators
* Plugin systems
* Additional platforms

without redesigning the AST.

---

# Project Structure

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
├── templates/
├── docs/
└── tests/
```

---

# Workspace Modules

## rustykrab-core

Shared framework primitives.

Responsibilities:

* Error handling
* Result types
* Shared traits
* Configuration
* Utilities

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

* Node model
* Property system
* Modifier system
* Event system
* Binding system
* Metadata system

---

## rustykrab-parser

Converts widget trees into AST.

Responsibilities:

* AST generation
* Validation entry point

---

## rustykrab-generator

Generator abstractions.

Responsibilities:

* Generator contracts
* Visitor interfaces
* Shared generation utilities

---

## rustykrab-swiftui

SwiftUI backend.

Responsibilities:

* SwiftUI generation
* iOS project generation

---

## rustykrab-compose

Jetpack Compose backend.

Responsibilities:

* Compose generation
* Android project generation

---

## rustykrab-cli

Command line interface.

Responsibilities:

* Project creation
* Build
* Run
* Environment validation

---

# AST Foundation

RustyKrab uses a generic AST architecture.

```rust
pub struct Ast {
    pub version: AstVersion,
    pub root: Node,
}
```

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

This design enables:

* Scalability
* Plugin support
* Future generators
* Future widgets

without AST redesign.

---

# Widget System

Current planned widgets:

### Basic Widgets

* Text
* Button
* Image
* TextField
* Spacer

### Layout Widgets

* VStack
* HStack
* ZStack
* ScrollView

### Navigation Widgets

* NavigationView
* NavigationLink

### Collection Widgets

* List
* Grid

### Custom Widgets

```rust
NodeKind::Custom(
    "ChartView"
)
```

---

# Property System

Widget configuration is stored in a flexible property model.

Example:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello"
  }
}
```

This avoids creating specialized AST nodes for every widget type.

---

# Modifier System

Inspired by SwiftUI and Jetpack Compose.

Example:

```rust
Text::new("Welcome")
    .padding(16)
    .foreground(Color::Blue)
```

AST:

```text
Text
 └── Modifiers
      ├── Padding
      └── Foreground
```

---

# Event System

Platform-independent interaction model.

Example:

```rust
Button::new("Login")
    .on_click("login")
```

AST:

```json
{
  "event": "Click",
  "action": "login"
}
```

---

# State Binding System

Reactive UI updates are achieved through bindings.

Example:

```rust
Text::new(counter)
```

AST:

```json
{
  "binding": {
    "property": "value",
    "state": "counter"
  }
}
```

---

# Navigation

Planned API:

```rust
Navigator::push()

Navigator::pop()

Navigator::replace()
```

Mappings:

### iOS

```swift
NavigationStack
```

### Android

```kotlin
NavHost
```

---

# CLI

Create project:

```bash
krab create MyApp
```

Build iOS:

```bash
krab build ios
```

Build Android:

```bash
krab build android
```

Run iOS:

```bash
krab run ios
```

Run Android:

```bash
krab run android
```

Environment diagnostics:

```bash
krab doctor
```

---

# Roadmap

## Milestone 1

Compiler Core

* Workspace
* Tooling
* AST Foundation
* Generator Contracts
* Parser Foundation

---

## Milestone 2

Widget DSL

* Widget APIs
* Layout System
* Property Integration

---

## Milestone 3

SwiftUI Generator

* SwiftUI Visitor
* Swift Source Generation
* Xcode Project Generation

---

## Milestone 4

Compose Generator

* Compose Visitor
* Kotlin Source Generation
* Android Project Generation

---

## Milestone 5

State Management

* Reactive State
* Binding Runtime
* Update Propagation

---

## Milestone 6

Navigation

* Navigator API
* Navigation AST Integration

---

## Milestone 7

Plugin System

* Custom Widgets
* Custom Generators
* Extension APIs

---

# Long-Term Vision

RustyKrab aims to become a Rust-first application platform capable of generating native applications across multiple environments.

Future Targets:

* Mobile
* Web
* Desktop
* Embedded UI
* WASM

The ultimate goal is:

> Write UI once in Rust. Generate native applications everywhere.

---

# License

License information will be added before the first public release.

---

# Status

🚧 Early Development

The project is currently focused on building the compiler core and AST foundation.
