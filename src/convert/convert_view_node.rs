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

use rsx_layout::types::LayoutBoundingClientRect;
use rsx_shared::traits::{TComputedStyles, TDOMNode};
use rsx_stylesheet::types::{BorderStyle, Color};

use prelude::DOMNodeRef;
use traits::TDisplayListBuilder;

#[cfg_attr(feature = "cargo-clippy", allow(single_match))]
pub fn push_view_dom_node<T>(display_list_builder: &mut T, bounding_client_rect: LayoutBoundingClientRect, node_ref: &DOMNodeRef)
where
    T: TDisplayListBuilder<
        // General
        ClientRect = LayoutBoundingClientRect,
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle
    >
{
    let computed_styles = node_ref.computed_styles();
    let background_color = computed_styles.background_color();
    let border_widths = [
        computed_styles.border_top_width(),
        computed_styles.border_right_width(),
        computed_styles.border_bottom_width(),
        computed_styles.border_left_width(),
    ];
    let border_colors = [
        computed_styles.border_top_color(),
        computed_styles.border_right_color(),
        computed_styles.border_bottom_color(),
        computed_styles.border_left_color(),
    ];
    let border_styles = [
        computed_styles.border_top_style(),
        computed_styles.border_right_style(),
        computed_styles.border_bottom_style(),
        computed_styles.border_left_style(),
    ];

    display_list_builder.push_rect(bounding_client_rect, background_color);

    display_list_builder.push_border(
        bounding_client_rect,
        border_widths,
        border_colors,
        border_styles
    );
}
