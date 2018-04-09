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

use prelude::{MeasuredImage, ShapedText};
use rsx_dom::types::DOMText;
use rsx_layout::types::LayoutBoundingClientRect;
use rsx_resources::updates::types::ResourceUpdates as TResourceUpdates;
use rsx_stylesheet::types::{BorderStyle, Color};
use smallvec::SmallVec;

// Images

pub use rsx_resources::updates::types::DefaultImageKey as JsonImageKey;
pub use rsx_resources::updates::types::DefaultImageKeysAPI as JsonImageKeysAPI;

// Images aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonImageKey as ImageKey;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonImageKeysAPI as ImageKeysAPI;

// Fonts

pub use rsx_resources::updates::types::DefaultFontInstanceKey as JsonFontInstanceKey;
pub use rsx_resources::updates::types::DefaultFontKey as JsonFontKey;
pub use rsx_resources::updates::types::DefaultFontKeysAPI as JsonFontKeysAPI;

// Fonts aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonFontInstanceKey as FontInstanceKey;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonFontKey as FontKey;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonFontKeysAPI as FontKeysAPI;

// Glyphs

pub use rsx_resources::updates::types::DefaultGlyphInstance as JsonGlyphInstance;

// Glyphs aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonGlyphInstance as GlyphInstance;

// Updates

pub type JsonResourceUpdates = TResourceUpdates<JsonImageKey, JsonFontKey, JsonFontInstanceKey>;

// Updates aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonResourceUpdates as ResourceUpdates;

// List

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonBuiltDisplayList(pub(crate) String);

// List aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonBuiltDisplayList as BuiltDisplayList;

// Builder

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonDisplayListBuilder(pub(crate) Vec<JsonSpecificDisplayItem>);

// Builder aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonDisplayListBuilder as DisplayListBuilder;

// Items

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum JsonSpecificDisplayItem {
    Rect(RectDisplayItem),
    Border(BorderDisplayItem),
    Image(ImageDisplayItem),
    Text(TextDisplayItem)
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct JsonRectDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: RectDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct JsonRectDisplayProps {
    pub color: Color
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct JsonBorderDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: BorderDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct JsonBorderDisplayProps {
    pub widths: [u32; 4],
    pub colors: [Color; 4],
    pub styles: [BorderStyle; 4]
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JsonImageDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: ImageDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JsonImageDisplayProps {
    pub image_src: DOMText,
    pub measured_image: MeasuredImage
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JsonTextDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: TextDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JsonTextDisplayProps {
    pub color: Color,
    pub source_text: SmallVec<[DOMText; 1]>,
    pub shaped_text: SmallVec<[ShapedText; 1]>
}

// Items aliases

#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonBorderDisplayItem as BorderDisplayItem;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonBorderDisplayProps as BorderDisplayProps;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonImageDisplayItem as ImageDisplayItem;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonImageDisplayProps as ImageDisplayProps;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonRectDisplayItem as RectDisplayItem;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonRectDisplayProps as RectDisplayProps;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonSpecificDisplayItem as SpecificDisplayItem;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonTextDisplayItem as TextDisplayItem;
#[cfg(feature = "json-display-list-aliases")]
pub use self::JsonTextDisplayProps as TextDisplayProps;
