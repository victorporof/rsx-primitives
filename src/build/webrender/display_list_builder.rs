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

#[cfg(not(feature = "dummy-api-mode"))]
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
#[cfg(not(feature = "dummy-api-mode"))]
use std::sync::Arc;

#[cfg(not(feature = "dummy-api-mode"))]
use app_units::Au;
use euclid::TypedTransform3D;
#[cfg(feature = "dummy-api-mode")]
use rand::{self, Rng};
use rsx_dom::types::DOMText;
use rsx_layout::types::LayoutBoundingClientRect;
#[cfg(not(feature = "dummy-api-mode"))]
use rsx_resources::images::types::ImagePixelFormat;
use rsx_shared::traits::{TFontKeysAPI, TImageKeysAPI};
use rsx_shared::types::{FontEncodedData, FontInstanceResourceData, FontResourceData, ImageEncodedData, ImageResourceData};
use rsx_stylesheet::types::{BorderStyle, Color};
use webrender::api;

use super::util::{
    make_border_details,
    make_border_widths,
    make_color_f,
    make_glyph_options,
    make_image_rendering_style,
    make_image_stretch_size,
    make_image_tile_spacing,
    make_primitive_info
};
use build::types::{
    BuiltDisplayList,
    DisplayListBuilder,
    FontInstanceKey,
    FontKey,
    GlyphInstance,
    ImageKey,
    ResourceKeysAPI,
    ResourceUpdates
};
use prelude::{MeasuredImage, ShapedText};
use traits::TDisplayListBuilder;

impl ResourceKeysAPI {
    #[cfg(feature = "dummy-api-mode")]
    pub fn dummy() -> Self {
        ResourceKeysAPI {}
    }
}

impl TImageKeysAPI for ResourceKeysAPI {
    type RootRendererAPI = Rc<api::RenderApi>;
    type ResourceUpdates = ResourceUpdates;
    type ImageKey = ImageKey;

    #[cfg(not(feature = "dummy-api-mode"))]
    fn new(api: Self::RootRendererAPI) -> Self {
        ResourceKeysAPI {
            api,
            updates: ResourceUpdates::new()
        }
    }

    #[cfg(feature = "dummy-api-mode")]
    fn new(_: Self::RootRendererAPI) -> Self {
        ResourceKeysAPI::dummy()
    }

    #[cfg(not(feature = "dummy-api-mode"))]
    fn add_image(&mut self, _: ImageEncodedData, resource: ImageResourceData) -> Self::ImageKey {
        let image_key = self.api.as_ref().generate_image_key();

        let format = match resource.format {
            ImagePixelFormat::Gray(8) => api::ImageFormat::A8,
            ImagePixelFormat::BGRA(8) => api::ImageFormat::BGRA8,
            _ => unimplemented!()
        };

        let descriptor = api::ImageDescriptor::new(resource.size.0, resource.size.1, format, false);
        let data = api::ImageData::new_shared(Arc::clone(resource.pixels));
        let tiling = None;

        self.updates.add_image(image_key, descriptor, data, tiling);

        image_key
    }

    #[cfg(feature = "dummy-api-mode")]
    fn add_image(&mut self, _: ImageEncodedData, _: ImageResourceData) -> Self::ImageKey {
        return ImageKey::new(api::IdNamespace(0), rand::thread_rng().gen::<u32>());
    }

    #[cfg(feature = "dummy-api-mode")]
    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        ResourceUpdates::new()
    }

    #[cfg(not(feature = "dummy-api-mode"))]
    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        mem::replace(&mut self.updates, ResourceUpdates::new())
    }
}

impl TFontKeysAPI for ResourceKeysAPI {
    type RootRendererAPI = Rc<api::RenderApi>;
    type ResourceUpdates = ResourceUpdates;
    type FontKey = FontKey;
    type FontInstanceKey = FontInstanceKey;
    type GlyphInstance = GlyphInstance;

    #[cfg(not(feature = "dummy-api-mode"))]
    fn new(api: Self::RootRendererAPI) -> Self {
        ResourceKeysAPI {
            api,
            updates: ResourceUpdates::new()
        }
    }

    #[cfg(feature = "dummy-api-mode")]
    fn new(_: Self::RootRendererAPI) -> Self {
        ResourceKeysAPI::dummy()
    }

    #[cfg(not(feature = "dummy-api-mode"))]
    fn add_font(&mut self, _: FontEncodedData, resource: FontResourceData) -> Self::FontKey {
        let font_key = self.api.as_ref().generate_font_key();

        let bytes = unsafe { (resource.bytes.deref() as *const Vec<u8>).read() }; // Fonts are never removed.
        let face_index = resource.face_index as u32;
        self.updates.add_raw_font(font_key, bytes, face_index);

        font_key
    }

