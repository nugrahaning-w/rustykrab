# AST Design Principles

**Project:** RustyKrab
**Module:** AST Engine
**Document Type:** Architecture Specification
**Version:** 1.0
**Status:** Draft
**Owner:** Core Framework Team

---

# 1. Purpose

Abstract Syntax Tree (AST) merupakan representasi internal yang digunakan RustyKrab untuk menjembatani Rust UI DSL dengan platform-specific code generators.

AST bertindak sebagai Intermediate Representation (IR) yang memisahkan proses:

* UI Definition
* Validation
* Code Generation

dari implementasi platform seperti SwiftUI dan Jetpack Compose.

Dengan pendekatan ini, RustyKrab dapat menghasilkan source code native untuk berbagai platform dari satu definisi UI yang sama.

---

# 2. Architectural Vision

RustyKrab dibangun berdasarkan prinsip:

> Write Once. Generate Native.

Developer mendefinisikan UI menggunakan Rust DSL.

Compiler akan mengubah DSL menjadi AST yang bersifat platform-independent.

Generator kemudian menerjemahkan AST menjadi source code native untuk platform target.

---

# 3. Architectural Context

```text
Rust DSL
    │
    ▼
Widget Tree
    │
    ▼
AST (Intermediate Representation)
    │
    ├───────────────┐
    │               │
    ▼               ▼
SwiftUI        Jetpack Compose
Generator         Generator
    │               │
    ▼               ▼
SwiftUI App    Android App
```

AST merupakan satu-satunya format yang dipahami seluruh generator.

Generator tidak memiliki pengetahuan mengenai Rust DSL.

Generator hanya memahami AST.

---

# 4. Core Responsibilities

AST bertanggung jawab untuk:

## 4.1 Represent UI Structure

AST harus mampu merepresentasikan struktur UI secara lengkap.

Contoh:

```text
VStack
├── Text
└── Button
```

---

## 4.2 Store UI Configuration

AST menyimpan seluruh konfigurasi widget.

Contoh:

```json
{
  "type": "Text",
  "properties": {
    "text": "Hello World"
  }
}
```

---

## 4.3 Support Validation

AST menjadi input utama Validation Framework.

Validator akan memeriksa:

* Hierarchy validity
* Required properties
* Structural correctness

---

## 4.4 Support Traversal

AST harus dapat ditraverse secara deterministik menggunakan Visitor Pattern.

---

## 4.5 Support Code Generation

AST menjadi input utama seluruh generator.

Contoh:

```text
AST
 ├── SwiftUI Generator
 ├── Compose Generator
 └── Future Generators
```

---

# 5. Non Responsibilities

AST tidak bertanggung jawab untuk:

## 5.1 UI Rendering

AST bukan rendering engine.

AST hanya menyimpan representasi data.

---

## 5.2 Runtime Execution

AST tidak menjalankan kode aplikasi.

Contoh yang tidak diperbolehkan:

```rust
login();
```

---

## 5.3 State Management

AST tidak menyimpan runtime state.

Contoh:

```rust
counter += 1;
```

tidak boleh menjadi bagian AST.

---

## 5.4 Platform APIs

AST tidak boleh memiliki ketergantungan terhadap:

* UIKit
* SwiftUI
* Android SDK
* Jetpack Compose
* AppKit

---

# 6. Design Principles

## 6.1 Platform Independence

AST harus sepenuhnya platform-agnostic.

AST tidak boleh mengandung:

```swift
Text("Hello")
```

atau

```kotlin
Text("Hello")
```

AST hanya menyimpan:

```json
{
  "type": "Text",
  "properties": {
    "text": "Hello"
  }
}
```

Generator bertanggung jawab menerjemahkan representasi tersebut ke platform target.

---

## 6.2 Single Source of Truth

AST harus menjadi satu-satunya representasi UI yang digunakan setelah proses parsing selesai.

Setelah Widget Tree dikonversi menjadi AST:

```text
Widget Tree
     ↓
AST
```

seluruh proses berikutnya harus bekerja menggunakan AST.

---

## 6.3 Deterministic Representation

Input yang sama harus selalu menghasilkan AST yang sama.

