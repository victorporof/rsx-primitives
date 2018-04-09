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

pub use self::ResourceKeysAPI as WebRenderImageKeysAPI;
pub use self::api::ImageKey as WebRenderImageKey;

// Images aliases

pub use self::WebRenderImageKey as ImageKey;
pub use self::WebRenderImageKeysAPI as ImageKeysAPI;

// Fonts

pub use self::ResourceKeysAPI as WebRenderFontKeysAPI;
pub use self::api::FontInstanceKey as WebRenderFontInstanceKey;
pub use self::api::FontKey as WebRenderFontKey;

// Fonts aliases

pub use self::WebRenderFontInstanceKey as FontInstanceKey;
pub use self::WebRenderFontKey as FontKey;
pub use self::WebRenderFontKeysAPI as FontKeysAPI;

// Glyphs

pub use self::api::GlyphInstance as WebRenderGlyphInstance;

// Glyphs aliases

pub use self::WebRenderGlyphInstance as GlyphInstance;

// Updates

pub use self::api::ResourceUpdates as WebRenderResourceUpdates;

// Updates aliases

pub use self::WebRenderResourceUpdates as ResourceUpdates;

// List

pub use self::api::BuiltDisplayList as WebRenderBuiltDisplayList;
pub use self::api::DisplayListBuilder as WebRenderDisplayListBuilder;

// List aliases

pub use self::WebRenderBuiltDisplayList as BuiltDisplayList;
pub use self::WebRenderDisplayListBuilder as DisplayListBuilder;

// Keys

pub struct WebRenderResourceKeysAPI {
    #[cfg(not(feature = "dummy-api-mode"))]
    pub(crate) api: Rc<api::RenderApi>,
    #[cfg(not(feature = "dummy-api-mode"))]
    pub(crate) updates: api::ResourceUpdates
}

// Keys aliases

pub use self::WebRenderResourceKeysAPI as ResourceKeysAPI;
