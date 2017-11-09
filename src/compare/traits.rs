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

use build::traits::TDisplayListBuilder;

pub trait TDisplayListDiffer: TDisplayListBuilder
where
    Self::TextDisplayItem: Serialize + for<'a> Deserialize<'a>,
    Self::RectDisplayItem: Serialize + for<'a> Deserialize<'a>,
    Self::BorderDisplayItem: Serialize + for<'a> Deserialize<'a>,
    Self::ImageDisplayItem: Serialize + for<'a> Deserialize<'a>
{
    type ListDiff;
    type ItemDiff;
    type TextDisplayItem;
    type RectDisplayItem;
    type BorderDisplayItem;
    type ImageDisplayItem;

    fn diff(&self, other: &Self) -> Self::ListDiff;
}

pub trait TTextDisplayItemDiffer {
    type ItemUpdate;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate>;
}

pub trait TRectDisplayItemDiffer {
    type ItemUpdate;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate>;
}

pub trait TBorderDisplayItemDiffer {
    type ItemUpdate;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate>;
}

pub trait TImageDisplayItemDiffer {
    type ItemUpdate;

    fn diff(&self, other: &Self) -> Vec<Self::ItemUpdate>;
}
