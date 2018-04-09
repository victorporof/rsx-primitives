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

#![cfg(feature = "bincode-display-list")]
#![feature(proc_macro)]

extern crate bincode;
extern crate rsx;
#[macro_use]
extern crate rsx_primitives;

use std::io::Cursor;

use rsx::{css, rsx};
use rsx_primitives::build::types::*;
use rsx_primitives::rsx_dom::types::*;
use rsx_primitives::rsx_layout::types::*;
use rsx_primitives::rsx_resources::files::types::*;
use rsx_primitives::rsx_resources::fonts::types::*;
use rsx_primitives::rsx_resources::images::types::*;
use rsx_primitives::rsx_resources::types::ResourceGroup;
use rsx_primitives::rsx_shared::traits::*;
use rsx_primitives::rsx_stylesheet::types::*;
use rsx_primitives::types::DisplayList;

#[test]
fn test_json_reflow_simple_resourceless() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let files = FileCache::new().unwrap();

    let image_keys = ImageKeysAPI::new(());
    let images = ImageCache::new(image_keys).unwrap();

    let font_keys = FontKeysAPI::new(());
    let fonts = FontCache::new(font_keys).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 100.0;
    let height = 100.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree).serialize();
    let serialized: Vec<u8> = display_list.into();
    let mut cursor = Cursor::new(serialized);

    {
        let next: BincodeSpecificDisplayItem = bincode::deserialize_from(&mut cursor).unwrap();
        assert_eq!(
            next,
            BincodeSpecificDisplayItem::Rect(BincodeRectDisplayItem {
                bounds: LayoutBoundingClientRect {
                    position: LayoutClientPosition { left: 0, top: 0 },
                    size: LayoutClientSize {
                        width: 0,
                        height: 100
                    }
                },
                display: BincodeRectDisplayProps {
                    color: Color {
                        red: 0,
                        green: 0,
                        blue: 0,
                        alpha: 0
                    }
                }
            })
        );
    }

    {
        let next: BincodeSpecificDisplayItem = bincode::deserialize_from(&mut cursor).unwrap();
        assert_eq!(
            next,
            BincodeSpecificDisplayItem::Border(BincodeBorderDisplayItem {
                bounds: LayoutBoundingClientRect {
                    position: LayoutClientPosition { left: 0, top: 0 },
                    size: LayoutClientSize {
                        width: 0,
                        height: 100
                    }
                },
                display: BincodeBorderDisplayProps {
                    widths: [0, 0, 0, 0],
                    colors: [
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0,
                            alpha: 0
                        },
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0,
                            alpha: 0
                        },
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0,
                            alpha: 0
                        },
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0,
                            alpha: 0
                        }
                    ],
                    styles: [
                        BorderStyle::Solid,
                        BorderStyle::Solid,
                        BorderStyle::Solid,
                        BorderStyle::Solid
                    ]
                }
            })
        );
    }

    {
        let next: Option<BincodeSpecificDisplayItem> = bincode::deserialize_from(&mut cursor).ok();
        assert_eq!(next.is_some(), false);
    }
}
