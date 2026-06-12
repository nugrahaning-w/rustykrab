# RustyKrab Property System Usage Guide

## Overview

The Property System is the primary mechanism used by RustyKrab AST nodes to store widget configuration.

Every widget-specific configuration is represented as a property.

Examples:

* Text content
* Button title
* Image source
* Layout configuration
* State bindings
* Navigation destinations

---

## Architecture

```text
Node
│
├── NodeKind
├── PropertyMap
│   ├── key
│   └── PropertyValue
│
└── Children
```

Example:

```json
{
  "kind": "Text",
  "properties": {
    "value": "Hello World"
  }
}
```

---

## PropertyValue

PropertyValue is the universal value container used throughout RustyKrab.

Supported types:

| Type       | Example       |
| ---------- | ------------- |
| String     | "Hello"       |
| Integer    | 16            |
| Float      | 0.5           |
| Boolean    | true          |
| Enum       | "center"      |
| Color      | "#FF0000"     |
| Expression | "counter + 1" |
| Reference  | "user.name"   |
| Function   | "login"       |
| Array      | [...]         |
| Object     | {...}         |
| Null       | null          |

Example:

```rust
PropertyValue::String(
    "Hello".into()
)
```

---

## PropertyMap

PropertyMap stores all properties associated with a node.

Example:

```rust
let mut properties =
    PropertyMap::new();

properties.insert(
    PropertyKeys::VALUE.into(),
    PropertyValue::String(
        "Hello".into(),
    ),
);
```

---

## PropertyObject

PropertyObject is used for nested structures.

Example:

```json
{
  "shadow": {
    "radius": 8,
    "opacity": 0.3
  }
}
```

Equivalent:

```rust
let mut shadow =
    PropertyObject::new();

shadow.insert(
    "radius".into(),
    PropertyValue::Integer(8),
);

shadow.insert(
    "opacity".into(),
    PropertyValue::Float(0.3),
);
```

---

## PropertyAccess

PropertyAccess provides a common interface for manipulating properties.

Example:

```rust
node.set(
    PropertyKeys::VALUE,
    PropertyValue::String(
        "Hello".into(),
    ),
);

let value =
    node.get(
        PropertyKeys::VALUE,
    );
```

---

## Property Builder API

Builder APIs allow fluent property creation.

Example:

```rust
let node = Node::new(
    NodeId::new("title"),
    NodeKind::Text,
)
.property(
    PropertyKeys::VALUE,
    PropertyValue::String(
        "Welcome".into(),
    ),
);
```

---

## Standard Property Keys

Always prefer PropertyKeys over raw strings.

Good:

```rust
PropertyKeys::VALUE
PropertyKeys::FONT_SIZE
PropertyKeys::SOURCE
```

Avoid:

```rust
"value"
"fontSize"
"fontsize"
```

Using PropertyKeys prevents typos and improves consistency.

---

## Validation Helpers

Validation helpers are available for checking required properties.

Example:

```rust
require_string_property(
    &node.properties,
    PropertyKeys::VALUE,
)?;
```

Example error:

```text
Required property 'value' is missing
```

---

## Complex Properties

### Nested Object

```json
{
  "style": {
    "shadow": {
      "radius": 8
    }
  }
}
```

### Array

```json
{
  "items": [
    "A",
    "B",
    "C"
  ]
}
```

### Array of Objects

```json
{
  "items": [
    {
      "title": "A"
    },
    {
      "title": "B"
    }
  ]
}
```

---

## Future Usage

The Property System will be used by:

* Modifier System
* Event System
* Binding System
* Validation Framework
* SwiftUI Generator
* Compose Generator

No changes to the Property API should be required when these systems are added.
