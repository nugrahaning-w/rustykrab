# State Binding System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the State Binding System used by the RustyKrab AST.

The Binding System provides a platform-independent mechanism for connecting UI nodes to application state.

Bindings allow nodes to:

* Read state values
* React to state changes
* Synchronize UI with application state
* Support two-way data binding
* Support future reactive systems

The Binding System does not manage state.

It only describes relationships between UI and state.

---

# 2. Design Goals

## DG-001 Platform Independence

Bindings must not contain platform-specific concepts.

Invalid:

```text id="s5i7g2"
@State
@Binding
mutableStateOf
LiveData
StateFlow
```

Valid:

```text id="l9k3h8"
StateReference
Binding
BindingMode
```

---

## DG-002 Reactive Ready

Bindings must support future reactive state systems.

Example:

```text id="f2r8c1"
State Changed
     │
     ▼
Binding Updated
     │
     ▼
UI Updated
```

---

## DG-003 Generator Agnostic

The same binding definition must work for:

* SwiftUI
* Compose
* Web
* Desktop

---

## DG-004 Two-Way Binding Support

The system must support:

* Read-only binding
* Write-only binding
* Two-way binding

---

## DG-005 Extensibility

Future state implementations must not require AST redesign.

---

# 3. Architecture Overview

```text id="v1d6n5"
State
   │
   ▼
Binding
   │
   ▼
Node Property
```

Bindings connect a state source to a node property.

---

# 4. Binding Model

## Binding

```rust id="t8g4x1"
pub struct Binding {
    pub id: BindingId,

    pub property: String,

    pub state: StateReference,

    pub mode: BindingMode,
}
```

---

# 5. Binding Components

## BindingId

Unique identifier for a binding.

```rust id="n3c7m9"
pub struct BindingId(
    pub String
);
```

Example:

```rust id="h7j4v2"
BindingId(
    "counter_value"
)
```

---

## Property

Identifies the node property that receives data.

Example:

```rust id="m2f9s8"
"value"
```

or

```rust id="j6v1p4"
"text"
```

or

```rust id="r8k3d5"
"is_visible"
```

---

## StateReference

References an application state source.

```rust id="b4n8x6"
pub struct StateReference {
    pub id: StateId,

    pub name: String,
}
```

---

Example:

```rust id="x1c5j9"
StateReference {
    id: StateId("counter"),

    name: "counter"
}
```

---

## BindingMode

Defines synchronization behavior.

```rust id="z6d2r7"
pub enum BindingMode {
    Read,

    Write,

    TwoWay,
}
```

---

# 6. Binding Modes

## Read Binding

Reads data from state.

State:

```text id="e7h5r3"
counter
```

Node:

```text id="c2k8v4"
Text
```

Flow:

```text id="p5j7n1"
State
 ↓
Text
```

---

## Write Binding

Writes data to state.

Flow:

```text id="o9v2m6"
Input
 ↓
State
```

---

## TwoWay Binding

Reads and writes.

Flow:

```text id="u4x8b2"
State
 ↕
Input
```

---

# 7. Example Bindings

## Text Display

DSL:

```rust id="g1m8c7"
Text::new(counter)
```

AST:

```json id="y2v4n8"
{
  "bindings": [
    {
      "property": "value",
      "state": {
        "name": "counter"
      },
      "mode": "Read"
    }
  ]
}
```

---

## TextField Input

DSL:

```rust id="r5x2h6"
TextField::new()
    .bind("email")
```

AST:

```json id="f3k9m1"
{
  "bindings": [
    {
      "property": "value",
      "state": {
        "name": "email"
      },
      "mode": "TwoWay"
    }
  ]
}
```

---

## Visibility Binding

DSL:

```rust id="n7p4d8"
Text::new("Loading")
    .visible(isLoading)
```

AST:

```json id="k8c1v5"
{
  "bindings": [
    {
      "property": "is_visible",
      "state": {
        "name": "isLoading"
      },
      "mode": "Read"
    }
  ]
}
```

---

# 8. Multiple Bindings

A node may contain multiple bindings.

