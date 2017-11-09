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

pub use rsx_resources::updates::types::DefaultImageKey as ImageKey;
pub use rsx_resources::updates::types::DefaultImageKeysAPI as ImageKeysAPI;

// Fonts

pub use rsx_resources::updates::types::DefaultFontInstanceKey as FontInstanceKey;
pub use rsx_resources::updates::types::DefaultFontKey as FontKey;
pub use rsx_resources::updates::types::DefaultFontKeysAPI as FontKeysAPI;

// Glyphs

pub use rsx_resources::updates::types::DefaultGlyphInstance as GlyphInstance;

// Updates

pub type ResourceUpdates = TResourceUpdates<ImageKey, FontKey, FontInstanceKey>;

// List

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuiltDisplayList(pub(crate) String);

// Builder

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayListBuilder(pub(crate) Vec<SpecificDisplayItem>);

// Items

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecificDisplayItem {
    Rect(RectDisplayItem),
    Border(BorderDisplayItem),
    Image(ImageDisplayItem),
    Text(TextDisplayItem)
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct RectDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: RectDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct RectDisplayProps {
    pub color: Color
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BorderDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: BorderDisplayProps
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BorderDisplayProps {
    pub widths: [u32; 4],
    pub colors: [Color; 4],
    pub styles: [BorderStyle; 4]
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: ImageDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageDisplayProps {
    pub image_src: DOMText,
    pub measured_image: MeasuredImage
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TextDisplayItem {
    pub bounds: LayoutBoundingClientRect,
    pub display: TextDisplayProps
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TextDisplayProps {
    pub color: Color,
    pub source_text: SmallVec<[DOMText; 1]>,
    pub shaped_text: SmallVec<[ShapedText; 1]>
}
