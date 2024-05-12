# flip-ui-rs

This is the rust integration for handling and compiling the built JSON File.

## Features

- **JSON Integration**: Easily import and manage your UI designs exported from the Flip UI Builder.
- **UI Rendering**: Efficiently render user interface components on Flipper Zero devices.
- **Interaction Handling**: Simplified event handling to manage user interactions within your applications.

## Important Note

Please note that the crate currently works with a different JSON format than the current website. I am working on a rework of the website!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flip_ui = { git = "https://github.com/flip-ui/flip-ui-rs" }
flip_ui_macro = { git = "https://github.com/flip-ui/flip-ui-rs", package = "flip-ui-macro" }
```

> An official rust crate release will come when the crate is in a good enough state!

## Example

```rust
#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;
use flip_ui_macro::flipper_ui;
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
	name = "Flipper Zero Rust",
	app_version = 1,
	has_icon = false,
);

// Define the entry function
entry!(main);

// Getting UI data && actions
flipper_ui! {
	App,
	"src/main.json",
	next => next,
	close => close,
}

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
	let mut app = App::create();

	app.show();

	0
}
```
