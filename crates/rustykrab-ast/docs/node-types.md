# Node Types Specification

**Project:** RustyKrab
**Module:** AST Engine
**Document Type:** Architecture Specification
**Version:** 1.0
**Status:** Draft
**Owner:** Core Framework Team

---

# 1. Purpose

Dokumen ini mendefinisikan seluruh NodeType yang digunakan dalam RustyKrab AST.

NodeType merupakan identitas dari setiap AstNode dan digunakan untuk menentukan:

* Jenis elemen UI
* Aturan hierarchy
* Valid parent-child relationship
* Validation rules
* Generator behavior

NodeType menjadi kontrak utama antara:

```text
AST
 ↓
Validator
 ↓
Visitor
 ↓
SwiftUI Generator
 ↓
Compose Generator
```

Perubahan pada NodeType dianggap sebagai perubahan kontrak AST dan harus dilakukan secara hati-hati.

---

# 2. Design Goals

NodeType System dirancang untuk:

## Platform Independent

Tidak merepresentasikan implementasi SwiftUI maupun Compose.

---

## Extensible

Node baru dapat ditambahkan tanpa mengubah struktur fundamental AST.

---

## Predictable

Setiap NodeType memiliki aturan hierarchy yang jelas.

---

## Generator Friendly

Generator dapat menghasilkan source code berdasarkan NodeType tanpa knowledge terhadap DSL.

---

## Validation Friendly

Validator dapat menentukan apakah tree valid berdasarkan kombinasi NodeType.

---

# 3. Node Categories

NodeType dibagi menjadi tiga kategori utama.

```text
NodeType
│
├── Root Nodes
├── Layout Nodes
└── Widget Nodes
```

---

# 4. Root Nodes

Root Nodes merupakan node level tertinggi dalam tree.

---

## Root Hierarchy

```text
App
│
└── Screen
```

---

# 4.1 App

## Purpose

Representasi aplikasi.

Merupakan root utama AST.

---

## Responsibilities

* Menjadi root tree
* Menjadi entry point traversal
* Menjadi entry point validation
* Menjadi entry point generator

---

## Allowed Parent

```text
None
```

---

## Allowed Children

```text
Screen
```

---

## Required Properties

Tidak ada.

---

## Validation Rules

* Harus menjadi root node.
* Tidak boleh memiliki parent.
* Hanya boleh muncul satu kali dalam satu AstTree.

---

## Example

```text
App
└── Screen
```

---

# 4.2 Screen

## Purpose

Representasi sebuah halaman aplikasi.

---

## Responsibilities

* Menjadi container utama UI screen.
* Menjadi boundary generation unit.

---

## Allowed Parent

```text
App
```

---

## Allowed Children

```text
VStack
HStack
ScrollView
```

---

## Required Properties

Tidak ada.

---

## Validation Rules

* Harus memiliki parent App.
* Tidak boleh menjadi root node.

---

## Example

```text
App
└── Screen
     └── VStack
```

---

# 5. Layout Nodes

Layout nodes bertanggung jawab mengatur posisi dan struktur child node.

---

# Layout Hierarchy

```text
Layout
│
├── VStack
├── HStack
├── ScrollView
└── Spacer
```

---

# 5.1 VStack

## Purpose

Menyusun child secara vertikal.

---

## Generator Mapping

SwiftUI:

```swift
VStack
```

Compose:

```kotlin
Column
```

---

## Allowed Parent

```text
Screen
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
Any Layout Node
Any Widget Node
```

---

## Optional Properties

```text
spacing
padding
alignment
```

---

## Validation Rules

* Boleh memiliki child 0 atau lebih.
* Child harus valid NodeType.

---

## Example

```text
VStack
├── Text
└── Button
```

---

# 5.2 HStack

## Purpose

Menyusun child secara horizontal.

---

## Generator Mapping

SwiftUI:

```swift
HStack
```

Compose:

```kotlin
Row
```

---

## Allowed Parent

```text
Screen
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
Any Layout Node
Any Widget Node
```

---

## Optional Properties

```text
spacing
padding
alignment
```

---

## Example

```text
HStack
├── Text
└── Button
```

---

# 5.3 ScrollView

## Purpose

Container yang dapat di-scroll.

---

## Generator Mapping

SwiftUI:

```swift
ScrollView
```

Compose:

```kotlin
LazyColumn
```

---

## Allowed Parent

```text
Screen
VStack
HStack
```

---

## Allowed Children

```text
Any Layout Node
Any Widget Node
```

---

## Optional Properties

```text
axis
shows_indicator
```

---

## Validation Rules

Minimal memiliki satu child.

---

## Example

```text
ScrollView
└── VStack
```

---

# 5.4 Spacer

## Purpose

Flexible spacing element.

---

## Generator Mapping

SwiftUI:

```swift
Spacer()
```

Compose:

```kotlin
Spacer()
```

---

## Allowed Parent

```text
VStack
HStack
```

---

## Allowed Children

```text
None
```

---

## Validation Rules

Tidak boleh memiliki child.

---

## Example

