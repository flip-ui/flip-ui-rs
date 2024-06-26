# flip-ui

[![crates.io](https://img.shields.io/crates/v/flip-ui.svg)](https://crates.io/crates/flip-ui)
[![crates.io](https://img.shields.io/crates/d/flip-ui.svg)](https://crates.io/crates/flip-ui)
[![docs.rs](https://docs.rs/flip-ui/badge.svg)](https://docs.rs/flip-ui)

Rust integration for handling and compiling the UI built via the Website.

## Features

- **JSON Integration**: Easily import and manage your UI designs exported from the [Flip UI Builder](https://flip.nwrenger.dev/).
- **UI Rendering**: Efficiently render user interface components on Flipper Zero devices.
- **Interaction Handling**: Simplified event handling to manage user interactions within your applications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flip_ui = "0.1.3"
```

## Example

```rust
#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;
use flip_ui::flip_ui;
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
flip_ui! {
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
