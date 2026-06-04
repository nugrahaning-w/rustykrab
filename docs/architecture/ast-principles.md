# AST Design Principles

Version: 1.0
Project: RustyKrab
Module: AST Engine
Status: Draft
Author: Aji Nugrahaning Widhi
Last Updated: June 2026

---

# 1. Overview

Abstract Syntax Tree (AST) adalah representasi internal platform-independent yang digunakan RustyKrab untuk menjembatani Rust UI DSL dengan generator platform native.

AST merupakan fondasi utama compiler RustyKrab dan berfungsi sebagai kontrak antara DSL layer dan code generation layer.

Semua UI yang ditulis menggunakan Rust DSL akan dikonversi menjadi AST sebelum menghasilkan source code SwiftUI maupun Jetpack Compose.

AST memungkinkan RustyKrab mewujudkan prinsip utama:

> Write Once. Generate Native.

---

# 2. Problem Statement

Pengembangan aplikasi mobile native saat ini mengharuskan developer menulis UI dua kali:

* SwiftUI untuk iOS
* Jetpack Compose untuk Android

Walaupun business logic dapat dibagikan, UI tetap harus diimplementasikan secara terpisah.

RustyKrab menyelesaikan masalah ini dengan pendekatan:

```text
Rust DSL
    ↓
AST
    ↓
SwiftUI
```

dan

```text
Rust DSL
    ↓
AST
    ↓
Jetpack Compose
```

Dengan AST sebagai intermediate representation, satu definisi UI dapat menghasilkan source code native untuk berbagai platform.

---

# 3. AST Architectural Role

AST berada di tengah compiler pipeline.

```text
┌──────────────────┐
│    Rust DSL      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   Widget Tree    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│       AST        │
│ Platform Neutral │
└──────┬─────┬─────┘
       │     │
       │     │
       ▼     ▼
┌──────────┐ ┌──────────┐
│ SwiftUI  │ │ Compose  │
│Generator │ │Generator │
└────┬─────┘ └────┬─────┘
     │            │
     ▼            ▼
┌──────────┐ ┌──────────┐
│ iOS App  │ │AndroidApp│
└──────────┘ └──────────┘
```

AST menjadi satu-satunya representasi UI yang dipahami seluruh generator.

---

# 4. Responsibilities

AST bertanggung jawab untuk:

## 4.1 Represent UI Structure

AST harus mampu merepresentasikan struktur UI.

Contoh:

```text
VStack
├── Text
└── Button
```

---

## 4.2 Store Widget Configuration

AST menyimpan seluruh konfigurasi widget.

Contoh:

```json
{
  "type": "text",
  "properties": {
    "text": "Hello World"
  }
}
```

---

## 4.3 Provide Generator Input

SwiftUI Generator dan Compose Generator menerima AST sebagai input utama.

---

## 4.4 Support Validation

AST menjadi objek yang divalidasi sebelum proses code generation.

---

## 4.5 Support Traversal

AST harus dapat ditraverse menggunakan Visitor Pattern.

---

# 5. Non Responsibilities

AST tidak bertanggung jawab untuk:

## 5.1 Rendering UI

AST bukan rendering engine.

AST hanya menyimpan representasi data.

---

## 5.2 Runtime State

AST tidak menyimpan state runtime.

Contoh yang tidak diperbolehkan:

```rust
counter += 1;
```

State Management akan ditangani oleh modul lain.

---

## 5.3 Business Logic

AST tidak menyimpan implementasi business logic.

Contoh:

```rust
login_user();
```

Tidak boleh berada di dalam AST.

---

## 5.4 Platform API Access

AST tidak berinteraksi langsung dengan:

* Camera
* Bluetooth
* Push Notification
* Device Storage

---

# 6. Design Goals

## 6.1 Platform Independent

AST tidak boleh mengandung konsep spesifik platform.

Contoh yang dilarang:

```swift
NavigationStack
```

```kotlin
NavHost
```

AST hanya menyimpan:

```json
{
  "type": "navigation"
}
```

Generator yang menentukan implementasi platform.

---

## 6.2 Serializable

AST harus dapat diserialisasi ke JSON.