```text
HStack
├── Text
├── Spacer
└── Button
```

---

# 6. Widget Nodes

Widget nodes merupakan elemen UI yang terlihat oleh user.

---

# Widget Hierarchy

```text
Widget
│
├── Text
├── Button
├── Image
└── TextField
```

---

# 6.1 Text

## Purpose

Menampilkan teks.

---

## Generator Mapping

SwiftUI:

```swift
Text("Hello")
```

Compose:

```kotlin
Text("Hello")
```

---

## Allowed Parent

```text
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
None
```

---

## Required Properties

```text
text
```

---

## Optional Properties

```text
font_size
font_weight
color
```

---

## Validation Rules

Property `text` wajib tersedia.

---

# 6.2 Button

## Purpose

Menampilkan tombol aksi.

---

## Generator Mapping

SwiftUI:

```swift
Button("Login")
```

Compose:

```kotlin
Button()
```

---

## Allowed Parent

```text
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
None
```

---

## Required Properties

```text
title
```

---

## Optional Properties

```text
action
style
enabled
```

---

## Validation Rules

Property `title` wajib tersedia.

---

# 6.3 Image

## Purpose

Menampilkan gambar.

---

## Generator Mapping

SwiftUI:

```swift
Image(...)
```

Compose:

```kotlin
Image(...)
```

---

## Allowed Parent

```text
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
None
```

---

## Required Properties

```text
source
```

---

## Optional Properties

```text
width
height
content_mode
```

---

## Validation Rules

Property `source` wajib tersedia.

---

# 6.4 TextField

## Purpose

Input teks.

---

## Generator Mapping

SwiftUI:

```swift
TextField(...)
```

Compose:

```kotlin
TextField(...)
```

---

## Allowed Parent

```text
VStack
HStack
ScrollView
```

---

## Allowed Children

```text
None
```

---

## Required Properties

```text
placeholder
```

---

## Optional Properties

```text
binding
keyboard_type
secure
```

---

## Validation Rules

Property `placeholder` wajib tersedia.

---

# 7. NodeType Enumeration

Implementasi awal NodeType.

```rust
pub enum NodeType {
    App,
    Screen,

    VStack,
    HStack,
    ScrollView,
    Spacer,

    Text,
    Button,
    Image,
    TextField,
}
```

---

# 8. Parent-Child Matrix

| Parent     | Allowed Children           |
| ---------- | -------------------------- |
| App        | Screen                     |
| Screen     | VStack, HStack, ScrollView |
| VStack     | Layout + Widget            |
| HStack     | Layout + Widget            |
| ScrollView | Layout + Widget            |
| Spacer     | None                       |
| Text       | None                       |
| Button     | None                       |
| Image      | None                       |
| TextField  | None                       |

---

# 9. Valid Hierarchy Examples

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

# 10. Invalid Hierarchy Examples

## Example 1

```text
Text
└── Button
```

Invalid.

Widget tidak boleh memiliki child.

---

## Example 2

```text
Spacer
└── Text
```

Invalid.

Spacer tidak boleh memiliki child.

---

## Example 3

```text
Screen
└── Screen
```

Invalid.

Screen tidak boleh memiliki child Screen.

---

## Example 4

```text
App
└── Button
```

Invalid.

App hanya boleh memiliki Screen.

---

# 11. Validation Requirements

Validator wajib memeriksa:

---

## Rule 1

NodeType harus valid.

---

## Rule 2

Parent-child relationship harus valid.

---

## Rule 3

Required property harus tersedia.

---

## Rule 4

Widget node tidak boleh memiliki child.

---

## Rule 5

Spacer tidak boleh memiliki child.

---

## Rule 6

App harus menjadi root node.

---

# 12. Future Node Types

Node berikut direncanakan untuk versi berikutnya.

---

## Layout

```text
Grid
LazyVStack
LazyHStack
ZStack
```

---

## Widgets

```text
Toggle
Slider
Picker
List
Map
ProgressView
```

---

## Navigation

```text
NavigationStack
Route
TabView
```

---

## State Management

```text
StateNode
BindingNode
```

---

# 13. Versioning Strategy

NodeType baru dapat ditambahkan.

NodeType yang sudah dirilis tidak boleh dihapus tanpa migration path.

Hal ini menjaga kompatibilitas AST terhadap generator lama.

---

# 14. Success Criteria

NodeType System dianggap berhasil apabila:

* Seluruh widget MVP terdefinisi.
* Seluruh layout MVP terdefinisi.
* Parent-child hierarchy tervalidasi.
* Generator dapat melakukan mapping ke SwiftUI.
* Generator dapat melakukan mapping ke Compose.
* Mendukung ekspansi tanpa breaking changes.

---

# 15. Guiding Principle

NodeType merepresentasikan konsep UI universal, bukan implementasi framework tertentu.

Generator bertanggung jawab menerjemahkan NodeType menjadi kode native.

```text
NodeType
     ↓
Generator
     ↓
SwiftUI / Compose
```

Dengan demikian AST tetap stabil, extensible, dan platform-independent sepanjang siklus hidup RustyKrab.
