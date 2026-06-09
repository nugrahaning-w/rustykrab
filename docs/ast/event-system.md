# Event System

**Project:** RustyKrab
**Module:** rustykrab-ast
**Document Type:** Design Specification
**Version:** 1.0
**Status:** Draft

---

# 1. Purpose

This document defines the Event System used by the RustyKrab AST.

The Event System provides a platform-independent abstraction for user interactions and system-generated events.

The primary goal is to separate UI declaration from event execution while allowing generators to map events into native platform implementations.

Supported platforms:

* SwiftUI
* Jetpack Compose

Future Platforms:

* Web
* Desktop
* WASM

---

# 2. Design Goals

## DG-001 Platform Independence

Events must not contain platform-specific implementations.

Invalid:

```text
UIButton.addTarget

View.OnClickListener

GestureDetector
```

Valid:

```text
Click
Change
Submit
Appear
```

---

## DG-002 Declarative Architecture

Widgets describe what event occurs.

Business logic determines what action executes.

Example:

```rust
Button::new("Login")
    .on_click("login")
```

The button describes an event.

It does not contain implementation details.

---

## DG-003 Generator Agnostic

The same event definition must work across generators.

Example:

```text
Click
```

Maps to:

```text
SwiftUI
Button Action

Compose
onClick
```

---

## DG-004 Extensibility

New event types must be added without redesigning Node.

---

## DG-005 State Integration

Events must integrate with future state management.

Example:

```text
Click
 ↓
Action
 ↓
State Update
 ↓
UI Refresh
```

---

# 3. Architecture Overview

```text
Node
 │
 ├── Properties
 ├── Modifiers
 ├── Events
 ├── Bindings
 └── Children
```

Events represent interactions attached to a node.

---

# 4. Event Model

## EventHandler

```rust
pub struct EventHandler {
    pub id: EventId,

    pub event: EventType,

    pub action: ActionReference,
}
```

---

# 5. EventId

Every event handler must have a unique identifier.

```rust
pub struct EventId(pub String);
```

Example:

```rust
EventId(
    "button_login_click"
)
```

Purpose:

* Debugging
* Analytics
* Tooling
* Tracing

---

# 6. EventType

Represents the event that occurs.

```rust
pub enum EventType {
    Click,

    DoubleClick,

    LongPress,

    Change,

    Submit,

    Focus,

    Blur,

    Appear,

    Disappear,

    Refresh,

    Scroll,

    Selection,

    Custom(String),
}
```

---

# 7. Event Categories

Events are grouped into categories.

---

## User Interaction Events

Triggered directly by the user.

Examples:

```text
Click

DoubleClick

LongPress

Selection
```

---

## Input Events

Triggered by input components.

Examples:

```text
Change

Submit

Focus

Blur
```

---

## Lifecycle Events

Triggered by widget lifecycle.

Examples:

```text
Appear

Disappear
```

---

## Container Events

Triggered by containers.

Examples:

```text
Scroll

Refresh
```

---

## Custom Events

Used by plugins.

Example:

```rust
Custom(
    "MapTapped"
)
```

---

# 8. ActionReference

Events do not execute logic directly.

They reference actions.

```rust
pub struct ActionReference {
    pub name: String,
}
```

---

Example:

```rust
Button::new("Login")
    .on_click("login")
```

AST:

```json
{
  "event": "Click",
  "action": {
    "name": "login"
  }
}
```

---

# 9. Event Examples

## Button Click

DSL:

```rust
Button::new("Login")
    .on_click("login")
```

AST:

```json
{
  "kind": "Button",
  "events": [
    {
      "event": "Click",
      "action": {
        "name": "login"
      }
    }
  ]
}
```

---

## TextField Change

DSL:

```rust
TextField::new()
    .on_change("email_changed")
```

AST:

```json
{
  "kind": "TextField",
  "events": [
    {
      "event": "Change",
      "action": {
        "name": "email_changed"
      }
    }
  ]
}
```

---

## Screen Appear

DSL:

```rust
Screen::new()
    .on_appear("load_user")
```

AST:

```json
{
  "events": [
    {
      "event": "Appear",
      "action": {
        "name": "load_user"
      }
    }
  ]
}
```

