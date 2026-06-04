# Property System Specification

**Project:** RustyKrab
**Module:** AST Engine
**Document Type:** Architecture Specification
**Version:** 1.0
**Status:** Draft
**Owner:** Core Framework Team

---

# 1. Purpose

Property System merupakan mekanisme yang digunakan RustyKrab AST untuk menyimpan konfigurasi setiap node.

AST bertanggung jawab merepresentasikan struktur UI.

Property System bertanggung jawab merepresentasikan konfigurasi UI.

Contoh:

```text
AST Structure
└── Text

Property System
└── text = "Hello World"
```

Property System menjadi kontrak utama antara:

```text
Rust DSL
     ↓
AST
     ↓
Property System
     ↓
Generator
```

Generator tidak memahami Rust DSL.

Generator hanya membaca:

* NodeType
* Properties

untuk menghasilkan source code platform target.

---

# 2. Design Goals

Property System harus memenuhi karakteristik berikut:

---

## Platform Independent

Property tidak boleh menyimpan informasi spesifik platform.

Invalid:

```json
{
  "swift_modifier": ".padding(16)"
}
```

Valid:

```json
{
  "padding": 16
}
```

Generator bertanggung jawab melakukan mapping.

---

## Serializable

Property harus dapat diubah menjadi JSON.

---

## Deterministic

Input yang sama harus menghasilkan property yang sama.

---

## Extensible

Property baru dapat ditambahkan tanpa mengubah struktur inti AST.

---

## Generator Friendly

Generator harus dapat membaca property secara langsung tanpa transformasi kompleks.

---

# 3. Property System Architecture

Property System terdiri dari:

```text
Properties
│
└── PropertyValue
```

---

## Architecture Diagram

```text
AstNode
│
├── NodeType
├── Properties
│    ├── key
│    └── PropertyValue
│
└── Children
```

---

# 4. Properties

## Purpose

Properties merupakan kumpulan konfigurasi yang dimiliki sebuah node.

---

## Rust Definition

```rust
pub type Properties =
HashMap<String, PropertyValue>;
```

---

## Example

```json
{
  "text": "Hello World",
  "font_size": 18
}
```

---

## Responsibilities

Properties bertanggung jawab untuk:

* Menyimpan konfigurasi UI
* Menyimpan metadata generator
* Menjadi input validator
* Menjadi input generator

---

# 5. PropertyValue

## Purpose

PropertyValue adalah representasi universal seluruh tipe data property.

---

## Rust Definition

```rust
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}
```

---

## Serialization Requirement

PropertyValue harus mendukung:

```rust
Serialize
Deserialize
```

menggunakan Serde.

---

# 6. Supported Primitive Types

---

## String

Digunakan untuk:

* Text
* Title
* Color
* Identifier
* Resource Name

Example:

```json
{
  "text": "Hello"
}
```

---

## Integer

Digunakan untuk:

* Width
* Height
* Padding
* Font Size

Example:

```json
{
  "padding": 16
}
```

---

## Float

Digunakan untuk:

* Opacity
* Scale
* Progress

Example:

```json
{
  "opacity": 0.8
}
```

---

## Bool

Digunakan untuk:

* Enabled
* Visible
* Secure Input

Example:

```json
{
  "enabled": true
}
```

---

# 7. Property Naming Convention

Semua property menggunakan format:

```text
snake_case
```

---

## Valid

```text
font_size
font_weight
content_mode
keyboard_type
```

---

## Invalid

```text
fontSize
FontSize
FONT_SIZE
```

---

# 8. Common Properties

Property berikut dapat digunakan oleh berbagai NodeType.

---

## padding

Type:

```text
Integer
```

Example:

```json
{
  "padding": 16
}
```

---

## spacing

Type:

```text
Integer
```

Example:

```json
{
  "spacing": 12
}
```

---

## width

Type:

```text
Integer
```

Example:

```json
{
  "width": 200
}
```

---

## height

Type:

```text
Integer
```

Example:

```json
{
  "height": 100
}
```

---

## enabled

Type:

```text
Bool
```

Default:

```text
true
```

---

# 9. Text Node Properties

NodeType:

```text
Text
```

---

## Required Properties

### text

Type:

```text
String
```

Example:

```json
{
  "text": "Hello World"
}
```

---

## Optional Properties

### font_size

Type:

```text
Integer
```

Example:

```json
{
  "font_size": 18
}
```

---

### font_weight

Type:

```text
String
```

Values:

```text
light
regular
medium
bold
```

---

### color

Type:

```text
String
```

Example:

```json
{
  "color": "#FF0000"
}
```

---

# 10. Button Node Properties

NodeType:

```text
Button
```

---

## Required Properties

### title

Type:

```text
String
```

Example:

```json
{
  "title": "Login"
}
```

---

## Optional Properties

### action

Type:

```text
String
```

Example:

```json
{
  "action": "login"
}
```

---

### style

Type:

```text
String
```

Values:

```text
primary
secondary
danger
```