    #[cfg(feature = "dummy-api-mode")]
    fn add_font(&mut self, _: FontEncodedData, _: FontResourceData) -> Self::FontKey {
        return FontKey::new(api::IdNamespace(0), rand::thread_rng().gen::<u32>());
    }

    #[cfg(not(feature = "dummy-api-mode"))]
    fn add_font_instance(&mut self, font_key: Self::FontKey, resource: FontInstanceResourceData) -> Self::FontInstanceKey {
        let font_instance_key = self.api.as_ref().generate_font_instance_key();

        let size = Au::from_f32_px(resource.size as f32);
        let options = None;
        let platform_options = None;
        let variations = vec![];

        self.updates.add_font_instance(
            font_instance_key,
            font_key,
            size,
            options,
            platform_options,
            variations
        );

        font_instance_key
    }

    #[cfg(feature = "dummy-api-mode")]
    fn add_font_instance(&mut self, _: Self::FontKey, _: FontInstanceResourceData) -> Self::FontInstanceKey {
        return FontInstanceKey::new(api::IdNamespace(0), rand::thread_rng().gen::<u32>());
    }

    #[cfg(feature = "dummy-api-mode")]
    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        ResourceUpdates::new()
    }

    #[cfg(not(feature = "dummy-api-mode"))]
    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        mem::replace(&mut self.updates, ResourceUpdates::new())
    }
}

impl TDisplayListBuilder for DisplayListBuilder {
    // Serialize

    type Serialized = BuiltDisplayList;

    fn serialize(self) -> Self::Serialized {
        let (.., serialized) = self.finalize();
        serialized
    }

    // General

    type ClientRect = LayoutBoundingClientRect;
    type TransformMatrix = TypedTransform3D<f32, api::LayerPixel, api::LayerPixel>;
    type PerspectiveMatrix = TypedTransform3D<f32, api::LayerPixel, api::LayerPixel>;

    fn push_stacking_context(&mut self, transform: Option<Self::TransformMatrix>, perspective: Option<Self::PerspectiveMatrix>) {
        self.push_stacking_context(
            &make_primitive_info(Default::default()),
            api::ScrollPolicy::Scrollable,
            transform.map(|v| v.into()),
            api::TransformStyle::Flat,
            perspective,
            api::MixBlendMode::Normal,
            vec![]
        );
    }

    fn pop_stacking_context(&mut self) {
        self.pop_stacking_context();
    }

    // Memory (unsupported)

    type DisplayListItem = !;

    fn get_first(&mut self) -> Option<&mut Self::DisplayListItem> {
        unreachable!()
    }

    fn get_previous(&mut self) -> Option<&mut Self::DisplayListItem> {
        unreachable!()
    }

    // Rect

    type RectColor = Color;

    fn push_rect(&mut self, bounding_client_rect: Self::ClientRect, background_color: Self::RectColor) {
        return_if_nil!(@rect: background_color);

        self.push_rect(
            &make_primitive_info(bounding_client_rect),
            make_color_f(background_color)
        );
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

        self.push_border(
            &make_primitive_info(bounding_client_rect),
            make_border_widths(border_widths),
            make_border_details(border_colors, border_styles)
        );
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

        if let Some(image_key) = image_key {
            self.push_image(
                &make_primitive_info(bounding_client_rect),
                make_image_stretch_size(bounding_client_rect.size),
                make_image_tile_spacing(),
                make_image_rendering_style(),
                image_key
            );
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
        let text_glyphs = text_run.glyphs();

        return_if_nil!(@text: font_instance_key, text_color, text_glyphs, text_content);
        debug_item!(@bounds: self, bounding_client_rect);

        if let Some(font_instance_key) = font_instance_key {
            let transform = Some(TypedTransform3D::create_translation(
                bounding_client_rect.position.left as f32,
                bounding_client_rect.position.top as f32,
                0.0
            ));

            TDisplayListBuilder::push_stacking_context(self, transform, None);

            self.push_text(
                &make_primitive_info(bounding_client_rect.zero_position()),
                text_glyphs.deref(),
                font_instance_key,
                make_color_f(text_color),
                make_glyph_options()
            );

            TDisplayListBuilder::pop_stacking_context(self);
        } else {
            println!("Warning: Font missing for \"{}\"", text_content.as_ref());
        }
    }

    // Debug

    type DebugTextContent = DOMText;
    type DebugImageSrc = DOMText;
}
