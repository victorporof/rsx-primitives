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

use rsx_layout::types::{LayoutBoundingClientRect, LayoutClientSize};
use rsx_stylesheet::types::{BorderStyle, Color};
use webrender::api;

pub fn make_primitive_info(bounding_client_rect: LayoutBoundingClientRect) -> api::LayoutPrimitiveInfo {
    api::PrimitiveInfo::new(make_layer_rect(bounding_client_rect))
}

pub fn make_layer_rect(bounding_client_rect: LayoutBoundingClientRect) -> api::LayoutRect {
    api::LayoutRect::new(
        api::LayoutPoint::new(
            bounding_client_rect.position.left as f32,
            bounding_client_rect.position.top as f32
        ),
        api::LayoutSize::new(
            bounding_client_rect.size.width as f32,
            bounding_client_rect.size.height as f32
        )
    )
}

pub fn make_color_f(color: Color) -> api::ColorF {
    api::ColorF::new(
        f32::from(color.red) / 255.0,
        f32::from(color.green) / 255.0,
        f32::from(color.blue) / 255.0,
        f32::from(color.alpha) / 255.0
    )
}

pub fn make_glyph_options() -> Option<api::GlyphOptions> {
    Some(api::GlyphOptions {
        render_mode: api::FontRenderMode::Alpha
    })
}

pub fn make_image_stretch_size(bounding_client_size: LayoutClientSize) -> api::LayoutSize {
    api::LayoutSize::new(
        bounding_client_size.width as f32,
        bounding_client_size.height as f32
    )
}

pub fn make_image_tile_spacing() -> api::LayoutSize {
    api::LayoutSize::zero()
}

pub fn make_image_rendering_style() -> api::ImageRendering {
    api::ImageRendering::Auto
}

pub fn make_border_widths(border_widths: [u32; 4]) -> api::BorderWidths {
    api::BorderWidths {
        top: border_widths[0] as f32,
        right: border_widths[1] as f32,
        bottom: border_widths[2] as f32,
        left: border_widths[3] as f32
    }
}

pub fn make_border_style(border_style: BorderStyle) -> api::BorderStyle {
    match border_style {
        BorderStyle::None => api::BorderStyle::None,
        BorderStyle::Solid => api::BorderStyle::Solid,
        BorderStyle::Double => api::BorderStyle::Double,
        BorderStyle::Dotted => api::BorderStyle::Dotted,
        BorderStyle::Dashed => api::BorderStyle::Dashed,
        BorderStyle::Hidden => api::BorderStyle::Hidden,
        BorderStyle::Groove => api::BorderStyle::Groove,
        BorderStyle::Ridge => api::BorderStyle::Ridge,
        BorderStyle::Inset => api::BorderStyle::Inset,
        BorderStyle::Outset => api::BorderStyle::Outset
    }
}

pub fn make_border_details(border_colors: [Color; 4], border_styles: [BorderStyle; 4]) -> api::BorderDetails {
    api::BorderDetails::Normal(api::NormalBorder {
        top: api::BorderSide {
            color: make_color_f(border_colors[0]),
            style: make_border_style(border_styles[0])
        },
        right: api::BorderSide {
            color: make_color_f(border_colors[1]),
            style: make_border_style(border_styles[1])
        },
        bottom: api::BorderSide {
            color: make_color_f(border_colors[2]),
            style: make_border_style(border_styles[2])
        },
        left: api::BorderSide {
            color: make_color_f(border_colors[3]),
            style: make_border_style(border_styles[3])
        },
        radius: api::BorderRadius::uniform(0.0)
    })
}
