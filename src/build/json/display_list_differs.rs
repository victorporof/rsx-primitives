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

use build::types::{BorderDisplayItem, DisplayListBuilder, ImageDisplayItem, RectDisplayItem, SpecificDisplayItem, TextDisplayItem};
use compare::traits::{
    TBorderDisplayItemDiffer,
    TDisplayListDiffer,
    TImageDisplayItemDiffer,
    TRectDisplayItemDiffer,
    TTextDisplayItemDiffer
};
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

impl TDisplayListDiffer for DisplayListBuilder {
    type ListDiff = DisplayListDiff<Self>;
    type ItemDiff = DisplayItemDiff<DisplayListBuilder>;
    type TextDisplayItem = TextDisplayItem;
    type RectDisplayItem = RectDisplayItem;
    type BorderDisplayItem = BorderDisplayItem;
    type ImageDisplayItem = ImageDisplayItem;

    fn diff(&self, other: &Self) -> Self::ListDiff {
        let mut this = self.0.iter().fuse();
        let mut other = other.0.iter().fuse();
        let mut changes = Vec::with_capacity(usize::max(this.len(), other.len()));

        let mut i = 0;

        loop {
            use self::DisplayItemDiff::*;
            use self::SpecificDisplayItem::*;

            match (this.next(), other.next()) {
                // Add
                (None, Some(&Rect(ref rect_b))) => {
                    changes.push(AddRect(*rect_b));
                }
                (None, Some(&Border(ref border_b))) => {
                    changes.push(AddBorder(*border_b));
                }
                (None, Some(&Image(ref image_b))) => {
                    changes.push(AddImage(ImageDisplayItem::clone(image_b)));
                }
                (None, Some(&Text(ref text_b))) => {
                    changes.push(AddText(TextDisplayItem::clone(text_b)));
                }

                // Remove
                (Some(_), None) => {
                    changes.push(RemoveSelf(i));
                }

                // Update
                (Some(&Rect(ref rect_a)), Some(&Rect(ref rect_b))) => {
                    let updates = rect_a.diff(rect_b);
                    if !updates.is_empty() {
                        changes.push(UpdateSelf(i, updates));
                    }
                }
                (Some(&Border(ref border_a)), Some(&Border(ref border_b))) => {
                    let updates = border_a.diff(border_b);
                    if !updates.is_empty() {
                        changes.push(UpdateSelf(i, updates));
                    }
                }
                (Some(&Image(ref image_a)), Some(&Image(ref image_b))) => {
                    let updates = image_a.diff(image_b);
                    if !updates.is_empty() {
                        changes.push(UpdateSelf(i, updates));
                    }
                }
                (Some(&Text(ref text_a)), Some(&Text(ref text_b))) => {
                    let updates = text_a.diff(text_b);
                    if !updates.is_empty() {
                        changes.push(UpdateSelf(i, updates));
                    }
                }

                // Replace
                (Some(_), Some(&Rect(ref rect_b))) => {
                    changes.push(ReplaceSelfWithRect(i, *rect_b));
                }
                (Some(_), Some(&Border(ref border_b))) => {
                    changes.push(ReplaceSelfWithBorder(i, *border_b));
                }
                (Some(_), Some(&Image(ref image_b))) => {
                    changes.push(ReplaceSelfWithImage(i, ImageDisplayItem::clone(image_b)));
                }
                (Some(_), Some(&Text(ref text_b))) => {
                    changes.push(ReplaceSelfWithText(i, TextDisplayItem::clone(text_b)));
                }

                // Done.
                (None, None) => break
            }

            i += 1;
        }

        DisplayListDiff { changes }
    }
}

impl TRectDisplayItemDiffer for RectDisplayItem {
    type ItemUpdate = DisplayItemUpdate<DisplayListBuilder>;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate> {
        use self::BoundsUpdate::{ChangePositionX, ChangePositionY, ChangeSizeHeight, ChangeSizeWidth};
        use self::DisplayItemUpdate::{BoundsUpdate, RectUpdate};
        use self::RectUpdate::ChangeColor;

        let mut changes = Vec::new();

        if self.bounds.position.left != other.bounds.position.left {
            changes.push(BoundsUpdate(ChangePositionX(other.bounds.position.left)));
        }
        if self.bounds.position.top != other.bounds.position.top {
            changes.push(BoundsUpdate(ChangePositionY(other.bounds.position.top)));
        }
        if self.bounds.size.width != other.bounds.size.width {
            changes.push(BoundsUpdate(ChangeSizeWidth(other.bounds.size.width)));
        }
        if self.bounds.size.height != other.bounds.size.height {
            changes.push(BoundsUpdate(ChangeSizeHeight(other.bounds.size.height)));
        }
        if self.display.color != other.display.color {
            changes.push(RectUpdate(ChangeColor(other.display.color)));
        }

        changes
    }
}

