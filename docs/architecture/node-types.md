# Node Types Specification

Version: 1.0
Project: RustyKrab
Module: AST Engine
Status: Draft
Author: Aji Nugrahaning Widhi
Last Updated: June 2026

---

# 1. Overview

Dokumen ini mendefinisikan seluruh jenis node (`NodeType`) yang didukung oleh RustyKrab AST.

NodeType digunakan untuk mengidentifikasi fungsi dan perilaku setiap node dalam AST.

Dokumen ini menjadi referensi utama untuk:

* AST Engine
* Validation Framework
* SwiftUI Generator
* Compose Generator
* Future Extensions

---

# 2. Objectives

Tujuan utama NodeType System:

* Mengidentifikasi jenis node dalam AST
* Menentukan hierarchy rules
* Menentukan valid parent-child relationship
* Menentukan property requirements
* Menyediakan kontrak yang stabil untuk generator

---

# 3. Node Categories

RustyKrab MVP memiliki tiga kategori node.

```text
NodeType
│
├── Root Nodes
│
├── Layout Nodes
│
└── Widget Nodes
```

---

# 4. Root Nodes

Root node berada di level tertinggi AST.

---

## 4.1 App

### Purpose

Representasi aplikasi.

Merupakan root utama dari seluruh AST.

---

### Example

```text
App
```

---

### Allowed Parent

Tidak memiliki parent.

```text
None
```

---

### Allowed Children

```text
Screen
```

---

### Required Properties

Tidak ada.

---

### Validation Rules

* Hanya boleh muncul sekali dalam satu AstTree.
* Harus menjadi root node.

---

## 4.2 Screen

### Purpose

Representasi sebuah halaman aplikasi.

---

### Example

```text
App
└── Screen
```

---

### Allowed Parent

```text
App
```

---

### Allowed Children

```text
VStack
HStack
ScrollView
```

---

### Required Properties

Tidak ada.

---

### Validation Rules

* Wajib memiliki parent App.
* Tidak boleh menjadi root.

---

# 5. Layout Nodes

Layout nodes bertanggung jawab mengatur posisi child node.

---

## 5.1 VStack

### Purpose

Menyusun child secara vertikal.

---

### Example

```text
VStack
├── Text
└── Button
```

---

### Allowed Parent

```text
Screen
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
Any Layout
Any Widget
```

---

### Required Properties

Tidak ada.

---

### Optional Properties

```text
spacing
padding
alignment
```

---

### Validation Rules

* Boleh memiliki child 0 atau lebih.
* Child harus valid NodeType.

---

## 5.2 HStack

### Purpose

Menyusun child secara horizontal.

---

### Example

```text
HStack
├── Button
└── Button
```

---

### Allowed Parent

```text
Screen
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
Any Layout
Any Widget
```

---

### Optional Properties

```text
spacing
padding
alignment
```

---

### Validation Rules

Sama seperti VStack.

---

## 5.3 ScrollView

### Purpose

Container yang dapat di-scroll.

---

### Example

```text
ScrollView
└── VStack
```

---

### Allowed Parent

```text
Screen
VStack
HStack
```

---

### Allowed Children

```text
Any Layout
Any Widget
```

---

### Optional Properties

```text
axis
shows_indicator
```

---

### Validation Rules

* Minimal memiliki satu child.

---

## 5.4 Spacer

### Purpose

Flexible spacing.

---

### Example

```text
HStack
├── Text
├── Spacer
└── Button
```

---

### Allowed Parent

```text
VStack
HStack
```

---

### Allowed Children

```text
None
```

---

### Required Properties

Tidak ada.

---

### Validation Rules

Spacer tidak boleh memiliki child.

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

# 6. Widget Nodes

Widget node adalah elemen UI yang dapat dilihat user.

---

## 6.1 Text

### Purpose

Menampilkan teks.

---

### Example

```text
Text
```

---

### Allowed Parent

```text
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
None
```

---

### Required Properties

```text
text
```

---

### Optional Properties

```text
font_size
font_weight
color
```

---

### Validation Rules

Property text wajib ada.

---

## 6.2 Button

### Purpose

Tombol aksi.

---

### Example

```text
Button
```

---

### Allowed Parent

```text
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
None
```

---

### Required Properties

```text
title
```

---

### Optional Properties

```text
action
style
```

---

### Validation Rules

title wajib ada.

---

## 6.3 Image

### Purpose

Menampilkan gambar.

---

### Example

```text
Image
```

---

### Allowed Parent

```text
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
None
```

---

### Required Properties

```text
source
```

---

### Optional Properties

```text
width
height
content_mode
```

---

### Validation Rules

source wajib ada.

---

## 6.4 TextField

### Purpose

Input teks.

---

### Example

```text
TextField
```

---

### Allowed Parent

```text
VStack
HStack
ScrollView
```

---

### Allowed Children

```text
None
```

---

### Required Properties

```text
placeholder
```

---

### Optional Properties

```text
binding
keyboard_type
```

---

### Validation Rules

placeholder wajib ada.

---

# 7. NodeType Enum

Implementasi awal.

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

# 8. Parent-Child Hierarchy Matrix

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

# 11. Validation Rules

Validator wajib memeriksa:

### Rule 1

NodeType harus valid.

---

### Rule 2

Parent-child relationship harus valid.

---

### Rule 3

Required property harus tersedia.

---

### Rule 4

Widget node tidak boleh memiliki child.

---

### Rule 5

Spacer tidak boleh memiliki child.

---

### Rule 6

App harus menjadi root node.

---

# 12. Future Node Types

NodeType berikut direncanakan untuk versi berikutnya.

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

## State

```text
StateNode
BindingNode
```

---

# 13. Success Criteria

NodeType System dianggap berhasil apabila:

* Seluruh widget MVP terdefinisi.
* Seluruh layout MVP terdefinisi.
* Parent-child hierarchy tervalidasi.
* Dapat digunakan oleh Validation Framework.
* Dapat digunakan oleh SwiftUI Generator.
* Dapat digunakan oleh Compose Generator.
* Mendukung penambahan node baru tanpa breaking changes.

---

# 14. Guiding Principle

Setiap NodeType harus merepresentasikan konsep UI yang bersifat universal dan tidak bergantung pada implementasi platform tertentu.

NodeType bukan representasi SwiftUI maupun Jetpack Compose, melainkan representasi abstrak yang dapat diterjemahkan ke berbagai platform native.