Contoh:

```rust
Text::new("Hello")
```

harus menghasilkan struktur AST yang identik pada setiap build.

---

## 6.4 Serializable

AST harus dapat diubah menjadi format serialisasi.

Target awal:

```text
JSON
```

Kegunaan:

* Snapshot Testing
* Debugging
* Tooling
* Build Cache
* Future Compiler Optimization

---

## 6.5 Extensibility

AST harus memungkinkan penambahan node baru tanpa perubahan besar pada arsitektur inti.

MVP:

```text
Text
Button
Image
TextField
```

Future:

```text
Grid
List
Navigation
Map
Video
TabView
```

Penambahan node baru tidak boleh mengubah struktur fundamental AST.

---

## 6.6 Generator Friendly

AST harus mudah diproses oleh generator.

Karakteristik:

* Tree-based
* Recursive
* Predictable
* Stable

Generator tidak boleh membutuhkan knowledge terhadap DSL.

---

## 6.7 Validation First

AST wajib divalidasi sebelum code generation.

Pipeline:

```text
DSL
 ↓
AST
 ↓
Validation
 ↓
Generator
```

Generator tidak bertanggung jawab memperbaiki AST yang invalid.

---

## 6.8 Immutability By Design

AST dianggap immutable setelah dibuat.

Perubahan AST dilakukan melalui transformasi dan menghasilkan tree baru.

Hal ini mempermudah:

* Debugging
* Snapshot Testing
* Compiler Optimization
* Future Parallel Processing

---

# 7. AST Lifecycle

## Phase 1 — DSL Authoring

Developer menulis Rust DSL.

```rust
VStack::new()
    .child(Text::new("Hello"))
    .child(Button::new("Login"))
```

---

## Phase 2 — Widget Tree Construction

Compiler membangun Widget Tree.

```text
VStack
├── Text
└── Button
```

---

## Phase 3 — AST Construction

Widget Tree dikonversi menjadi AST.

```text
AstTree
└── VStack
    ├── Text
    └── Button
```

---

## Phase 4 — Validation

Validation Framework memverifikasi:

* Node hierarchy
* Required properties
* Structural rules

---

## Phase 5 — Traversal

Visitor melakukan traversal AST.

```text
App
VStack
Text
Button
```

---

## Phase 6 — Code Generation

Generator menghasilkan source code native.

SwiftUI:

```swift
VStack {
    Text("Hello")
    Button("Login") {}
}
```

Compose:

```kotlin
Column {
    Text("Hello")
    Button(
        onClick = {}
    ) {
        Text("Login")
    }
}
```

---

# 8. Quality Attributes

## Maintainability

AST harus mudah dipahami dan dipelihara.

---

## Scalability

Target awal:

* 100+ Screens
* 500+ Widgets
* 50+ Routes

---

## Testability

AST harus mendukung:

* Unit Test
* Integration Test
* Snapshot Test

---

## Predictability

Traversal dan generation harus menghasilkan output yang konsisten.

---

## Stability

Generator dapat bergantung pada kontrak AST yang stabil.

---

# 9. Future Evolution

AST dirancang untuk mendukung ekspansi berikut:

## Navigation System

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

## Desktop Platforms

```text
SwiftUI macOS
Compose Desktop
```

---

## Web Platforms

```text
HTML Generator
```

---

## Additional Native Targets

```text
React Native
Flutter
```

---

# 10. Success Criteria

AST Architecture dianggap berhasil apabila:

* Mampu merepresentasikan seluruh widget MVP.
* Tidak mengandung platform-specific implementation.
* Mendukung serialization.
* Mendukung validation framework.
* Mendukung visitor traversal.
* Mendukung code generation.
* Mendukung extensibility tanpa breaking changes.
* Dapat digunakan oleh SwiftUI Generator.
* Dapat digunakan oleh Compose Generator.

---

# 11. Guiding Principle

AST bukan representasi SwiftUI.

AST bukan representasi Jetpack Compose.

AST bukan rendering engine.

AST adalah bahasa universal yang menjadi kontrak antara Rust DSL dan seluruh generator.

> Write Once. Generate Native.