impl TBorderDisplayItemDiffer for BorderDisplayItem {
    type ItemUpdate = DisplayItemUpdate<DisplayListBuilder>;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate> {
        use self::BorderUpdate::{
            ChangeBottomBorder as Bottom,
            ChangeLeftBorder as Left,
            ChangeRightBorder as Right,
            ChangeTopBorder as Top
        };
        use self::BoundsUpdate::{ChangePositionX, ChangePositionY, ChangeSizeHeight, ChangeSizeWidth};
        use self::DisplayItemUpdate::{BorderUpdate, BoundsUpdate};
        use self::SingleBorderUpdate::*;

        let mut changes = Vec::new();

        if self.bounds.position.left != other.bounds.position.left {
            changes.push(BoundsUpdate(ChangePositionX(other.bounds.position.left)));
        }
        if self.bounds.position.top != other.bounds.position.top {
            changes.push(BoundsUpdate(ChangePositionY(other.bounds.position.top)));
        }
        if self.bounds.size.width != other.bounds.size.width {
            changes.push(BoundsUpdate(ChangeSizeWidth(other.bounds.size.width)));
        }
        if self.bounds.size.height != other.bounds.size.height {
            changes.push(BoundsUpdate(ChangeSizeHeight(other.bounds.size.height)));
        }
        if self.display.widths[0] != other.display.widths[0] {
            changes.push(BorderUpdate(Top(ChangeSize(other.display.widths[0]))));
        }
        if self.display.widths[1] != other.display.widths[1] {
            changes.push(BorderUpdate(Right(ChangeSize(other.display.widths[1]))));
        }
        if self.display.widths[2] != other.display.widths[2] {
            changes.push(BorderUpdate(Bottom(ChangeSize(other.display.widths[2]))));
        }
        if self.display.widths[3] != other.display.widths[3] {
            changes.push(BorderUpdate(Left(ChangeSize(other.display.widths[3]))));
        }
        if self.display.colors[0] != other.display.colors[0] {
            changes.push(BorderUpdate(Top(ChangeColor(other.display.colors[0]))));
        }
        if self.display.colors[1] != other.display.colors[1] {
            changes.push(BorderUpdate(Right(ChangeColor(other.display.colors[1]))));
        }
        if self.display.colors[2] != other.display.colors[2] {
            changes.push(BorderUpdate(Bottom(ChangeColor(other.display.colors[2]))));
        }
        if self.display.colors[3] != other.display.colors[3] {
            changes.push(BorderUpdate(Left(ChangeColor(other.display.colors[3]))));
        }
        if self.display.styles[0] != other.display.styles[0] {
            changes.push(BorderUpdate(Top(ChangeStyle(other.display.styles[0]))));
        }
        if self.display.styles[1] != other.display.styles[1] {
            changes.push(BorderUpdate(Right(ChangeStyle(other.display.styles[1]))));
        }
        if self.display.styles[2] != other.display.styles[2] {
            changes.push(BorderUpdate(Bottom(ChangeStyle(other.display.styles[2]))));
        }
        if self.display.styles[3] != other.display.styles[3] {
            changes.push(BorderUpdate(Left(ChangeStyle(other.display.styles[3]))));
        }

        changes
    }
}

impl TImageDisplayItemDiffer for ImageDisplayItem {
    type ItemUpdate = DisplayItemUpdate<DisplayListBuilder>;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate> {
        use self::BoundsUpdate::{ChangePositionX, ChangePositionY, ChangeSizeHeight, ChangeSizeWidth};
        use self::DisplayItemUpdate::{BoundsUpdate, ImageUpdate};
        use self::ImageUpdate::ChangeImage;

        let mut changes = Vec::new();

        if self.bounds.position.left != other.bounds.position.left {
            changes.push(BoundsUpdate(ChangePositionX(other.bounds.position.left)));
        }
        if self.bounds.position.top != other.bounds.position.top {
            changes.push(BoundsUpdate(ChangePositionY(other.bounds.position.top)));
        }
        if self.bounds.size.width != other.bounds.size.width {
            changes.push(BoundsUpdate(ChangeSizeWidth(other.bounds.size.width)));
        }
        if self.bounds.size.height != other.bounds.size.height {
            changes.push(BoundsUpdate(ChangeSizeHeight(other.bounds.size.height)));
        }
        if self.display.measured_image != other.display.measured_image {
            changes.push(ImageUpdate(ChangeImage {
                raw: other.display.image_src.clone(),
                #[cfg(not(feature = "no-json-images-mode"))]
                measured_image: other.display.measured_image.clone()
            }));
        }

        changes
    }
}

impl TTextDisplayItemDiffer for TextDisplayItem {
    type ItemUpdate = DisplayItemUpdate<DisplayListBuilder>;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate> {
        use self::BoundsUpdate::{ChangePositionX, ChangePositionY, ChangeSizeHeight, ChangeSizeWidth};
        use self::DisplayItemUpdate::{BoundsUpdate, TextUpdate};
        use self::TextUpdate::{ChangeColor, ChangeText};

        let mut changes = Vec::new();

        if self.bounds.position.left != other.bounds.position.left {
            changes.push(BoundsUpdate(ChangePositionX(other.bounds.position.left)));
        }
        if self.bounds.position.top != other.bounds.position.top {
            changes.push(BoundsUpdate(ChangePositionY(other.bounds.position.top)));
        }
        if self.bounds.size.width != other.bounds.size.width {
            changes.push(BoundsUpdate(ChangeSizeWidth(other.bounds.size.width)));
        }
        if self.bounds.size.height != other.bounds.size.height {
            changes.push(BoundsUpdate(ChangeSizeHeight(other.bounds.size.height)));
        }
        if self.display.color != other.display.color {
            changes.push(TextUpdate(ChangeColor(other.display.color)));
        }
        if self.display.shaped_text != other.display.shaped_text {
            changes.push(TextUpdate(ChangeText {
                raw: other.display.source_text.clone(),
                #[cfg(not(feature = "no-json-glyphs-mode"))]
                text_run: other.display.shaped_text.clone()
            }));
        }

        changes
    }
}
