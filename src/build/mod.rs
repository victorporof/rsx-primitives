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

pub mod traits;

// Bincode

#[cfg(feature = "bincode-display-list")]
#[path = "bincode/display_list_builder.rs"]
mod bincode_display_list_builder;

#[cfg(feature = "bincode-display-list")]
#[path = "bincode/display_list_differs.rs"]
mod bincode_display_list_differs;

#[cfg(feature = "bincode-display-list")]
#[path = "bincode/types.rs"]
mod bincode_types;

#[cfg(feature = "bincode-display-list")]
#[path = "bincode/export.rs"]
mod bincode_export;

#[cfg(feature = "bincode-display-list")]
#[path = "bincode/util.rs"]
mod bincode_util;

// Json

#[cfg(feature = "json-display-list")]
#[path = "json/display_list_builder.rs"]
mod json_display_list_builder;

#[cfg(feature = "json-display-list")]
#[path = "json/display_list_differs.rs"]
mod json_display_list_differs;

#[cfg(feature = "json-display-list")]
#[path = "json/types.rs"]
mod json_types;

#[cfg(feature = "json-display-list")]
#[path = "json/export.rs"]
mod json_export;

#[cfg(feature = "json-display-list")]
#[path = "json/util.rs"]
mod json_util;

// WebRender

#[cfg(feature = "webrender-display-list")]
#[path = "webrender/display_list_builder.rs"]
mod webrender_display_list_builder;

#[cfg(feature = "webrender-display-list")]
#[path = "webrender/display_list_differs.rs"]
mod webrender_display_list_differs;

#[cfg(feature = "webrender-display-list")]
#[path = "webrender/types.rs"]
mod webrender_types;

#[cfg(feature = "webrender-display-list")]
#[path = "webrender/util.rs"]
mod webrender_util;

// Pub

pub mod util {
    #[cfg(feature = "json-display-list")]
    pub use super::json_util::*;
    #[cfg(feature = "webrender-display-list")]
    pub use super::webrender_util::*;
}

pub mod types {
    #[cfg(feature = "bincode-display-list")]
    pub use super::bincode_display_list_builder::*;
    #[cfg(feature = "bincode-display-list")]
    pub use super::bincode_types::*;
    #[cfg(feature = "json-display-list")]
    pub use super::json_display_list_builder::*;
    #[cfg(feature = "json-display-list")]
    pub use super::json_types::*;
    #[cfg(feature = "webrender-display-list")]
    pub use super::webrender_display_list_builder::*;
    #[cfg(feature = "webrender-display-list")]
    pub use super::webrender_types::*;
}
