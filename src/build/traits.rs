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

use serde::{Deserialize, Serialize};

pub trait TDisplayListBuilder
where
    Self::TextColor: Serialize + for<'a> Deserialize<'a>,
    Self::TextRun: Serialize + for<'a> Deserialize<'a>,
    Self::RectColor: Serialize + for<'a> Deserialize<'a>,
    Self::BorderSize: Serialize + for<'a> Deserialize<'a>,
    Self::BorderColor: Serialize + for<'a> Deserialize<'a>,
    Self::BorderStyle: Serialize + for<'a> Deserialize<'a>,
    Self::MeasuredImage: Serialize + for<'a> Deserialize<'a>,
    Self::DebugTextContent: Serialize + for<'a> Deserialize<'a>,
    Self::DebugImageSrc: Serialize + for<'a> Deserialize<'a>
{
    // Serialize

    type Serialized;
    fn serialize(self) -> Self::Serialized;

    // General

    type ClientRect;
    type TransformMatrix;
    type PerspectiveMatrix;
    fn push_stacking_context(&mut self, Option<Self::TransformMatrix>, Option<Self::PerspectiveMatrix>);
    fn pop_stacking_context(&mut self);

    // Memory

    type DisplayListItem;
    fn get_first(&mut self) -> Option<&mut Self::DisplayListItem>;
    fn get_previous(&mut self) -> Option<&mut Self::DisplayListItem>;

    // Rect

    type RectColor;
    fn push_rect(&mut self, Self::ClientRect, Self::RectColor);

    // Border

    type BorderSize;
    type BorderColor;
    type BorderStyle;
    fn push_border(&mut self, Self::ClientRect, [Self::BorderSize; 4], [Self::BorderColor; 4], [Self::BorderStyle; 4]);

    // Image

    type MeasuredImage;
    fn push_image(&mut self, Self::ClientRect, &Self::MeasuredImage, &Self::DebugImageSrc);

    // Text

    type TextColor;
    type TextRun;
    fn push_text(&mut self, Self::ClientRect, Self::TextColor, &Self::TextRun, &Self::DebugTextContent);

    // Debug

    type DebugTextContent;
    type DebugImageSrc;
}
