**Under heavy research and development, please don't use this yet!**

# rsx-primitives
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Build Status](https://travis-ci.org/victorporof/rsx-primitives.svg?branch=master)](https://travis-ci.org/victorporof/rsx-primitives)

Basic component primitives necessary to render any [RSX code](https://github.com/victorporof/rsx), inspired by [react-primitives](https://github.com/lelandrichardson/react-primitives). Turns a [RSX LayoutTree](https://github.com/victorporof/rsx-layout) into a [Servo WebRender DisplayList](http://doc.servo.org/webrender/api/struct.BuiltDisplayList.html) or a simpler JSON-based DisplayList backed by [Serde](https://github.com/serde-rs/json).

## Purpose
Display lists are a final abstraction layer for all the data structures generated when writing RSX code in a project, before being sent to a renderer.

## How to use
[Documentation](https://victorporof.github.io/rsx-primitives)

This crate concerns itself strictly generating a display list. If you're just looking to write RSX in your project, take a look at the [RSX compiler plugin](https://github.com/victorporof/rsx-compiler-plugin) instead.

Otherwise, add this to your `Cargo.toml` file:

```toml
[dependencies]
rsx = { git = "https://github.com/victorporof/rsx.git" }
rsx-primitives = { git = "https://github.com/victorporof/rsx-primitives.git" }
```

By default, this crate builds Servo Webrender specific display lists. To opt into other kinds of data structures, use the `json-display-list` or `webrender-display-list` feature gates.

For example, to only build JSON-based display lists:

```toml
[dependencies]
rsx-primitives = { git = "https://github.com/victorporof/rsx-primitives.git", default-features = false, features = ["json-display-list"] }
```

Then, simply import the library into your code to generate a Servo WebRender-powered `webrender::api::BuiltDisplayList` (wrapped as `rsx_primitives::DisplayList`) from RSX layout trees, or a `String` JSON representing a display list using Serde. See the [WebRender's documentation](https://github.com/servo/webrender/wiki) for a complete list of all the data structures used in WebRender, and the [Serde JSON documentation](https://github.com/serde-rs/json#operating-on-untyped-json-values) for the data structures used for representing JSON.

```rust
#![feature(proc_macro)]

extern crate rsx;
extern crate rsx_primitives;

use rsx::{rsx, css};
use rsx_primitives::types::DisplayList;
use rsx_primitives::rsx_layout::types::{LayoutDirection, LayoutTree, LayoutRectTree};
use rsx_primitives::rsx_resources::files::types::*;
use rsx_primitives::rsx_resources::fonts::types::*;
use rsx_primitives::rsx_stylesheet::types::*;
use rsx_primitives::rsx_dom::types::*;

let mut stylesheet: Stylesheet = css! { ... };
let node: Node = rsx! { ... };

// Create a layout tree with a DOM node as root.
let mut layout: LayoutTree = LayoutTree::from(&node);

// Calculate bounding client rects for subtree starting with a DOM node.
let width = 1024.0;
let height = 768.0;
layout.reflow_subtree(&node, width, height, LayoutDirection::LTR);

// Get a list of all bounding client rects for subtree starting with a DOM node.
let rects: Option<LayoutRectTree> = layout.get_bounding_client_rects_for_subtree(&node);

// Get a list of display instructions from a DOM node and computed bounding client rects.
let display_list: DisplayList = DisplayList::from(&rects.unwrap(), width, height);
```

*Note: `rsx-primitives` also re-exports the `rsx-dom` [crate](https://github.com/victorporof/rsx-dom), the `rsx-stylesheet` [crate](https://github.com/victorporof/rsx-stylesheet), the `rsx-layout` [crate](https://github.com/victorporof/rsx-layout) and the `rsx-resources` [crate](https://github.com/victorporof/rsx-resources).*
