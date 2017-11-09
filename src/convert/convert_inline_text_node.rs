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

use rsx_dom::types::DOMText;
use rsx_layout::types::LayoutBoundingClientRect;
use rsx_shared::traits::{TDOMNode, TInheritedStyles};
use rsx_stylesheet::types::Color;

use prelude::{DOMNodeRef, ShapedText};
use traits::TDisplayListBuilder;

pub fn push_inline_text_node<T>(display_list_builder: &mut T, bounding_client_rect: LayoutBoundingClientRect, node_ref: &DOMNodeRef)
where
    T: TDisplayListBuilder<
        // General
        ClientRect = LayoutBoundingClientRect,
        // Text
        TextColor = Color,
        TextRun = ShapedText,
        // Debug
        DebugTextContent = DOMText
    >
{
    let computed_color = node_ref.computed_styles().color();
    let shaped_text = node_ref.get_shaped_text();
    let source_text = node_ref.data().text().unwrap();

    display_list_builder.push_text(
        bounding_client_rect,
        computed_color,
        shaped_text,
        source_text
    );
}
