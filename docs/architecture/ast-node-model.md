# AST Node Model Specification

**Project:** RustyKrab
**Module:** AST Engine
**Document Type:** Architecture Specification
**Version:** 1.0
**Status:** Draft
**Owner:** Core Framework Team

---

# 1. Purpose

Dokumen ini mendefinisikan model data inti yang digunakan untuk merepresentasikan Abstract Syntax Tree (AST) pada RustyKrab.

AST Node Model merupakan kontrak utama yang digunakan oleh:

* AST Builder
* Validation Framework
* Visitor Framework
* SwiftUI Generator
* Compose Generator
* Future Generators

Dokumen ini menjelaskan struktur internal AST serta hubungan antar node yang digunakan untuk merepresentasikan UI secara platform-independent.

---

# 2. Design Goals

AST Node Model harus memenuhi tujuan berikut:

## Platform Independent

Model tidak boleh mengandung implementasi spesifik platform.

Contoh yang tidak diperbolehkan:

```swift
Text("Hello")
```

```kotlin
Text("Hello")
```

---

## Serializable

Model harus dapat diubah ke JSON dan dikembalikan kembali tanpa kehilangan informasi.

---

## Traversable

Model harus mudah ditraverse menggunakan Visitor Pattern.

---

## Extensible

Node baru dapat ditambahkan tanpa mengubah struktur fundamental AST.

---

## Deterministic

Input yang sama harus menghasilkan struktur AST yang sama.

---

# 3. High Level Architecture

AST direpresentasikan sebagai tree hierarchy.

```text
AstTree
│
└── AstNode
     │
     ├── NodeType
     ├── Properties
     └── Children
```

Setiap elemen UI direpresentasikan oleh satu AstNode.

---

# 4. AST Structure Overview

Contoh representasi UI:

```text
VStack
├── Text("Hello")
└── Button("Login")
```

AST:

```text
AstTree
│
└── VStack
     │
     ├── Text
     └── Button
```

JSON:

```json
{
  "root": {
    "node_type": "VStack",
    "properties": {},
    "children": [
      {
        "node_type": "Text",
        "properties": {
          "text": "Hello"
        }
      },
      {
        "node_type": "Button",
        "properties": {
          "title": "Login"
        }
      }
    ]
  }
}
```

---

# 5. Core Components

AST terdiri dari dua komponen utama:

```text
AstTree
AstNode
```

---

# 6. AstTree

## Purpose

AstTree merupakan root container yang menyimpan seluruh struktur AST.

Semua proses compiler dimulai dari AstTree.

---

## Responsibilities

### Root Ownership

Memiliki root node AST.

---

### Traversal Entry Point

Menjadi titik awal traversal.

---

### Validation Entry Point

Menjadi titik awal validasi.

---

### Generator Entry Point

Menjadi input seluruh generator.

---

## Rust Model

```rust
pub struct AstTree {
    pub root: AstNode,
}
```

---

## Example

```text
AstTree
│
└── App
```

---

## Constraints

### Single Root

AST hanya boleh memiliki satu root.

Valid:

```text
App
```

Invalid:

```text
App
App
```

---

### Root Required

AstTree tidak boleh kosong.

---

# 7. AstNode

## Purpose

AstNode merupakan representasi universal dari setiap elemen UI.

Seluruh widget, layout, dan container direpresentasikan menggunakan struktur yang sama.

---

## Design Philosophy

Daripada membuat:

```rust
TextNode
ButtonNode
ImageNode
```

RustyKrab menggunakan:

```rust
AstNode
```

yang dikombinasikan dengan:

```rust
NodeType
```

Pendekatan ini lebih scalable dan lebih cocok untuk compiler architecture.

---

## Rust Model

```rust
pub struct AstNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub properties: Properties,
    pub children: Vec<AstNode>,
}
```

---

# 8. Node Identity

## Purpose

Setiap node harus memiliki identitas unik.

---

## Rust Model

```rust
pub type NodeId = String;
```

---

## Example

```text
node-1
node-2
node-3
```

---

## Constraints

* Harus unik dalam satu tree
* Tidak boleh kosong
* Tidak boleh berubah setelah dibuat

---

# 9. Node Type

## Purpose

Menentukan jenis node.

---

## Examples

```text
App
Screen

VStack
HStack

Text
Button
Image
TextField
```

---

## Ownership

Definisi lengkap berada pada:

```text
node-types.md
```

---

# 10. Properties

## Purpose

Menyimpan konfigurasi node.