---

### enabled

Type:

```text
Bool
```

---

# 11. Image Node Properties

NodeType:

```text
Image
```

---

## Required Properties

### source

Type:

```text
String
```

Example:

```json
{
  "source": "logo.png"
}
```

---

## Optional Properties

### width

Type:

```text
Integer
```

---

### height

Type:

```text
Integer
```

---

### content_mode

Type:

```text
String
```

Values:

```text
fit
fill
```

---

# 12. TextField Node Properties

NodeType:

```text
TextField
```

---

## Required Properties

### placeholder

Type:

```text
String
```

Example:

```json
{
  "placeholder": "Enter email"
}
```

---

## Optional Properties

### binding

Type:

```text
String
```

Example:

```json
{
  "binding": "email"
}
```

---

### keyboard_type

Type:

```text
String
```

Values:

```text
default
email
number
phone
```

---

### secure

Type:

```text
Bool
```

Default:

```text
false
```

---

# 13. VStack Properties

NodeType:

```text
VStack
```

---

## Optional Properties

### spacing

Type:

```text
Integer
```

---

### padding

Type:

```text
Integer
```

---

### alignment

Type:

```text
String
```

Values:

```text
leading
center
trailing
```

---

# 14. HStack Properties

NodeType:

```text
HStack
```

---

## Optional Properties

### spacing

### padding

### alignment

Sama seperti VStack.

---

# 15. ScrollView Properties

NodeType:

```text
ScrollView
```

---

## Optional Properties

### axis

Type:

```text
String
```

Values:

```text
vertical
horizontal
```

Default:

```text
vertical
```

---

### shows_indicator

Type:

```text
Bool
```

Default:

```text
true
```

---

# 16. Spacer Properties

NodeType:

```text
Spacer
```

---

## Optional Properties

### min_length

Type:

```text
Integer
```

Example:

```json
{
  "min_length": 20
}
```

---

# 17. Property Validation Rules

Validator wajib memverifikasi seluruh property.

---

## Rule 1

Required property wajib tersedia.

Valid:

```json
{
  "text": "Hello"
}
```

Invalid:

```json
{}
```

untuk NodeType:

```text
Text
```

---

## Rule 2

Property type harus sesuai.

Invalid:

```json
{
  "padding": "large"
}
```

Valid:

```json
{
  "padding": 16
}
```

---

## Rule 3

Property name harus menggunakan snake_case.

Valid:

```text
font_size
```

Invalid:

```text
fontSize
```

---

## Rule 4

Unknown property menghasilkan warning.

Example:

```json
{
  "banana": true
}
```

---

# 18. Serialization Requirements

Property System harus mendukung:

---

## Serialize

```rust
serde_json::to_string()
```

---

## Deserialize

```rust
serde_json::from_str()
```

---

## Roundtrip Safety

Harus selalu berlaku:

```text
Properties
     ↓
JSON
     ↓
Properties
```

tanpa kehilangan informasi.

---

# 19. Generator Mapping Examples

---

## Text

AST:

```json
{
  "text": "Hello"
}
```

SwiftUI:

```swift
Text("Hello")
```

Compose:

```kotlin
Text("Hello")
```

---

## Button

AST:

```json
{
  "title": "Login"
}
```

SwiftUI:

```swift
Button("Login") {}
```

Compose:

```kotlin
Button(
    onClick = {}
) {
    Text("Login")
}
```

---

## VStack

AST:

```json
{
  "spacing": 16
}
```

SwiftUI:

```swift
VStack(spacing: 16)
```

Compose:

```kotlin
Column(
    verticalArrangement =
        Arrangement.spacedBy(16.dp)
)
```

---

# 20. Future Extensions

Versi berikutnya akan mendukung:

---

## Layout

```text
margin
max_width
max_height
frame
```

---

## Styling

```text
background_color
border_color
corner_radius
shadow
```

---

## Navigation

```text
route
destination
```

---

## State Management

```text
state_key
binding
observable
```

---

# 21. Versioning Strategy

Property baru dapat ditambahkan tanpa breaking changes.

Property yang sudah dirilis tidak boleh dihapus tanpa migration path yang jelas.

---

# 22. Success Criteria

Property System dianggap berhasil apabila:

* Seluruh NodeType MVP memiliki property specification.
* Property dapat di-serialize ke JSON.
* Property dapat di-deserialize dari JSON.
* Property dapat divalidasi.
* Generator dapat menggunakan property tanpa ambiguity.
* Mendukung ekspansi tanpa perubahan fundamental AST.

---

# 23. Guiding Principle

AST mendefinisikan struktur UI.

Property System mendefinisikan konfigurasi UI.

Generator menerjemahkan keduanya menjadi source code native.

```text
NodeType
     +
Properties
     ↓
Generator
     ↓
SwiftUI / Compose
```

Property System harus tetap sederhana, extensible, platform-independent, dan menjadi kontrak stabil antara AST Engine dan seluruh Generator dalam ekosistem RustyKrab.
