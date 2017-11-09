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

use rsx_dom::types::{
    DOMArenaRef as TDOMNodeRef,
    DOMArenaRefMut as TDOMNodeRefMut,
    DOMAttribute as TDOMAttribute,
    DOMAttributeValue as TDOMAttributeValue,
    DOMAttributes as TDOMAttributes,
    DOMChildren as TDOMChildren,
    DOMData as TDOMData,
    DOMNode as TDOMNode,
    DOMNodeId as TDOMNodeId,
    DOMText,
    DOMTree as TDOMTree
};
use rsx_event_manager::types::{Event, EventManager as TEventManager};
use rsx_layout::types::{LayoutNode as TLayoutNode, MeasuredImage as TMeasuredImage, ShapedText as TShapedText};
use rsx_resources::files::types::FileCache as TFileCache;
use rsx_resources::fonts::types::{FontCache as TFontCache, GlyphStore as TGlyphStore};
use rsx_resources::images::types::{ImageCache as TImageCache, ImageDimensionsInfo as TImageDimensionsInfo};
use rsx_resources::types::ResourceGroup as TResourceGroup;
use rsx_stylesheet::types::{ComputedStyles, StyleDeclarations};

use build::types::{FontInstanceKey, FontKey, FontKeysAPI, GlyphInstance, ImageKey, ImageKeysAPI};

// Resources

pub type FileCache = TFileCache;
pub type ImageCache = TImageCache<ImageKeysAPI>;
pub type FontCache = TFontCache<FontKeysAPI>;
pub type ResourceGroup = TResourceGroup<ImageKeysAPI, FontKeysAPI>;

// Layout Nodes

pub type LayoutNode = TLayoutNode<StyleDeclarations, ComputedStyles, TResourceGroup<ImageKeysAPI, FontKeysAPI>, DOMText>;

// Layout Images

pub type ImageDimensionsInfo = TImageDimensionsInfo<ImageKey>;
pub type MeasuredImage = TMeasuredImage<ImageDimensionsInfo>;

// Layout Text

pub type GlyphStore = TGlyphStore<FontKey, FontInstanceKey, GlyphInstance>;
pub type ShapedText = TShapedText<GlyphStore>;

// DOM Tree

pub type DOMNodeId = TDOMNodeId<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMNodeRef<'a> = TDOMNodeRef<'a, Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMNodeRefMut<'a> = TDOMNodeRefMut<'a, Event, StyleDeclarations, ComputedStyles, LayoutNode>;

// DOM Nodes

pub type DOMTree = TDOMTree<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMNode = TDOMNode<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMData = TDOMData<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMChildren = TDOMChildren<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMAttribute = TDOMAttribute<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMAttributes = TDOMAttributes<Event, StyleDeclarations, ComputedStyles, LayoutNode>;
pub type DOMAttributeValue = TDOMAttributeValue<Event, StyleDeclarations, ComputedStyles, LayoutNode>;

// Event Manager

pub type EventManager = TEventManager<DOMNode>;