---

## Examples

Text:

```json
{
  "text": "Hello"
}
```

Button:

```json
{
  "title": "Login"
}
```

VStack:

```json
{
  "spacing": 16
}
```

---

## Ownership

Definisi lengkap berada pada:

```text
property-system.md
```

---

# 11. Children

## Purpose

Menyimpan child nodes.

---

## Rust Model

```rust
Vec<AstNode>
```

---

## Example

```text
VStack
├── Text
└── Button
```

Representasi:

```rust
children: vec![
    text_node,
    button_node,
]
```

---

# 12. Parent Child Relationship

AST menggunakan tree hierarchy.

---

## Rule 1

Node hanya boleh memiliki satu parent.

Valid:

```text
App
└── Screen
```

Invalid:

```text
App ──┐
      ▼
    Text
      ▲
Screen─┘
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
Text
└── Text
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

# 13. Node Categories

AST mengenal tiga kategori node.

```text
NodeType
│
├── Root Nodes
├── Layout Nodes
└── Widget Nodes
```

---

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

# 14. Example Hierarchies

## Example 1

```text
App
└── Screen
     └── VStack
          ├── Text
          └── Button
```

Valid.

---

## Example 2

```text
App
└── Screen
     └── ScrollView
          └── VStack
               ├── Image
               ├── Text
               └── Button
```

Valid.

---

## Example 3

```text
App
└── Screen
     └── HStack
          ├── Text
          ├── Spacer
          └── Button
```

Valid.

---

# 15. Traversal Model

AST traversal menggunakan Depth First Search (DFS).

---

## Example

```text
App
│
└── VStack
     │
     ├── Text
     └── Button
```

Traversal order:

```text
App
VStack
Text
Button
```

---

## Reasoning

DFS dipilih karena:

* Natural untuk tree UI
* Mudah digunakan generator
* Memory footprint kecil
* Predictable

---

# 16. Visitor Pattern Integration

Traversal akan menggunakan Visitor Pattern.

---

## Visitor Contract

```rust
pub trait AstVisitor {
    fn visit_node(
        &mut self,
        node: &AstNode,
    );
}
```

---

## Example Visitors

```text
SwiftUIVisitor
ComposeVisitor
ValidationVisitor
```

---

# 17. Validation Rules

Validator wajib memeriksa:

---

## Tree Rules

### Root Required

AstTree wajib memiliki root.

---

### Unique NodeId

Semua NodeId harus unik.

---

### No Cycles

AST harus acyclic.

---

## Structural Rules

### Widget Nodes

Widget tidak boleh memiliki child.

Contoh:

```text
Text
└── Button
```

Invalid.

---

### Spacer

Spacer tidak boleh memiliki child.

---

### App

App harus menjadi root node.

---

# 18. Serialization Requirements

AST harus mendukung:

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

## Roundtrip Safety

Berikut harus selalu benar:

```text
AST
 ↓
JSON
 ↓
AST
```

tanpa kehilangan informasi.

---

# 19. Performance Considerations

Target MVP:

```text
100+ screens
500+ widgets
50+ routes
```

Traversal harus memiliki kompleksitas:

```text
O(n)
```

dengan:

```text
n = total node count
```

---

# 20. Future Extensions

AST Node Model harus mampu mendukung:

---

## Additional Layouts

```text
Grid
LazyVStack
LazyHStack
ZStack
```

---

## Additional Widgets

```text
Toggle
Slider
Picker
List
Map
```

---

## Navigation

```text
NavigationNode
RouteNode
TabNode
```

---

## State Management

```text
StateNode
BindingNode
```

---

# 21. Success Criteria

AST Node Model dianggap berhasil apabila:

* Mampu merepresentasikan seluruh widget MVP.
* Mendukung nested hierarchy.
* Mendukung DFS traversal.
* Mendukung Visitor Pattern.
* Mendukung serialization.
* Mendukung validation.
* Tidak mengandung platform-specific implementation.
* Dapat digunakan oleh SwiftUI Generator.
* Dapat digunakan oleh Compose Generator.

---

# 22. Guiding Principle

Setiap elemen UI di RustyKrab harus direpresentasikan menggunakan satu model universal:

```text
AstNode
```

dengan perilaku yang ditentukan oleh:

```text
NodeType
Properties
Children
```

Pendekatan ini memungkinkan AST tetap sederhana, scalable, generator-friendly, dan sesuai dengan praktik yang digunakan oleh modern compiler dan UI framework architecture.
