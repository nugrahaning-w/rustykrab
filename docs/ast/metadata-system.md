# Metadata System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the Metadata System used by the RustyKrab AST.

The Metadata System provides contextual information about AST nodes without affecting business logic, layout behavior, rendering, or generation output.

Metadata exists to support:

* Compiler diagnostics
* Error reporting
* Source mapping
* Debugging
* Tooling
* IDE integration
* Static analysis
* Future developer experience improvements

Metadata must never affect generated UI behavior.

---

# 2. Design Goals

## DG-001 Diagnostics Support

Metadata must provide sufficient information for meaningful compiler errors.

Example:

```text id="vh5x21"
Error:
Text cannot contain children

File:
main.rs

Line:
15

Column:
8
```

---

## DG-002 Source Mapping

Every AST node should be traceable back to its original source location.

---

## DG-003 Debugging Support

Metadata should help developers inspect AST structures.

---

## DG-004 Tooling Support

Metadata should enable:

* IDE plugins
* AST explorers
* Visualization tools
* Compiler analyzers

---

## DG-005 Non-Functional

Metadata must never change runtime behavior.

Invalid:

```text id="w9f7a2"
Metadata controls rendering
```

Valid:

```text id="u1d8v6"
Metadata only describes context
```

---

# 3. Architecture Overview

```text id="s4j2m9"
Node
 │
 ├── NodeKind
 ├── Properties
 ├── Modifiers
 ├── Events
 ├── Bindings
 ├── Metadata
 └── Children
```

Every node may contain metadata.

---

# 4. Metadata Model

## Metadata

```rust id="p8m4k2"
pub struct Metadata {
    pub source: SourceLocation,

    pub compiler: CompilerMetadata,

    pub diagnostics: Vec<DiagnosticTag>,
}
```

The metadata structure is intentionally extensible.

---

# 5. SourceLocation

SourceLocation maps AST nodes back to DSL source code.

---

## Definition

```rust id="t5v8n1"
pub struct SourceLocation {
    pub file: String,

    pub line: usize,

    pub column: usize,
}
```

---

## Example

```rust id="n7x2f4"
Text::new("Hello")
```

Source:

```text id="r9w3d6"
main.rs

Line: 12

Column: 5
```

Metadata:

```json id="k1j8m5"
{
  "source": {
    "file": "main.rs",
    "line": 12,
    "column": 5
  }
}
```

---

# 6. CompilerMetadata

Stores compiler-related information.

---

## Definition

```rust id="m4k7v2"
pub struct CompilerMetadata {
    pub compiler_version: String,

    pub ast_version: String,
}
```

---

## Example

```json id="h2q5c8"
{
  "compiler_version": "0.1.0",
  "ast_version": "1.0"
}
```

---

# 7. DiagnosticTag

Diagnostic tags provide analysis information.

---

## Definition

```rust id="f6x9n3"
pub enum DiagnosticTag {
    Warning(String),

    Error(String),

    Hint(String),

    Deprecated(String),
}
```

---

## Example

```json id="d3m7v1"
{
  "diagnostics": [
    {
      "type": "Warning",
      "message": "Unused modifier"
    }
  ]
}
```

---

# 8. Metadata Lifecycle

Metadata is created during parsing.

```text id="z8c2m5"
Widget DSL
      │
      ▼
Parser
      │
      ▼
AST Node
      │
      ▼
Metadata Attached
```

The parser is responsible for generating source information.

---

# 9. Error Reporting Flow

Example:

Invalid DSL:

```rust id="y5p8k2"
Text::new("Hello")
    .child(
        Button::new("Login")
    )
```

Validation:

```text id="j7n4w9"
Text cannot contain children
```

Compiler Output:

```text id="v1k6c3"
Error: Text cannot contain children

File: main.rs
Line: 10
Column: 5
```

Metadata provides the source location.

---

# 10. AST Debugging

Metadata allows AST inspection tools.

Example:

