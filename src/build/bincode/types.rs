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

pub use rsx_resources::updates::types::DefaultImageKey as BincodeImageKey;
pub use rsx_resources::updates::types::DefaultImageKeysAPI as BincodeImageKeysAPI;

// Images aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeImageKey as ImageKey;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeImageKeysAPI as ImageKeysAPI;

// Fonts

pub use rsx_resources::updates::types::DefaultFontInstanceKey as BincodeFontInstanceKey;
pub use rsx_resources::updates::types::DefaultFontKey as BincodeFontKey;
pub use rsx_resources::updates::types::DefaultFontKeysAPI as BincodeFontKeysAPI;

// Fonts aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeFontInstanceKey as FontInstanceKey;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeFontKey as FontKey;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeFontKeysAPI as FontKeysAPI;

// Glyphs

pub use rsx_resources::updates::types::DefaultGlyphInstance as BincodeGlyphInstance;

// Glyphs aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeGlyphInstance as GlyphInstance;

// Updates

pub type BincodeResourceUpdates = TResourceUpdates<BincodeImageKey, BincodeFontKey, BincodeFontInstanceKey>;

// Updates aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeResourceUpdates as ResourceUpdates;

// List

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BincodeBuiltDisplayList(pub(crate) Vec<u8>);

// List aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeBuiltDisplayList as BuiltDisplayList;

// Builder

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BincodeDisplayListBuilder(pub(crate) Vec<u8>);

// Builder aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeDisplayListBuilder as DisplayListBuilder;

// Items

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum BincodeSpecificDisplayItem {
    Rect(RectDisplayItem),
    Border(BorderDisplayItem),
    Image(ImageDisplayItem),
    Text(TextDisplayItem)
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BincodeRectDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: RectDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BincodeRectDisplayProps {
    pub color: Color
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BincodeBorderDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: BorderDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BincodeBorderDisplayProps {
    pub widths: [u32; 4],
    pub colors: [Color; 4],
    pub styles: [BorderStyle; 4]
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BincodeImageDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: ImageDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BincodeImageDisplayProps {
    pub image_src: DOMText,
    pub measured_image: MeasuredImage
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BincodeTextDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: TextDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BincodeTextDisplayProps {
    pub color: Color,
    pub source_text: SmallVec<[DOMText; 1]>,
    pub shaped_text: SmallVec<[ShapedText; 1]>
}

// Items aliases

#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeBorderDisplayItem as BorderDisplayItem;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeBorderDisplayProps as BorderDisplayProps;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeImageDisplayItem as ImageDisplayItem;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeImageDisplayProps as ImageDisplayProps;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeRectDisplayItem as RectDisplayItem;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeRectDisplayProps as RectDisplayProps;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeSpecificDisplayItem as SpecificDisplayItem;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeTextDisplayItem as TextDisplayItem;
#[cfg(feature = "bincode-display-list-aliases")]
pub use self::BincodeTextDisplayProps as TextDisplayProps;
