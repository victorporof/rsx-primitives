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

use build::types::TextDisplayItem;
use prelude::ShapedText;

// TODO: properly handle text direction and don't assume it's
// laid out horizontally and LTR.
pub fn are_sibling_text_display_items(target: &TextDisplayItem, next_bounding_client_rect: LayoutBoundingClientRect) -> bool {
    let &TextDisplayItem { ref bounds, .. } = target;

    if (bounds.position.top as f32 - next_bounding_client_rect.position.top as f32)
        .abs()
        .round() > 2.0
    {
        false
    } else if ((bounds.position.left + bounds.size.width) as f32 - next_bounding_client_rect.position.left as f32)
        .abs()
        .round() > 2.0
    {
        false
    } else {
        true
    }
}

// TODO: properly handle text direction and don't assume it's
// laid out horizontally and LTR.
pub fn extend_text_display_item_with(
    target: &mut TextDisplayItem,
    bounding_client_rect: LayoutBoundingClientRect,
    text_run: &ShapedText,
    text_content: &DOMText
) {
    let &mut TextDisplayItem {
        ref mut bounds,
        ref mut display
    } = target;

    bounds.size.width += bounding_client_rect.size.width - 1;
    bounds.size.height = u32::max(bounds.size.height, bounding_client_rect.size.height);

    display.source_text.push(DOMText::clone(text_content));
    display.shaped_text.push(ShapedText::clone(text_run));
}
