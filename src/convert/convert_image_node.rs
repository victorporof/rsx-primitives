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
use rsx_dom::util::find_src;
use rsx_layout::types::LayoutBoundingClientRect;
use rsx_shared::traits::TDOMNode;
use rsx_stylesheet::types::{BorderStyle, Color};

use convert::push_view_dom_node;
use prelude::{DOMNodeRef, MeasuredImage};
use traits::TDisplayListBuilder;

pub fn push_image_dom_node<T>(display_list_builder: &mut T, bounding_client_rect: LayoutBoundingClientRect, node_ref: &DOMNodeRef)
where
    T: TDisplayListBuilder<
        // General
        ClientRect = LayoutBoundingClientRect,
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle,
        // Image
        MeasuredImage = MeasuredImage,
        // Debug
        DebugImageSrc = DOMText
    >
{
    let measured_image = node_ref.get_measured_image();
    let image_src = find_src(node_ref.data().get_attributes());

    push_view_dom_node(display_list_builder, bounding_client_rect, node_ref);

    display_list_builder.push_image(
        bounding_client_rect,
        measured_image,
        image_src.unwrap_or(&DOMText::from(""))
    );
}
