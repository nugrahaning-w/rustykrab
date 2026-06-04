# AST Node Model Specification

Version: 1.0
Project: RustyKrab
Module: AST Engine
Status: Draft
Author: Aji Nugrahaning Widhi
Last Updated: June 2026

---

# 1. Overview

Dokumen ini mendefinisikan struktur data utama yang digunakan oleh RustyKrab AST Engine.

AST Node Model merupakan representasi internal dari UI yang telah dikonversi dari Rust DSL dan akan digunakan sebagai input untuk seluruh code generator.

Dokumen ini mendefinisikan:

* AstTree
* AstNode
* Node Identity
* Parent-Child Relationship
* Tree Structure
* Traversal Rules
* Validation Rules
* Serialization Model

Dokumen ini menjadi referensi utama untuk implementasi crate:

```text
crates/rustykrab-ast
```

---

# 2. Design Goals

AST Node Model harus memenuhi karakteristik berikut:

## Platform Independent

Tidak mengandung SwiftUI maupun Compose specific implementation.

## Serializable

Dapat dikonversi menjadi JSON.

## Traversable

Mudah ditraverse menggunakan Visitor Pattern.

## Immutable Friendly

Mendukung transformasi AST tanpa mutasi langsung.

## Extensible

Node baru dapat ditambahkan tanpa mengubah struktur inti.

---

# 3. High-Level Structure

AST direpresentasikan sebagai tree.

```text
AstTree
│
└── AstNode
     │
     ├── id
     ├── node_type
     ├── properties
     └── children
```

Seluruh UI direpresentasikan oleh satu root node yang berada di dalam AstTree.

---

# 4. AstTree

## Purpose

AstTree merupakan root container dari seluruh Abstract Syntax Tree.

AstTree menjadi entry point untuk:

* Validation
* Traversal
* Serialization
* Code Generation

---

## Structure

```rust
pub struct AstTree {
    pub root: AstNode,
}
```

---

## Responsibilities

### Store Root Node

Menyimpan node paling atas dalam tree.

### Provide Traversal Entry Point

Generator dan validator memulai traversal dari root.

### Provide Serialization Entry Point

AST dapat diubah menjadi JSON melalui AstTree.

### Provide Validation Entry Point

Semua validator bekerja pada level AstTree.

---

## Example

```text
AstTree
│
└── App
```

---

# 5. AstNode

## Purpose

AstNode merupakan unit terkecil dalam AST.

Semua elemen UI direpresentasikan sebagai AstNode.

Contoh:

```text
Text
Button
Image
VStack
Screen
App
```

Semuanya memiliki struktur yang sama.

---

## Structure

```rust
pub struct AstNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub properties: Properties,
    pub children: Vec<AstNode>,
}
```

---

# 6. AstNode Fields

## 6.1 NodeId

### Purpose

Memberikan identitas unik untuk setiap node.

---

### Definition

```rust
pub type NodeId = String;
```

Implementasi berikut juga diperbolehkan:

```rust
pub struct NodeId(Uuid);
```

---

### Requirements

* Wajib unik dalam satu tree
* Tidak boleh kosong
* Tidak boleh berubah setelah dibuat

---

### Example

```text
node-1
node-2
node-3
```

---

## 6.2 NodeType

### Purpose

Menentukan jenis node.

---

### Definition

```rust
pub enum NodeType
```

---

### Example

```text
App
Screen
VStack
Text
Button
```

---

### Requirements

* Harus valid
* Harus berasal dari NodeType enum

---

## 6.3 Properties

### Purpose

Menyimpan konfigurasi node.

---

### Definition

```rust
pub type Properties =
HashMap<String, PropertyValue>;
```

---

### Example

Text Widget

```json
{
  "text": "Hello World"
}
```

Button Widget

```json
{
  "title": "Login",
  "action": "login"
}
```

VStack

```json
{
  "spacing": 16
}
```

---

### Requirements

* Dapat kosong
* Harus serializable
* Harus deterministic

---

## 6.4 Children

### Purpose

Menyimpan child node.

---

### Definition

```rust
Vec<AstNode>
```

---

### Example

```text
VStack
├── Text
└── Button
```

Representasi:

```json
{
  "type": "vstack",
  "children": [
    {
      "type": "text"
    },
    {
      "type": "button"
    }
  ]
}
```

---

### Requirements

* Ordered
* Recursive
* Tidak boleh cyclic

---

# 7. Parent Child Relationship

AST menggunakan struktur tree.

```text
App
│
└── Screen
     │
     └── VStack
          │
          ├── Text
          └── Button
```

---

## Rule 1

Node hanya boleh memiliki satu parent.

Valid:

```text
A
└── B
```

Invalid:

```text
A ──┐
    │
    ▼
    B
    ▲
    │
C ──┘
```

---

## Rule 2

