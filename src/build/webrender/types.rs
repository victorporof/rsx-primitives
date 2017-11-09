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

#[cfg(not(feature = "dummy-api-mode"))]
use std::rc::Rc;

use webrender::api;

// Images

pub use self::ResourceKeysAPI as ImageKeysAPI;
pub use self::api::ImageKey;

// Fonts

pub use self::ResourceKeysAPI as FontKeysAPI;
pub use self::api::FontInstanceKey;
pub use self::api::FontKey;

// Glyphs

pub use self::api::GlyphInstance;

// Updates

pub use self::api::ResourceUpdates;

// List

pub use self::api::BuiltDisplayList;
pub use self::api::DisplayListBuilder;

// Keys

pub struct ResourceKeysAPI {
    #[cfg(not(feature = "dummy-api-mode"))]
    pub(crate) api: Rc<api::RenderApi>,
    #[cfg(not(feature = "dummy-api-mode"))]
    pub(crate) updates: api::ResourceUpdates
}
