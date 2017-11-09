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

use std::ffi::CString;
use std::os::raw::c_char;

use rsx_dom::types::DOMText;
use rsx_stylesheet::types::{BorderStyle, Color};
use serde::{Serialize, Serializer};
use serde_json;

use build::traits::TDisplayListBuilder;
use compare::traits::TDisplayListDiffer;
use compare::types::{
    BorderUpdate,
    BoundsUpdate,
    DisplayItemDiff,
    DisplayItemUpdate,
    DisplayListDiff,
    ImageUpdate,
    RectUpdate,
    SingleBorderUpdate,
    TextUpdate
};
use prelude::{MeasuredImage, ShapedText};

pub const DISPLAY_ITEM_DIFF_KEY: &str = "Diff";

pub const ADD_RECT_KEY: &str = "AddRect";
pub const ADD_BORDER_KEY: &str = "AddBorder";
pub const ADD_IMAGE_KEY: &str = "AddImage";
pub const ADD_TEXT_KEY: &str = "AddText";
pub const REMOVE_SELF_KEY: &str = "D";
pub const REPLACE_SELF_WITH_RECT_KEY: &str = "IntoRect";
pub const REPLACE_SELF_WITH_BORDER_KEY: &str = "IntoBorder";
pub const REPLACE_SELF_WITH_IMAGE_KEY: &str = "IntoImage";
pub const REPLACE_SELF_WITH_TEXT_KEY: &str = "IntoText";
pub const UPDATE_SELF_KEY: &str = "M";

pub const BOUNDS_UPDATE_KEY: &str = "Z";
pub const BOUNDS_UPDATE_CHANGE_X_KEY: &str = "X";
pub const BOUNDS_UPDATE_CHANGE_Y_KEY: &str = "Y";
pub const BOUNDS_UPDATE_CHANGE_WIDTH_KEY: &str = "W";
pub const BOUNDS_UPDATE_CHANGE_HEIGHT_KEY: &str = "H";

pub const RECT_UPDATE_KEY: &str = "R";
pub const RECT_UPDATE_CHANGE_COLOR_KEY: &str = "C";

pub const BORDER_UPDATE_KEY: &str = "B";
pub const BORDER_UPDATE_CHANGE_TOP_KEY: &str = "T";
pub const BORDER_UPDATE_CHANGE_RIGHT_KEY: &str = "R";
pub const BORDER_UPDATE_CHANGE_BOTTOM_KEY: &str = "B";
pub const BORDER_UPDATE_CHANGE_LEFT_KEY: &str = "L";
pub const BORDER_UPDATE_CHANGE_WIDTH_KEY: &str = "W";
pub const BORDER_UPDATE_CHANGE_COLOR_KEY: &str = "C";
pub const BORDER_UPDATE_CHANGE_STYLE_KEY: &str = "S";

pub const IMAGE_UPDATE_KEY: &str = "I";
pub const IMAGE_UPDATE_CHANGE_DATA_KEY: &str = "I";

pub const TEXT_UPDATE_KEY: &str = "T";
pub const TEXT_UPDATE_CHANGE_COLOR_KEY: &str = "C";
pub const TEXT_UPDATE_CHANGE_CONTENT_KEY: &str = "T";

impl<T> Into<String> for DisplayListDiff<T>
where
    T: TDisplayListDiffer<
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle,
        // Image
        MeasuredImage = MeasuredImage,
        // Text
        TextColor = Color,
        TextRun = ShapedText,
        // Debug
        DebugImageSrc = DOMText,
        DebugTextContent = DOMText
    >
{
    #[cfg(not(feature = "pretty-json-mode"))]
    fn into(self) -> String {
        serde_json::to_string(&self.changes).unwrap_or_else(|_| "".to_string())
    }

    #[cfg(feature = "pretty-json-mode")]
    fn into(self) -> String {
        serde_json::to_string_pretty(&self.changes).unwrap_or_else(|_| "".to_string())
    }
}

impl<T> Into<*mut c_char> for DisplayListDiff<T>
where
    T: TDisplayListDiffer<
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle,
        // Image
        MeasuredImage = MeasuredImage,
        // Text
        TextColor = Color,
        TextRun = ShapedText,
        // Debug
        DebugImageSrc = DOMText,
        DebugTextContent = DOMText
    >
{
    fn into(self) -> *mut c_char {
        let string: String = self.into();
        unsafe { CString::from_vec_unchecked(string.into_bytes()) }.into_raw()
    }
}

