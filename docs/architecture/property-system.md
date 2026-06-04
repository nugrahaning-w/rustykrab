# Property System Specification

Version: 1.0
Project: RustyKrab
Module: AST Engine
Status: Draft
Author: Aji Nugrahaning Widhi
Last Updated: June 2026

---

# 1. Overview

Property System adalah mekanisme yang digunakan AST untuk menyimpan konfigurasi setiap node.

Semua informasi yang diperlukan generator untuk menghasilkan source code native disimpan dalam property.

Contoh:

Rust DSL

```rust
Text::new("Hello World")
```

AST

```json
{
  "type": "Text",
  "properties": {
    "text": "Hello World"
  }
}
```

SwiftUI Generator

```swift
Text("Hello World")
```

Compose Generator

```kotlin
Text("Hello World")
```

Property System menjadi kontrak utama antara AST dan Code Generator.

---

# 2. Objectives

Property System harus:

* Platform Independent
* Serializable
* Extensible
* Type Safe
* Generator Friendly
* Backward Compatible

---

# 3. Design Principles

## Platform Neutral

Property tidak boleh mengandung implementasi SwiftUI maupun Compose.

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

Seluruh property harus dapat diubah menjadi JSON.

---

## Extensible

Node baru dapat menambahkan property baru tanpa mengubah arsitektur inti.

---

## Deterministic

Input yang sama harus menghasilkan property yang sama.

---

# 4. Property Model

## Properties

Properties merupakan kumpulan key-value yang dimiliki sebuah node.

---

### Rust Definition

```rust
pub type Properties =
HashMap<String, PropertyValue>;
```

---

### Example

```json
{
  "text": "Hello",
  "font_size": 18
}
```

---

# 5. PropertyValue

PropertyValue adalah tipe data universal yang digunakan AST.

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

## Supported Types

### String

Contoh:

```json
{
  "text": "Hello World"
}
```

---

### Integer

Contoh:

```json
{
  "padding": 16
}
```

---

### Float

Contoh:

```json
{
  "opacity": 0.8
}
```

---

### Bool

Contoh:

```json
{
  "enabled": true
}
```

---

# 6. Standard Property Naming Rules

Semua property menggunakan:

```text
snake_case
```

---

## Valid

```text
font_size
font_weight
content_mode
```

---

## Invalid

```text
fontSize
FontSize
FONT_SIZE
```

---

# 7. Common Properties

Property berikut dapat digunakan oleh banyak node.

---

## padding

Type

```text
Integer
```

Example

```json
{
  "padding": 16
}
```

---

## spacing

Type

```text
Integer
```

Example

```json
{
  "spacing": 12
}
```

---

## alignment

Type

```text
String
```

Values

```text
leading
center
trailing
```

---

## width

Type

```text
Integer
```

---

## height

Type

```text
Integer
```

---

## enabled

Type

```text
Bool
```

Default

```text
true
```

---

# 8. Text Properties

Node Type:

```text
Text
```

---

## Required Properties

### text

Type

```text
String
```

Example

```json
{
  "text": "Hello World"
}
```

---

## Optional Properties

### font_size

Type

```text
Integer
```

Example

```json
{
  "font_size": 18
}
```

---

### font_weight

Type

```text
String
```

Values

```text
light
regular
medium
bold
```

---

### color

Type

```text
String
```

Example

```json
{
  "color": "#FF0000"
}
```

---

# 9. Button Properties

Node Type:

```text
Button
```

---

## Required Properties

### title

Type

```text
String
```

Example

```json
{
  "title": "Login"
}
```

---

## Optional Properties

### action

Type

```text
String
```

Example

```json
{
  "action": "login"
}
```

---

### style

Type

```text
String
```

Values

```text
primary
secondary
danger
```

---

### enabled

Type

```text
Bool
```

---

# 10. Image Properties

Node Type:

```text
Image
```

---

## Required Properties

### source

Type

```text
String
```

Example

```json
{
  "source": "logo.png"
}
```

---

## Optional Properties

### width

Type

```text
Integer
```

---

### height

Type

```text
Integer
```

---

### content_mode

Type

```text
String
```

Values

```text
fit
fill
```

---

# 11. TextField Properties

Node Type:

```text
TextField
```

---

## Required Properties

### placeholder

Type

```text
String
```

Example

```json
{
  "placeholder": "Enter email"
}
```

---

## Optional Properties

### binding

Type

```text
String
```

Example

```json
{
  "binding": "email"
}
```

---

### keyboard_type

Type

```text
String
```

Values

```text
default
email
number
phone
```

---

### secure

Type

```text
Bool
```

Default

```text
false
```

---

# 12. VStack Properties

Node Type:

```text
VStack
```

---

## Optional Properties

### spacing

Type

```text
Integer
```

---

### padding

Type

```text
Integer
```

---

### alignment

Type

```text
String
```

Values

```text
leading
center
trailing
```

---

# 13. HStack Properties

Node Type:

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

# 14. ScrollView Properties

Node Type:

```text
ScrollView
```

---

## Optional Properties

### axis

Type

```text
String
```

Values

```text
vertical
horizontal
```

Default

```text
vertical
```

---

### shows_indicator

Type

```text
Bool
```

Default

```text
true
```

---

# 15. Spacer Properties

Node Type:

```text
Spacer
```

---

## Optional Properties

### min_length

Type

```text
Integer
```

Example

```json
{
  "min_length": 20
}
```

---

# 16. Property Validation Rules

## Rule 1

Required property wajib tersedia.

Contoh:

Text wajib memiliki:

```json
{
  "text": "Hello"
}
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

Unknown property menghasilkan warning.

Contoh:

```json
{
  "banana": true
}
```

---

## Rule 4

Property name harus snake_case.

Valid:

```text
font_size
```

Invalid:

```text
fontSize
```

---

# 17. Generator Mapping Examples

## Text

AST

```json
{
  "text": "Hello"
}
```

SwiftUI

```swift
Text("Hello")
```

Compose

```kotlin
Text("Hello")
```

---

## Button

AST

```json
{
  "title": "Login"
}
```

SwiftUI

```swift
Button("Login") {}
```

Compose

```kotlin
Button(
    onClick = {}
) {
    Text("Login")
}
```

---

## VStack

AST

```json
{
  "spacing": 16
}
```

SwiftUI

```swift
VStack(spacing: 16)
```

Compose

```kotlin
Column(
    verticalArrangement =
        Arrangement.spacedBy(16.dp)
)
```

---

# 18. Future Property Extensions

Versi berikutnya akan mendukung:

## Layout

```text
margin
frame
max_width
max_height
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

## State

```text
binding
state_key
```

---

## Navigation

```text
route
destination
```

---

# 19. Success Criteria

Property System dianggap berhasil apabila:

* Seluruh widget MVP memiliki property specification.
* Seluruh layout MVP memiliki property specification.
* Generator dapat menggunakan property tanpa ambiguity.
* Property dapat di-serialize ke JSON.
* Property dapat divalidasi.
* Property dapat diperluas tanpa breaking changes.

---

# 20. Guiding Principle

Property System harus menyimpan informasi yang cukup untuk menghasilkan source code native tanpa pernah menyimpan implementasi spesifik platform.

Property adalah bahasa universal antara AST dan Generator.

> AST mendefinisikan struktur UI.
> Property mendefinisikan konfigurasi UI.
> Generator menerjemahkan keduanya menjadi source code native.