Node dapat memiliki banyak child.

Valid:

```text
VStack
├── Text
├── Button
└── Image
```

---

## Rule 3

Node tidak boleh menjadi parent dirinya sendiri.

Invalid:

```text
A
└── A
```

---

## Rule 4

Circular reference tidak diperbolehkan.

Invalid:

```text
A
└── B
     └── A
```

---

# 8. MVP Node Hierarchy

## Root Nodes

```text
App
Screen
```

---

## Layout Nodes

```text
VStack
HStack
ScrollView
Spacer
```

---

## Widget Nodes

```text
Text
Button
Image
TextField
```

---

# 9. Example Tree Structures

## Simple Layout

```text
VStack
├── Text
└── Button
```

---

## Nested Layout

```text
VStack
├── Text
├── HStack
│    ├── Button
│    └── Button
└── Image
```

---

## Full Screen Example

```text
App
└── Screen
     └── VStack
          ├── Text
          ├── TextField
          └── Button
```

---

# 10. JSON Representation

## Example AST

```json
{
  "id": "node-1",
  "node_type": "VStack",
  "properties": {
    "spacing": 16
  },
  "children": [
    {
      "id": "node-2",
      "node_type": "Text",
      "properties": {
        "text": "Hello"
      },
      "children": []
    },
    {
      "id": "node-3",
      "node_type": "Button",
      "properties": {
        "title": "Login"
      },
      "children": []
    }
  ]
}
```

---

# 11. Traversal Model

Traversal default menggunakan Depth First Search (DFS).

---

## Example Tree

```text
App
│
└── VStack
     │
     ├── Text
     └── Button
```

---

## Traversal Order

```text
App
VStack
Text
Button
```

---

## Why DFS?

DFS dipilih karena:

* Natural untuk UI tree
* Mudah digunakan generator
* Memory footprint kecil
* Deterministic

---

# 12. Visitor Pattern Support

Generator akan menggunakan Visitor Pattern.

---

## Visitor Interface

```rust
pub trait AstVisitor {
    fn visit_node(
        &mut self,
        node: &AstNode,
    );
}
```

---

## Traversal Example

```rust
tree.walk(visitor);
```

---

## Example Generator

```text
SwiftUIVisitor
ComposeVisitor
```

---

# 13. Validation Rules

## Tree Rules

### Root Required

AstTree wajib memiliki root.

---

### Unique NodeId

Seluruh node wajib memiliki ID unik.

---

### No Circular Reference

Tree harus acyclic.

---

## Widget Rules

### Text

Required Property:

```json
{
  "text": "Hello"
}
```

---

### Button

Required Property:

```json
{
  "title": "Login"
}
```

---

## Layout Rules

### Spacer

Tidak boleh memiliki child.

Valid:

```text
Spacer
```

Invalid:

```text
Spacer
└── Text
```

---

# 14. Serialization Requirements

AST wajib mendukung:

## Serialize

```rust
Serialize
```

AST → JSON

---

## Deserialize

```rust
Deserialize
```

JSON → AST

---

## Deterministic Output

Output JSON harus konsisten.

Input yang sama harus menghasilkan output yang sama.

---

# 15. Memory and Performance Considerations

Target MVP:

* 100+ screens
* 500+ widgets
* 50+ navigation routes

AST traversal harus tetap:

```text
O(n)
```

dengan:

```text
n = total node count
```

---

# 16. Future Extensions

Node Model harus mendukung ekspansi tanpa breaking changes.

## Future Layouts

```text
Grid
LazyVStack
LazyHStack
```

---

## Future Widgets

```text
List
Toggle
Slider
Picker
```

---

## Navigation

```text
NavigationNode
RouteNode
```

---

## State Management

```text
StateNode
BindingNode
```

---

# 17. Success Criteria

AST Node Model dianggap berhasil apabila:

* Seluruh widget MVP dapat direpresentasikan.
* Mendukung nested hierarchy.
* Mendukung DFS traversal.
* Mendukung Visitor Pattern.
* Mendukung JSON serialization.
* Mendukung validation framework.
* Tidak mengandung platform-specific implementation.
* Dapat digunakan oleh SwiftUI Generator.
* Dapat digunakan oleh Compose Generator.

---

# Appendix A – Class Diagram

```text
+-------------------+
|      AstTree      |
+-------------------+
| root: AstNode     |
+-------------------+

          │
          ▼

+-------------------------+
|        AstNode          |
+-------------------------+
| id: NodeId             |
| node_type: NodeType    |
| properties: Properties |
| children: Vec<Node>    |
+-------------------------+

          │
          ▼

+-------------------+
|     NodeType      |
+-------------------+
| App               |
| Screen            |
| VStack            |
| HStack            |
| ScrollView        |
| Spacer            |
| Text              |
| Button            |
| Image             |
| TextField         |
+-------------------+
```
