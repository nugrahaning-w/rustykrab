# Generic Node Model

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the Generic Node Model used by the RustyKrab AST.

The Node Model represents the fundamental building block of every AST tree.

All widgets, containers, layouts, navigation components, and future custom widgets must be represented using the same generic node structure.

The Generic Node Model is designed to:

* Minimize AST redesign
* Simplify generator implementation
* Support future platform expansion
* Enable plugin support
* Support state management
* Support event handling
* Support modifiers

---

# 2. Design Goals

## DG-001 Generic Representation

All UI elements must use the same node structure.

Invalid:

```text id="ebh9r1"
TextNode
ButtonNode
ImageNode
VStackNode
```

Preferred:

```text id="v4u71n"
Node {
    kind: Text
}

Node {
    kind: Button
}
```

---

## DG-002 Extensibility

New widgets must be added without introducing new AST structures.

Examples:

```text id="s44vdn"
Grid
List
Map
Chart
VideoPlayer
WebView
```

---

## DG-003 Platform Independence

The node model must not contain platform-specific concepts.

Invalid:

```text id="mow7yl"
SwiftUIButton
ComposeButton
AndroidText
```

Valid:

```text id="hz9r9z"
Button
Text
Image
```

---

## DG-004 Generator Compatibility

Every generator must consume the same node structure.

Supported:

* SwiftUI Generator
* Compose Generator
* Web Generator
* Desktop Generator

---

## DG-005 Plugin Support

Third-party widgets must be representable.

Example:

```text id="h7r5nv"
ChartView
CameraView
QRCodeScanner
```

---

# 3. Node Overview

Every AST tree is composed of Nodes.

```text id="j7y8oo"
Ast
 │
 └── Node
      │
      ├── Node
      ├── Node
      └── Node
```

Nodes may contain children.

Nodes may also act as leaf elements.

---

# 4. Node Structure

## Proposed Model

```rust id="49x5n5"
pub struct Node {
    pub id: NodeId,

    pub kind: NodeKind,

    pub properties: PropertyMap,

    pub modifiers: Vec<Modifier>,

    pub events: Vec<EventHandler>,

    pub bindings: Vec<Binding>,

    pub metadata: Metadata,

    pub children: Vec<Node>,
}
```

This structure is the official AST node contract.

---

# 5. Node Components

## NodeId

Unique identifier of a node.

Example:

```rust id="oc87k4"
NodeId("node_001")
```

Purpose:

* Tree tracking
* Debugging
* Validation
* Tooling

---

## NodeKind

Represents widget type.

Example:

```rust id="b5yfv5"
Text
Button
Image
VStack
```

NodeKind determines generator behavior.

Example:

```text id="2lv65u"
NodeKind::Text

→ SwiftUI Text

→ Compose Text
```

---

## Properties

Represents widget configuration.

Example:

```rust id="jlwm8z"
{
    "value": "Hello World"
}
```

or

```rust id="k3g3l4"
{
    "title": "Login"
}
```

Properties are widget-specific.

---

## Modifiers

Represents visual modifications.

Example:

```rust id="b3u8pb"
Padding(16)

Foreground(Blue)

Width(100)
```

Modifiers are generator-independent.

---

## Events

Represents user interactions.

Example:

```rust id="r2sqn7"
Click

Change

Submit
```

Generators convert events into platform-specific handlers.

---

## Bindings

Represents state connections.

Example:

```rust id="2uxw2d"
counter
userName
email
```

Bindings are used by the reactive system.

---

## Metadata

Represents source information.

Example:

```rust id="zhm2db"
main.rs

line 10

column 5
```

Used for:

* Diagnostics
* Error reporting
* IDE support

---

## Children

Represents hierarchy.

Example:

```text id="n8vvrm"
VStack
 ├── Text
 └── Button
```

Children define layout relationships.

---

# 6. Node Classification

Nodes are classified into two categories.

---

## Leaf Nodes

Leaf nodes cannot contain children.

Examples:

```text id="jmkxf3"
Text
Button
Image
TextField
Spacer
```

Valid:

```text id="8m86nn"
Text
```

Invalid:

```text id="7s67li"
Text
 └── Button
```

---

## Container Nodes

Container nodes can contain children.

Examples:

```text id="sqe1k8"
VStack
HStack
ZStack
ScrollView
List
Grid
```

Valid:

```text id="aq6t9c"
VStack
 ├── Text
 └── Button
```

---

# 7. Node Lifecycle

A node progresses through the following lifecycle.

```text id="wr3s5w"
DSL Widget

↓

Widget Tree

↓

AST Node

↓

Validation

↓

Generation
```

---

# 8. Tree Structure Rules

## NR-001 Every Node Must Have NodeKind

Valid:

```rust id="w5k9k8"
kind: Text
```

Invalid:

```rust id="4im5sv"
kind: null
```

---

## NR-002 Every Node Must Have NodeId

Valid:

```rust id="nuwc65"
id: node_001
```

---

## NR-003 Child Relationships Must Be Valid

Example:

```text id="2fjlwm"
Text
 └── Button
```

Invalid.

---

## NR-004 Children Must Preserve Order

Valid:

```text id="e7qf5y"
VStack
 ├── Text
 ├── Image
 └── Button
```

Order must be maintained during generation.

---

# 9. Example Node Models

## Text Widget

```json id="5dhv2z"
{
  "id": "1",
  "kind": "Text",
  "properties": {
    "value": "Welcome"
  },
  "modifiers": [],
  "events": [],
  "bindings": [],
  "children": []
}
```

---

## Button Widget

```json id="y0c2n0"
{
  "id": "2",
  "kind": "Button",
  "properties": {
    "title": "Login"
  },
  "events": [
    {
      "event": "Click",
      "action": "login"
    }
  ]
}
```

---

## Container Widget

```json id="efq1bk"
{
  "id": "3",
  "kind": "VStack",
  "children": [
    {
      "kind": "Text"
    },
    {
      "kind": "Button"
    }
  ]
}
```

---

# 10. Why Generic Node Instead of Dedicated Nodes?

Rejected Design:

```rust id="ls2m35"
TextNode

ButtonNode

ImageNode

VStackNode
```

Problems:

* Large number of structs
* Difficult maintenance
* Difficult extensibility
* Complex generator logic

---

Selected Design:

```rust id="3v1yoq"
Node {
    kind
    properties
}
```

Benefits:

* Smaller codebase
* Easier generators
* Easier serialization
* Easier plugin support
* Easier future expansion

---

# 11. Future Expansion

The Generic Node Model is designed to support future systems.

Examples:

```text id="f2mq26"
Animation System

Theme System

Accessibility System

Localization System

Plugin System
```

Additional fields may be added without redesigning the entire AST.

---

# 12. Architectural Decisions

| Decision                      | Reason                      |
| ----------------------------- | --------------------------- |
| Generic Node Model            | Simplifies expansion        |
| NodeKind-based behavior       | Reduces AST complexity      |
| Children stored in Node       | Simplifies traversal        |
| Properties stored dynamically | Supports future widgets     |
| Metadata included             | Better diagnostics          |
| Events included               | Supports interaction system |
| Bindings included             | Supports reactive state     |

---

# 13. Acceptance Criteria

This design is considered complete when:

* Generic Node structure is defined
* Node responsibilities are documented
* Node classification is documented
* Tree rules are documented
* Lifecycle is documented
* Example models are provided
* Architectural decisions are documented

---