```json id="q9f3x7"
{
  "kind": "Text",

  "metadata": {
    "source": {
      "file": "main.rs",
      "line": 10,
      "column": 5
    }
  }
}
```

Useful for:

* AST Viewer
* Compiler Debugger
* Developer Tools

---

# 11. IDE Integration

Future IDE plugins can use metadata for:

```text id="e2m8k1"
Jump to Source

Error Highlighting

Hover Information

AST Visualization
```

Metadata is the foundation for these capabilities.

---

# 12. Validation Support

Validators should use metadata when generating errors.

Example:

```text id="r5q7n2"
Image source missing
```

Output:

```text id="u3k1d9"
Image source missing

File: home.rs
Line: 25
Column: 8
```

---

# 13. Serialization Example

```json id="p4v8m6"
{
  "kind": "Text",
  "metadata": {
    "source": {
      "file": "main.rs",
      "line": 12,
      "column": 5
    },
    "compiler": {
      "compiler_version": "0.1.0",
      "ast_version": "1.0"
    }
  }
}
```

---

# 14. Metadata Ownership

Metadata is owned by the AST.

```text id="n8c4w2"
Node
 └── Metadata
```

Generators may read metadata.

Generators must not modify metadata.

---

# 15. Generator Usage

Generators may use metadata for:

```text id="z4x7m1"
Debug Comments

Diagnostics

Source Mapping
```

Example:

SwiftUI:

```swift id="a7m2v5"
// Generated from:
// main.rs:12
Text("Hello")
```

This behavior is optional.

---

# 16. Future Expansion

Future metadata fields may include:

```rust id="x9k4d8"
build_id

module_name

project_name

author

created_at

updated_at
```

without requiring AST redesign.

---

## Future Tooling Metadata

```rust id="g2n6p1"
editor_id

workspace_id

plugin_metadata
```

Reserved for future tooling support.

---

# 17. Why Use Metadata?

Rejected Design:

```rust id="f5x8k3"
Node {
    kind,
    properties
}
```

Problems:

* Poor diagnostics
* No source mapping
* Weak tooling support
* Difficult debugging

---

Selected Design:

```rust id="m7q1v4"
Node {
    kind,
    properties,
    metadata
}
```

Benefits:

* Better developer experience
* Better diagnostics
* Better tooling
* Better maintainability

---

# 18. Architectural Decisions

| Decision                           | Reason                 |
| ---------------------------------- | ---------------------- |
| Metadata stored per node           | Precise source mapping |
| SourceLocation included            | Better error reporting |
| Compiler metadata included         | Version tracking       |
| Diagnostic tags included           | Analysis support       |
| Metadata read-only for generators  | Stability              |
| Metadata separated from properties | Separation of concerns |

---

# 19. Validation Rules

## MD-001

SourceLocation must contain a valid file path.

---

## MD-002

Line numbers must be positive.

Valid:

```text id="b5k9d2"
1
2
3
```

Invalid:

```text id="y1q4v8"
0
-1
```

---

## MD-003

Column numbers must be positive.

---

## MD-004

Compiler version must not be empty.

---

# 20. Acceptance Criteria

This design is considered complete when:

* Metadata structure is defined
* SourceLocation is defined
* CompilerMetadata is defined
* DiagnosticTag is defined
* Error reporting flow is documented
* IDE integration strategy is documented
* Validation rules are documented
* Serialization examples are provided

---

# 21. Future Integration

The Metadata System will support:

```text id="w8m2c6"
Compiler Diagnostics

AST Explorer

IDE Plugins

Language Server

Debugging Tools

Code Analysis
```

without requiring modifications to:

```text id="j4n7p3"
Node

Property System

Modifier System

Binding System
```

---

# 22. Next Document

After approval of this document:

```text id="k6v3m8"
Architecture Review
```

will validate the complete AST Foundation architecture:

* AST Root
* Node Model
* NodeKind System
* Property System
* Modifier System
* Event System
* Binding System
* Metadata System

before implementation begins in Story 2.2.
