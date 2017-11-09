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

use serde_json;

use rsx_dom::types::DOMText;
use rsx_layout::types::LayoutBoundingClientRect;
use rsx_stylesheet::types::{BorderStyle, Color};
use smallvec::SmallVec;

use build::json_util::{are_sibling_text_display_items, extend_text_display_item_with};
use build::types::{
    BorderDisplayItem,
    BorderDisplayProps,
    BuiltDisplayList,
    DisplayListBuilder,
    ImageDisplayItem,
    ImageDisplayProps,
    RectDisplayItem,
    RectDisplayProps,
    SpecificDisplayItem,
    TextDisplayItem,
    TextDisplayProps
};
use prelude::{MeasuredImage, ShapedText};
use traits::TDisplayListBuilder;

impl TDisplayListBuilder for DisplayListBuilder {
    // Serialize

    type Serialized = BuiltDisplayList;

    #[cfg(not(feature = "pretty-json-mode"))]
    fn serialize(self) -> Self::Serialized {
        BuiltDisplayList(serde_json::to_string(&self.0).unwrap_or_else(|_| "".to_string()))
    }

    #[cfg(feature = "pretty-json-mode")]
    fn serialize(self) -> Self::Serialized {
        BuiltDisplayList(serde_json::to_string_pretty(&self.0).unwrap_or_else(|_| "".to_string()))
    }

    // General

    type ClientRect = LayoutBoundingClientRect;
    type TransformMatrix = !;
    type PerspectiveMatrix = !;

    fn push_stacking_context(&mut self, _: Option<Self::TransformMatrix>, _: Option<Self::PerspectiveMatrix>) {
        unreachable!()
    }

    fn pop_stacking_context(&mut self) {
        unreachable!()
    }

    // Memory

    type DisplayListItem = SpecificDisplayItem;

    fn get_first(&mut self) -> Option<&mut Self::DisplayListItem> {
        self.0.first_mut()
    }

    fn get_previous(&mut self) -> Option<&mut Self::DisplayListItem> {
        self.0.last_mut()
    }

    // Rect

    type RectColor = Color;

    fn push_rect(&mut self, bounding_client_rect: Self::ClientRect, background_color: Self::RectColor) {
        return_if_nil!(@rect: background_color);

        self.0.push(SpecificDisplayItem::Rect(RectDisplayItem {
            bounds: bounding_client_rect,
            display: RectDisplayProps {
                color: background_color
            }
        }));
    }

    // Border

    type BorderSize = u32;
    type BorderColor = Color;
    type BorderStyle = BorderStyle;

    fn push_border(
        &mut self,
        bounding_client_rect: Self::ClientRect,
        border_widths: [Self::BorderSize; 4],
        border_colors: [Self::BorderColor; 4],
        border_styles: [Self::BorderStyle; 4]
    ) {
        return_if_nil!(@border: border_widths, border_colors, border_styles);
        debug_item!(@bounds: self, bounding_client_rect);

        self.0.push(SpecificDisplayItem::Border(BorderDisplayItem {
            bounds: bounding_client_rect,
            display: BorderDisplayProps {
                widths: border_widths,
                colors: border_colors,
                styles: border_styles
            }
        }));
    }

    // Image

    type MeasuredImage = MeasuredImage;

    fn push_image(
        &mut self,
        bounding_client_rect: Self::ClientRect,
        measured_image: &Self::MeasuredImage,
        image_src: &Self::DebugImageSrc
    ) {
        let image_key = measured_image.image_key();

        return_if_nil!(@image: image_key, image_src);
        debug_item!(@bounds: self, bounding_client_rect);

        if let Some(_) = image_key {
            self.0.push(SpecificDisplayItem::Image(ImageDisplayItem {
                bounds: bounding_client_rect,
                display: ImageDisplayProps {
                    measured_image: MeasuredImage::clone(measured_image),
                    image_src: DOMText::clone(image_src)
                }
            }));
        } else {
            println!("Warning: Image data missing for \"{}\"", image_src.as_ref());
        }
    }

    // Text

    type TextColor = Color;
    type TextRun = ShapedText;

    fn push_text(
        &mut self,
        bounding_client_rect: Self::ClientRect,
        text_color: Self::TextColor,
        text_run: &Self::TextRun,
        text_content: &Self::DebugTextContent
    ) {
        let font_instance_key = text_run.font_instance_key();
        let _text_glyphs = text_run.glyphs();

        if let Some(&mut SpecificDisplayItem::Text(ref mut previous)) = self.get_previous() {
            if are_sibling_text_display_items(previous, bounding_client_rect) {
                extend_text_display_item_with(previous, bounding_client_rect, text_run, text_content);
                return;
            }
        }

        return_if_nil!(@text: font_instance_key, text_color, _text_glyphs, text_content);
        debug_item!(@bounds: self, bounding_client_rect);

        if let Some(_) = font_instance_key {
            self.0.push(SpecificDisplayItem::Text(TextDisplayItem {
                bounds: bounding_client_rect,
                display: TextDisplayProps {
                    color: text_color,
                    shaped_text: SmallVec::from_buf([ShapedText::clone(text_run)]),
                    source_text: SmallVec::from_buf([DOMText::clone(text_content)])
                }
            }));
        } else {
            println!("Warning: Font missing for \"{}\"", text_content.as_ref());
        }
    }

    // Debug

    type DebugTextContent = DOMText;
    type DebugImageSrc = DOMText;
}