Example:

```rust id="j4t8w3"
TextField::new()
```

AST:

```json id="q2h7m9"
{
  "bindings": [
    {
      "property": "value",
      "state": {
        "name": "email"
      }
    },
    {
      "property": "is_enabled",
      "state": {
        "name": "isEditable"
      }
    }
  ]
}
```

---

# 9. Reactive Update Flow

```text id="w6n4j8"
State Updated
      │
      ▼
Binding Updated
      │
      ▼
Node Updated
      │
      ▼
Generator Runtime
      │
      ▼
UI Refreshed
```

The AST remains unchanged.

Only state values change.

---

# 10. Generator Mapping

## SwiftUI

Read Binding:

```swift id="e9x3v6"
Text("\(counter)")
```

TwoWay Binding:

```swift id="q1f8h2"
TextField(
    "",
    text: $email
)
```

---

## Compose

Read Binding:

```kotlin id="c5m7r1"
Text(counter.toString())
```

TwoWay Binding:

```kotlin id="d8n2p4"
TextField(
    value = email,
    onValueChange = {
        email = it
    }
)
```

---

# 11. State Ownership

Bindings do not own state.

Invalid:

```text id="v3x8m5"
Binding
  └── State Value
```

Valid:

```text id="u7d4j2"
Binding
  └── State Reference
```

State ownership belongs to the future State Management System.

---

# 12. Validation Rules

## BS-001

Every binding must reference a property.

Valid:

```json id="k4h7v2"
{
  "property": "value"
}
```

Invalid:

```json id="z8n1f5"
{
}
```

---

## BS-002

Every binding must reference a state.

Valid:

```json id="x5j3m8"
{
  "state": {
    "name": "counter"
  }
}
```

Invalid:

```json id="r6p9d4"
{
  "property": "value"
}
```

---

## BS-003

Binding mode must be defined.

Valid:

```json id="n2k8h6"
{
  "mode": "Read"
}
```

Invalid:

```json id="m7v1c3"
{
}
```

---

## BS-004

Binding identifiers must be unique within a node.

---

# 13. Serialization Example

```json id="y8f2r6"
{
  "kind": "Text",
  "bindings": [
    {
      "id": "counter_text",
      "property": "value",
      "state": {
        "id": "counter",
        "name": "counter"
      },
      "mode": "Read"
    }
  ]
}
```

---

# 14. Why Use Bindings?

Rejected Design:

```rust id="q5n9k2"
Text::new(
    counter.get()
)
```

Problems:

* No state relationship information
* Difficult code generation
* No reactive metadata
* No tooling support

---

Selected Design:

```rust id="b1m4r8"
Binding {
    property: "value",

    state: "counter",

    mode: Read
}
```

Benefits:

* Explicit state relationship
* Generator-friendly
* Reactive-ready
* Serializable
* Tooling-friendly

---

# 15. Future Expansion

Future binding targets:

```text id="g7h2n5"
Computed State

Derived State

Remote Data

Cache Data

Environment Values

Theme Values
```

Future binding modes:

```text id="m3v8r1"
OneTime

Lazy

Computed
```

No AST redesign should be required.

---

# 16. Integration with Future Systems

The Binding System will integrate with:

```text id="p9f6d2"
State Management

Navigation

Dependency Injection

Theme System

Localization
```

through references only.

The AST remains platform-independent.

---

# 17. Architectural Decisions

| Decision                          | Reason                                     |
| --------------------------------- | ------------------------------------------ |
| Binding separated from Properties | Explicit state relationships               |
| StateReference used               | No state ownership                         |
| BindingMode introduced            | Supports multiple synchronization patterns |
| Multiple bindings per node        | Flexibility                                |
| Generator-independent design      | Multi-platform support                     |
| Reactive architecture supported   | Future-proofing                            |

---

# 18. Acceptance Criteria

This design is considered complete when:

* Binding model is defined
* StateReference is defined
* BindingMode is defined
* Reactive flow is documented
* Generator mappings are documented
* Validation rules are documented
* Serialization examples are provided
* Future expansion strategy is documented

---