Tujuan:

* Snapshot Testing
* Debugging
* Build Cache
* Tooling Support

Contoh:

```json
{
  "type": "text",
  "properties": {
    "text": "Hello"
  }
}
```

---

## 6.3 Generator Friendly

AST harus mudah diproses oleh generator.

Karakteristik:

* Deterministic
* Predictable
* Tree-based
* Traversable

Target:

```text
Traversal Complexity = O(n)
```

---

## 6.4 Extensible

AST harus mendukung penambahan node baru tanpa perubahan besar.

MVP:

```text
Text
Button
Image
TextField
```

Future:

```text
List
Grid
LazyStack
TabView
Map
```

---

## 6.5 Testable

AST harus mudah diuji.

Jenis testing:

* Unit Test
* Integration Test
* Snapshot Test

---

## 6.6 Maintainable

Perubahan generator tidak boleh memerlukan perubahan AST yang signifikan.

AST harus stabil sebagai kontrak antar modul.

---

# 7. Design Constraints

## 7.1 No Platform Specific Data

AST tidak boleh menyimpan:

```swift
Text("Hello")
```

atau

```kotlin
Text("Hello")
```

AST hanya menyimpan bentuk netral:

```json
{
  "type": "text",
  "text": "Hello"
}
```

---

## 7.2 No Runtime Execution

AST tidak boleh menjalankan kode.

Contoh yang dilarang:

```rust
fn on_click() {
    login();
}
```

Yang boleh:

```json
{
  "action": "login"
}
```

---

## 7.3 No Circular Reference

AST harus selalu berupa tree.

Valid:

```text
A
└── B
```

Invalid:

```text
A
└── B
     └── A
```

---

## 7.4 Immutable By Default

Setelah AST dihasilkan, node tidak boleh dimodifikasi secara langsung.

Transformasi dilakukan dengan membuat AST baru.

---

# 8. AST Lifecycle

## Step 1 – Developer Writes DSL

Developer menulis UI menggunakan Rust DSL.

```rust
VStack::new()
    .child(Text::new("Hello"))
    .child(Button::new("Login"))
```

---

## Step 2 – Widget Tree Creation

Compiler membangun widget tree.

```text
VStack
├── Text
└── Button
```

---

## Step 3 – AST Generation

Widget tree dikonversi menjadi AST.

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

## Step 4 – Validation

Validator memeriksa:

* Required properties
* Invalid hierarchy
* Missing node types
* Unsupported configuration

---

## Step 5 – Code Generation

AST digunakan oleh generator.

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

# 9. AST Quality Attributes

## Correctness

AST harus merepresentasikan UI dengan akurat.

---

## Consistency

Node dengan tipe yang sama harus memiliki struktur yang konsisten.

---

## Determinism

Input yang sama harus menghasilkan AST yang sama.

---

## Stability

Generator dapat bergantung pada struktur AST yang stabil.

---

## Scalability

Target MVP:

* 100+ screens
* 500+ widgets
* 50+ routes

---

# 10. Future Evolution

AST dirancang agar dapat berkembang untuk:

## State Management

```text
StateNode
BindingNode
```

---

## Navigation

```text
NavigationNode
RouteNode
```

---

## Native APIs

```text
StorageNode
NetworkNode
```

---

## Web Support

```text
HTML Generator
```

---

## Desktop Support

```text
SwiftUI macOS
Compose Desktop
```

---

# 11. Success Criteria

AST dianggap berhasil apabila:

* Merepresentasikan seluruh widget MVP.
* Dapat digunakan oleh SwiftUI Generator.
* Dapat digunakan oleh Compose Generator.
* Tidak mengandung platform-specific implementation.
* Mendukung JSON serialization.
* Mendukung validation framework.
* Mendukung visitor traversal.
* Mendukung extensibility tanpa breaking changes.

---

# 12. Guiding Principle

AST bukan representasi SwiftUI.

AST bukan representasi Compose.

AST bukan rendering engine.

AST adalah representasi UI universal yang menjadi bahasa bersama antara Rust DSL dan seluruh code generator RustyKrab.

> Write Once. Generate Native.