impl<T> Serialize for DisplayItemDiff<T>
where
    T: TDisplayListDiffer<
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle,
        // Image
        MeasuredImage = MeasuredImage,
        // Text
        TextColor = Color,
        TextRun = ShapedText,
        // Debug
        DebugImageSrc = DOMText,
        DebugTextContent = DOMText
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &DisplayItemDiff::AddRect(ref item) => serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 0, ADD_RECT_KEY, item),
            &DisplayItemDiff::AddBorder(ref item) => serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 1, ADD_BORDER_KEY, item),
            &DisplayItemDiff::AddImage(ref item) => serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 2, ADD_IMAGE_KEY, item),
            &DisplayItemDiff::AddText(ref item) => serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 3, ADD_TEXT_KEY, item),
            &DisplayItemDiff::RemoveSelf(ref index) => {
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 4, REMOVE_SELF_KEY, index)
            }
            &DisplayItemDiff::ReplaceSelfWithRect(ref index, ref item) => {
                let v = &(index, item);
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 5, REPLACE_SELF_WITH_RECT_KEY, v)
            }
            &DisplayItemDiff::ReplaceSelfWithBorder(ref index, ref item) => {
                let v = &(index, item);
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 6, REPLACE_SELF_WITH_BORDER_KEY, v)
            }
            &DisplayItemDiff::ReplaceSelfWithImage(ref index, ref item) => {
                let v = &(index, item);
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 7, REPLACE_SELF_WITH_IMAGE_KEY, v)
            }
            &DisplayItemDiff::ReplaceSelfWithText(ref index, ref item) => {
                let v = &(index, item);
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 8, REPLACE_SELF_WITH_TEXT_KEY, v)
            }
            &DisplayItemDiff::UpdateSelf(ref index, ref vec) => {
                let v = &(index, vec);
                serializer.serialize_newtype_variant(DISPLAY_ITEM_DIFF_KEY, 9, UPDATE_SELF_KEY, v)
            }
        }
    }
}

impl<T> Serialize for DisplayItemUpdate<T>
where
    T: TDisplayListBuilder<
        // Rect
        RectColor = Color,
        // Border
        BorderSize = u32,
        BorderColor = Color,
        BorderStyle = BorderStyle,
        // Image
        MeasuredImage = MeasuredImage,
        // Text
        TextColor = Color,
        TextRun = ShapedText,
        // Debug
        DebugImageSrc = DOMText,
        DebugTextContent = DOMText
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &DisplayItemUpdate::BoundsUpdate(ref bounds_update) => {
                serializer.serialize_newtype_variant(UPDATE_SELF_KEY, 0, BOUNDS_UPDATE_KEY, bounds_update)
            }
            &DisplayItemUpdate::RectUpdate(ref rect_update) => {
                serializer.serialize_newtype_variant(UPDATE_SELF_KEY, 1, RECT_UPDATE_KEY, rect_update)
            }
            &DisplayItemUpdate::BorderUpdate(ref border_update) => {
                serializer.serialize_newtype_variant(UPDATE_SELF_KEY, 2, BORDER_UPDATE_KEY, border_update)
            }
            &DisplayItemUpdate::ImageUpdate(ref image_update) => {
                serializer.serialize_newtype_variant(UPDATE_SELF_KEY, 3, IMAGE_UPDATE_KEY, image_update)
            }
            &DisplayItemUpdate::TextUpdate(ref text_update) => {
                serializer.serialize_newtype_variant(UPDATE_SELF_KEY, 4, TEXT_UPDATE_KEY, text_update)
            }
        }
    }
}

