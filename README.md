# event_bridge
[![Pipeline Status](https://github.com/mat-kie/event_bridge/actions/workflows/rust.yml/badge.svg)](https://github.com/mat-kie/event_bridge/actions/workflows/rust.yml)

A procedural macro that generates asynchronous event handlers from enum
definitions, automatically dispatching enum variants to associated trait
methods. This reduces boilerplate and simplifies the integration between
synchronous UI events and asynchronous backend logic.

## Motivation

In many applications, especially those with graphical user interfaces (GUIs),
input from the UI thread must be forwarded to asynchronous backend services or
controllers. Writing repetitive `match` expressions to route each event to its
corresponding async API method can be tedious and error-prone.

`event_bridge` streamlines this process. By annotating your event `enum` with a
custom procedural macro, you automatically get an async event handler method
that matches each variant and calls the appropriately named method on your
specified API trait.

## How It Works

When you apply the `#[derive(EventBridge)]` macro to your event enum,
it generates a `forward_to` method that:

- Consumes your event (the enum variant).
- Matches the event variant to the corresponding async trait method based on variant naming.
- Calls the trait method with the extracted fields from the enum variant.
- Returns a `Result<(), ErrorType>` to handle any errors that may occur during the method call.

This significantly reduces the boilerplate involved in writing `match` arms and
manually forwarding arguments for each new event added to your application.

## Installation

Add `event_bridge` and `async-trait` to your `Cargo.toml`:

```toml
[dependencies]
event_bridge = "0.1.0"
async-trait = "0.1"
```

## Quick Start

```rust
use async_trait::async_trait;
use event_bridge::EventBridge;
use std::sync::Arc;
use tokio::sync::Mutex;

// Define a custom error type for demonstration
type MyErrorType = String;

// Define an event enum and associate it with a trait and error type
#[derive(EventBridge)]
#[forward_to_trait(MyApiTrait)]
#[trait_returned_error(MyErrorType)]
pub enum Event {
  SetValue(i32),
  SetName(String),
  Initialize,
}

// Define the API trait that corresponds to the event variants
#[async_trait]
pub trait MyApiTrait {
  type EventType;
  async fn set_value(&mut self, value: i32) -> Result<(), MyErrorType>;
  async fn set_name(&mut self, name: String) -> Result<(), MyErrorType>;
  async fn initialize(&mut self) -> Result<(), MyErrorType>;
}

// Example usage
#[tokio::main]
async fn main() {
  // Assume `MyApiImpl` implements `MyApiTrait`
  let mut api_impl = MyApiImpl::new();

  // Example event to dispatch to the API implementation
  let some_event = Event::SetValue(42);

  // Dispatching an event becomes straightforward:
  let result = some_event.forward_to(&mut api_impl).await;
}
```

## Configuration

The macro supports two attributes on the enum:

- `#[forward_to_trait(TraitName)]` (required)
  
  Specifies the trait that defines the async methods corresponding to each enum
  variant.

- `#[trait_returned_error(ErrorType)]` (optional)
  
  Specifies the error type returned by the generated event handler. If omitted,
  the handler defaults to `Result<(), ()>`.

## Supported Variants

The macro handles various enum variants:

```rust
#[derive(EventBridge)]
#[forward_to_trait(Handler)]
enum Event {
  NoArgs,                      // Unit variant
  SingleArg(String),           // Single argument
  MultipleArgs(i32, String),   // Multiple arguments
  NamedFields { id: i32 },     // Named fields
}
```

Each variant must correspond to a method in the specified trait. The method name
is derived from the variant name converted to `snake_case`, and the method
arguments match the variant’s fields in order and type.

## Error Handling

If you specify `#[trait_returned_error(MyError)]`, the generated event handler
returns `Result<(), MyError>`. Ensure that your trait methods also return
`Result<(), MyError>` to maintain consistency.

## License

This project is licensed under the MIT license.

## Contributing

Contributions, issues, and pull requests are welcome. Feel free to open a
discussion on GitHub if you have any questions or suggestions.