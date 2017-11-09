/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(match_ref_pats))]
#![feature(never_type)]
#![feature(pointer_methods)]
#![feature(macro_reexport)]

#[macro_reexport(fragment)]
pub extern crate rsx_dom;
pub extern crate rsx_event_manager;
pub extern crate rsx_layout;
pub extern crate rsx_resources;
pub extern crate rsx_shared;
pub extern crate rsx_stylesheet;

#[cfg(feature = "webrender-display-list")]
pub extern crate app_units;
#[cfg(feature = "webrender-display-list")]
pub extern crate euclid;
#[cfg(feature = "webrender-display-list")]
pub extern crate webrender;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate smallvec;

#[cfg(feature = "dummy-api-mode")]
extern crate rand;

#[macro_use]
mod macros;
mod convert;

pub mod debug;
pub mod build;
pub mod compare;
pub mod prelude;
pub mod types;

pub mod traits {
    pub use build::traits::*;
}
