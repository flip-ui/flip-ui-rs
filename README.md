# flip-ui-rs

Rust integration for handling and compiling the UI built via the Website.

## Features

- **JSON Integration**: Easily import and manage your UI designs exported from the [Flip UI Builder](https://flip-ui.github.io/).
- **UI Rendering**: Efficiently render user interface components on Flipper Zero devices.
- **Interaction Handling**: Simplified event handling to manage user interactions within your applications.

## Important Note

Please note that the crate currently works with a different JSON format than the current website. I am working on a rework of the website!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flip_ui = { git = "https://github.com/flip-ui/flip-ui-rs" }
```

> An official rust crate release will come when the crate is in a good enough state!

## Example

```rust
#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;
use flip_ui::flipper_ui;
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
	name = "Flipper Zero Rust",
	app_version = 1,
	has_icon = false,
);

// Define the entry function
entry!(main);

// Getting UI data && events
flipper_ui! {
	App,
	"src/main.json",
	next => next,
	close => close,
	back => back,
}

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
	let mut app = App::create();

	app.show();

	0
}
```