---

# 10. Multiple Event Support

A node may contain multiple events.

Example:

```rust
Button::new("Save")
    .on_click("save")
    .on_focus("track_focus")
```

AST:

```json
{
  "events": [
    {
      "event": "Click"
    },
    {
      "event": "Focus"
    }
  ]
}
```

---

# 11. Event Dispatch Flow

```text
User Interaction
        │
        ▼
     Event
        │
        ▼
 Event Handler
        │
        ▼
 Action Reference
        │
        ▼
 State Update
        │
        ▼
 UI Refresh
```

---

# 12. Generator Mapping

## SwiftUI

| EventType | SwiftUI       |
| --------- | ------------- |
| Click     | Button Action |
| Change    | onChange      |
| Appear    | onAppear      |
| Disappear | onDisappear   |
| Focus     | FocusState    |
| Submit    | onSubmit      |

---

## Compose

| EventType | Compose          |
| --------- | ---------------- |
| Click     | onClick          |
| Change    | onValueChange    |
| Appear    | LaunchedEffect   |
| Disappear | DisposableEffect |
| Focus     | onFocusChanged   |
| Submit    | KeyboardActions  |

---

# 13. Future Async Actions

The event system must support asynchronous actions.

Example:

```rust
Button::new("Login")
    .on_click("authenticate")
```

Action:

```rust
async fn authenticate() {
}
```

AST remains unchanged.

Only runtime execution changes.

---

# 14. Future Navigation Integration

Events will trigger navigation actions.

Example:

```rust
Button::new("Profile")
    .on_click("navigate_profile")
```

Flow:

```text
Click
 ↓
Action
 ↓
Navigator::push()
```

The Event System remains unchanged.

---

# 15. Future Analytics Integration

Events may be intercepted.

Example:

```text
Click
 ↓
Analytics
 ↓
Action
```

Use cases:

* Event tracking
* Metrics
* Logging
* A/B testing

---

# 16. Validation Rules

## ES-001

Every event must have EventType.

Valid:

```json
{
  "event": "Click"
}
```

Invalid:

```json
{
}
```

---

## ES-002

Every event must have ActionReference.

Valid:

```json
{
  "event": "Click",
  "action": {
    "name": "login"
  }
}
```

Invalid:

```json
{
  "event": "Click"
}
```

---

## ES-003

Event identifiers must be unique within a node.

---

## ES-004

Custom events must have a non-empty name.

Valid:

```rust
Custom("MapTapped")
```

Invalid:

```rust
Custom("")
```

---

# 17. Serialization Example

```json
{
  "id": "login_button",
  "kind": "Button",
  "events": [
    {
      "id": "login_click",
      "event": "Click",
      "action": {
        "name": "login"
      }
    }
  ]
}
```

---

# 18. Why Use Event References?

Rejected Design:

```rust
Button::new("Login")
    .on_click(|| {
        authenticate();
    })
```

Problems:

* Difficult serialization
* Difficult code generation
* Platform coupling
* Plugin limitations

---

Selected Design:

```rust
Button::new("Login")
    .on_click("authenticate")
```

Benefits:

* Serializable
* Platform-independent
* Generator-friendly
* Supports tooling
* Supports remote analysis

---

# 19. Future Expansion

Future event types:

```text
Drag

Swipe

Pinch

Rotate

Hover

Keyboard

LocationChanged

NetworkChanged
```

Future action targets:

```text
State

Navigation

Analytics

Async Commands

Plugin Actions
```

No AST redesign should be required.

---

# 20. Architectural Decisions

| Decision                 | Reason                          |
| ------------------------ | ------------------------------- |
| EventHandler abstraction | Consistent event representation |
| ActionReference used     | Separation of UI and logic      |
| Multiple events per node | Flexibility                     |
| Custom events supported  | Plugin readiness                |
| Event IDs included       | Diagnostics and tooling         |
| Async support planned    | Production readiness            |

---

# 21. Acceptance Criteria

This design is considered complete when:

* EventHandler model is defined
* EventType system is defined
* ActionReference is defined
* Event categories are documented
* Generator mappings are documented
* Validation rules are documented
* Serialization examples are provided
* Future expansion strategy is documented

---