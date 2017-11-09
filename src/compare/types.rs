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

use smallvec::SmallVec;

use build::traits::TDisplayListBuilder;
use compare::traits::TDisplayListDiffer;

pub struct DisplayListDiff<T: TDisplayListDiffer> {
    pub changes: Vec<DisplayItemDiff<T>>
}

pub enum DisplayItemDiff<T: TDisplayListDiffer> {
    AddRect(T::RectDisplayItem),
    AddBorder(T::BorderDisplayItem),
    AddImage(T::ImageDisplayItem),
    AddText(T::TextDisplayItem),
    RemoveSelf(usize),
    ReplaceSelfWithRect(usize, T::RectDisplayItem),
    ReplaceSelfWithBorder(usize, T::BorderDisplayItem),
    ReplaceSelfWithImage(usize, T::ImageDisplayItem),
    ReplaceSelfWithText(usize, T::TextDisplayItem),
    UpdateSelf(usize, Vec<DisplayItemUpdate<T>>)
}

pub enum DisplayItemUpdate<T: TDisplayListBuilder> {
    BoundsUpdate(BoundsUpdate),
    RectUpdate(RectUpdate<T>),
    BorderUpdate(BorderUpdate<T>),
    ImageUpdate(ImageUpdate<T>),
    TextUpdate(TextUpdate<T>)
}

pub enum BoundsUpdate {
    ChangePositionX(u32),
    ChangePositionY(u32),
    ChangeSizeWidth(u32),
    ChangeSizeHeight(u32)
}

pub enum RectUpdate<T: TDisplayListBuilder> {
    ChangeColor(T::RectColor)
}

pub enum BorderUpdate<T: TDisplayListBuilder> {
    ChangeTopBorder(SingleBorderUpdate<T>),
    ChangeRightBorder(SingleBorderUpdate<T>),
    ChangeBottomBorder(SingleBorderUpdate<T>),
    ChangeLeftBorder(SingleBorderUpdate<T>)
}

pub enum SingleBorderUpdate<T: TDisplayListBuilder> {
    ChangeSize(T::BorderSize),
    ChangeColor(T::BorderColor),
    ChangeStyle(T::BorderStyle)
}

pub enum ImageUpdate<T: TDisplayListBuilder> {
    ChangeImage {
        raw: T::DebugImageSrc,
        #[cfg(not(feature = "no-json-images-mode"))]
        measured_image: T::MeasuredImage
    }
}

pub enum TextUpdate<T: TDisplayListBuilder> {
    ChangeColor(T::TextColor),
    ChangeText {
        raw: SmallVec<[T::DebugTextContent; 1]>,
        #[cfg(not(feature = "no-json-glyphs-mode"))]
        text_run: SmallVec<[T::TextRun; 1]>
    }
}
