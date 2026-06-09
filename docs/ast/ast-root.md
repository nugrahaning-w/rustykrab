# AST Root Model

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the root model of the RustyKrab Abstract Syntax Tree (AST).

The root model serves as the entry point of the entire AST structure and provides a stable contract between:

* Widget Parser
* AST Layer
* Validation Framework
* Code Generators
* Tooling Infrastructure

Every RustyKrab application must produce exactly one AST root.

---

# 2. Design Goals

The AST root model is designed to achieve the following goals.

## DG-001 Single Entry Point

All AST operations must start from a single root object.

Examples:

* Validation
* Traversal
* Serialization
* Code Generation

---

## DG-002 Version Tracking

The AST must contain version information.

This enables:

* AST evolution
* Backward compatibility
* Generator compatibility checks

---

## DG-003 Metadata Container

The root object may contain metadata describing the AST.

Examples:

* Compiler version
* Target platform
* Build configuration

---

## DG-004 Generator Contract

Generators must consume the AST root rather than individual nodes.

Required:

```text id="4d0f9q"
Generator
    │
    ▼
   Ast
```

Forbidden:

```text id="5s67hk"
Generator
    │
    ▼
   Node
```

---

# 3. Root Model Overview

The AST consists of:

```text id="4lkz2r"
Ast
 │
 └── Root Node
      │
      ├── Child Node
      ├── Child Node
      └── Child Node
```

Every AST instance must contain exactly one root node.

---

# 4. Root Structure

## Proposed Design

```rust id="vgs5wc"
pub struct Ast {
    pub version: AstVersion,

    pub root: Node,
}
```

---

# 5. Why Use an AST Wrapper?

Alternative design:

```rust id="js9fba"
Node
```

as root.

This approach was rejected.

---

## Problems

### No Versioning

```rust id="8h4t0e"
Node
```

contains no AST version information.

---

### No Metadata

Future compiler information cannot be stored.

Examples:

```text id="ibqj5d"
Compiler Version

Target Platform

Feature Flags
```

---

### Weak Generator Contract

Generators would need to work directly with nodes.

This creates tighter coupling.

---

## Advantages of Ast Wrapper

```rust id="j3z94y"
Ast {
    version,
    root
}
```

Provides:

* Explicit root object
* Version tracking
* Future extensibility
* Cleaner generator APIs

---

# 6. AST Version Model

## Purpose

AST versioning enables future evolution without breaking generators.

---

## Design

```rust id="fxtw4n"
pub struct AstVersion {
    pub major: u16,

    pub minor: u16,
}
```

---

## Example

```rust id="8rrszf"
AstVersion {
    major: 1,
    minor: 0,
}
```

---

## Serialization

```json id="rwog6w"
{
  "version": {
    "major": 1,
    "minor": 0
  }
}
```

---

# 7. Root Node Requirements

The AST root node must satisfy the following rules.

---

## AR-001 Root Must Exist

Valid:

```text id="if5cqo"
Ast
 └── Node
```

Invalid:

```text id="0mbe2d"
Ast
 └── null
```

---

## AR-002 Single Root

Valid:

```text id="gn8o95"
Ast
 └── VStack
```

Invalid:

```text id="8ctpk4"
Ast
 ├── VStack
 └── HStack
```

Multiple roots are not allowed.

---

## AR-003 Platform Independent

Root nodes must not contain platform-specific concepts.

Invalid:

```text id="yn74zm"
SwiftUIView
ComposeColumn
```

Valid:

```text id="7fwm6d"
VStack
HStack
Text
```

---

# 8. AST Lifecycle

The root object participates in the following lifecycle.

```text id="8l5h9y"
Widget Tree
     │
     ▼
 AST Creation
     │
     ▼
 AST Validation
     │
     ▼
 AST Serialization
     │
     ▼
 AST Generation
```

Every stage operates through the AST root.

---

# 9. Ownership Architecture

The AST root is owned by the AST layer.

```text id="jlwmf8"
rustykrab-parser
        │
        ▼
       Ast
        │
        ▼
rustykrab-generator
```

Responsibilities:

Parser:

* Creates AST

Generator:

* Consumes AST

Validator:

* Analyzes AST

Serializer:

* Serializes AST

---

# 10. Future Expansion

The root model is intentionally minimal.

Future additions may include:

```rust id="jlwmg0"
pub struct Ast {
    pub version: AstVersion,

    pub metadata: AstMetadata,

    pub root: Node,
}
```

---

## Possible Metadata

```rust id="qhtum9"
pub struct AstMetadata {
    pub compiler_version: String,

    pub source_file: String,

    pub generated_at: String,
}
```

These fields are not required for MVP.

---

# 11. Generator Contract

All generators must accept an AST object.

Required:

```rust id="1z9g4r"
pub trait Generator {
    fn generate(
        &self,
        ast: &Ast,
    ) -> Result<String>;
}
```

---

Forbidden:

```rust id="4x6icg"
pub trait Generator {
    fn generate(
        &self,
        node: &Node,
    ) -> Result<String>;
}
```

The AST root is the official boundary between compiler stages.

---

# 12. Serialization Example

Example AST:

```json id="1o0kl0"
{
  "version": {
    "major": 1,
    "minor": 0
  },
  "root": {
    "kind": "VStack",
    "children": [
      {
        "kind": "Text",
        "properties": {
          "value": "Welcome"
        }
      }
    ]
  }
}
```

---

# 13. Architectural Decisions

| Decision               | Reason                                  |
| ---------------------- | --------------------------------------- |
| Ast wrapper used       | Supports versioning and future metadata |
| Single root required   | Simplifies traversal and generation     |
| Root node mandatory    | Prevents invalid AST                    |
| Version included       | Supports AST evolution                  |
| Generator consumes Ast | Creates clear architecture boundaries   |

---

# 14. Acceptance Criteria

This design is considered complete when:

* AST has a dedicated root object
* AST version strategy is defined
* Root node requirements are documented
* Generator contract is defined
* Ownership architecture is documented
* Serialization example is provided
* Future expansion strategy is documented

---