impl Serialize for BoundsUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &BoundsUpdate::ChangePositionX(ref l) => {
                serializer.serialize_newtype_variant(BOUNDS_UPDATE_KEY, 0, BOUNDS_UPDATE_CHANGE_X_KEY, l)
            }
            &BoundsUpdate::ChangePositionY(ref t) => {
                serializer.serialize_newtype_variant(BOUNDS_UPDATE_KEY, 1, BOUNDS_UPDATE_CHANGE_Y_KEY, t)
            }
            &BoundsUpdate::ChangeSizeWidth(ref w) => {
                serializer.serialize_newtype_variant(BOUNDS_UPDATE_KEY, 2, BOUNDS_UPDATE_CHANGE_WIDTH_KEY, w)
            }
            &BoundsUpdate::ChangeSizeHeight(ref h) => {
                serializer.serialize_newtype_variant(BOUNDS_UPDATE_KEY, 3, BOUNDS_UPDATE_CHANGE_HEIGHT_KEY, h)
            }
        }
    }
}

impl<T> Serialize for RectUpdate<T>
where
    T: TDisplayListBuilder<RectColor = Color>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &RectUpdate::ChangeColor(ref color) => {
                let v = &[color.red, color.green, color.blue, color.alpha];
                serializer.serialize_newtype_variant(RECT_UPDATE_KEY, 0, RECT_UPDATE_CHANGE_COLOR_KEY, v)
            }
        }
    }
}

impl<T> Serialize for BorderUpdate<T>
where
    T: TDisplayListBuilder<BorderSize = u32, BorderColor = Color, BorderStyle = BorderStyle>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &BorderUpdate::ChangeTopBorder(ref c) => {
                serializer.serialize_newtype_variant(BORDER_UPDATE_KEY, 0, BORDER_UPDATE_CHANGE_TOP_KEY, c)
            }
            &BorderUpdate::ChangeRightBorder(ref c) => {
                serializer.serialize_newtype_variant(BORDER_UPDATE_KEY, 1, BORDER_UPDATE_CHANGE_RIGHT_KEY, c)
            }
            &BorderUpdate::ChangeBottomBorder(ref c) => {
                serializer.serialize_newtype_variant(BORDER_UPDATE_KEY, 2, BORDER_UPDATE_CHANGE_BOTTOM_KEY, c)
            }
            &BorderUpdate::ChangeLeftBorder(ref c) => {
                serializer.serialize_newtype_variant(BORDER_UPDATE_KEY, 3, BORDER_UPDATE_CHANGE_LEFT_KEY, c)
            }
        }
    }
}

impl<T> Serialize for SingleBorderUpdate<T>
where
    T: TDisplayListBuilder<BorderSize = u32, BorderColor = Color, BorderStyle = BorderStyle>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &SingleBorderUpdate::ChangeSize(ref width) => {
                serializer.serialize_newtype_variant("", 0, BORDER_UPDATE_CHANGE_WIDTH_KEY, width)
            }
            &SingleBorderUpdate::ChangeColor(ref color) => {
                let value = &[color.red, color.green, color.blue, color.alpha];
                serializer.serialize_newtype_variant("", 1, BORDER_UPDATE_CHANGE_COLOR_KEY, value)
            }
            &SingleBorderUpdate::ChangeStyle(ref style) => {
                serializer.serialize_newtype_variant("", 2, BORDER_UPDATE_CHANGE_STYLE_KEY, style)
            }
        }
    }
}

impl<T> Serialize for ImageUpdate<T>
where
    T: TDisplayListBuilder<MeasuredImage = MeasuredImage, DebugImageSrc = DOMText>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &ImageUpdate::ChangeImage { ref raw, .. } => {
                let v = raw.as_ref();
                serializer.serialize_newtype_variant(IMAGE_UPDATE_KEY, 0, IMAGE_UPDATE_CHANGE_DATA_KEY, v)
            }
        }
    }
}

impl<T> Serialize for TextUpdate<T>
where
    T: TDisplayListBuilder<TextColor = Color, TextRun = ShapedText, DebugTextContent = DOMText>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            &TextUpdate::ChangeColor(ref color) => {
                let v = &[color.red, color.green, color.blue, color.alpha];
                serializer.serialize_newtype_variant(TEXT_UPDATE_KEY, 0, TEXT_UPDATE_CHANGE_COLOR_KEY, v)
            }
            &TextUpdate::ChangeText { ref raw, .. } => {
                let v = &raw.join("");
                serializer.serialize_newtype_variant(TEXT_UPDATE_KEY, 1, TEXT_UPDATE_CHANGE_CONTENT_KEY, v)
            }
        }
    }
}
