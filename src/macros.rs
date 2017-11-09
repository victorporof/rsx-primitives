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

pub use rsx_stylesheet::types::Color;

#[cfg(feature = "display-list-debug-mode")]
pub const DEBUG_COLOR: Color = Color {
    red: 128,
    green: 0,
    blue: 0,
    alpha: 128
};

#[cfg(not(feature = "display-list-optimize-mode"))]
macro_rules! return_if_nil {
    ( $( $tt:tt )* ) => {};
}

#[cfg(feature = "display-list-optimize-mode")]
macro_rules! return_if_nil {
    (@rect: $background_color:expr) => {
        if $background_color.alpha == 0 {
            return;
        }
    };
    (@border: $widths:expr, $colors:expr, $styles:expr) => {
        if $widths[0] == 0 && $widths[1] == 0 && $widths[2] == 0 && $widths[3] == 0 {
            return;
        }
        if $colors[0].alpha == 0 && $colors[1].alpha == 0 && $colors[2].alpha == 0 && $colors[3].alpha == 0 {
            return;
        }
        if $styles[0] == BorderStyle::None && $styles[1] == BorderStyle::None && $styles[2] == BorderStyle::None && $styles[3] == BorderStyle::None {
            return;
        }
    };
    (@image: $image_key:expr, $image_src:expr) => {
        if $image_key.is_none() {
            // println!("Warning: Image data missing for \"{}\"", $image_src.as_ref());
            return;
        }
    };
    (@text: $font_instance_key:expr, $text_color:expr, $text_glyphs:expr, $text_content:expr) => {
        if $font_instance_key.is_none() {
            // println!("Warning: Font data missing for \"{}\"", $text_content.as_ref());
            return;
        }
    };
}

#[cfg(not(feature = "display-list-debug-mode"))]
macro_rules! debug_item {
    ( $( $tt:tt )* ) => {};
}

#[cfg(feature = "display-list-debug-mode")]
macro_rules! debug_item {
    (@bounds: $self:expr, $bounding_client_rect:expr) => {
        TDisplayListBuilder::push_rect($self, $bounding_client_rect, ::macros::DEBUG_COLOR)
    };
}
