// /*
// Copyright 2016 Mozilla
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.
// */

#![cfg(all(feature = "webrender-display-list", feature = "dummy-api-mode"))]
#![feature(proc_macro)]

extern crate rsx;
#[macro_use]
extern crate rsx_primitives;
extern crate webrender;

use rsx::{css, rsx};
use rsx_primitives::build::types::*;
use rsx_primitives::debug;
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
fn test_webrender_reflow_simple_resourceless() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let files = FileCache::new().unwrap();

    let image_keys = ImageKeysAPI::dummy();
    let images = ImageCache::new(image_keys).unwrap();

    let font_keys = FontKeysAPI::dummy();
    let fonts = FontCache::new(font_keys).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 100.0;
    let height = 100.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree, width, height).serialize();
    let stringified = debug::webrender_display_list_to_string(&display_list);

    let expected = r#"[
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×100 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×100 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×100 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×100 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    )
]"#;

    assert_eq!(stringified, expected);
}

#[test]
fn test_webrender_reflow_simple() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::dummy();
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::dummy();
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 100.0;
    let height = 100.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree, width, height).serialize();
    let stringified = debug::webrender_display_list_to_string(&display_list);

    let expected = r#"[
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(66×100 at (0,0)),
                local_clip: Rect(
                    TypedRect(66×100 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(66×100 at (0,0)),
                local_clip: Rect(
                    TypedRect(66×100 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: PushStackingContext(
                PushStackingContextDisplayItem {
                    stacking_context: StackingContext {
                        scroll_policy: Fixed,
                        transform: Some(
                            Value(
                                [I]
                            )
                        ),
                        transform_style: Flat,
                        perspective: None,
                        mix_blend_mode: Normal
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Text(
                TextDisplayItem {
                    font_key: FontInstanceKey(
                        IdNamespace(
                            0
                        ),
                        "#.to_owned()
        + &format!(
            "{}",
            resources
                .fonts()
                .get_font_with_size("FreeSans", 12)
                .unwrap()
                .external_instance_key()
                .1
        )
        + r#"
                    ),
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 1
                    },
                    glyph_options: Some(
                        GlyphOptions {
                            render_mode: Alpha
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(66×100 at (0,0)),
                local_clip: Rect(
                    TypedRect(66×100 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    ),
    (
        GenericDisplayItem {
            item: PopStackingContext,
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    )
]"#;

    assert_eq!(stringified, expected);
}

#[test]
fn test_webrender_reflow_example() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree = rsx! {
        <root style={stylesheet.take(".root")}>
            <image style={stylesheet.take(".image")} src="tests/fixtures/Quantum.png" />
            <text style={stylesheet.take(".text")}>
                Hello world!
            </text>
        </root>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::dummy();
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::dummy();
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree, width, height).serialize();
    let stringified = debug::webrender_display_list_to_string(&display_list);

    let expected = r#"[
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(500×120 at (0,0)),
                local_clip: Rect(
                    TypedRect(500×120 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(500×120 at (0,0)),
                local_clip: Rect(
                    TypedRect(500×120 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Image(
                ImageDisplayItem {
                    image_key: ImageKey(
                        IdNamespace(
                            0
                        ),
                        "#.to_owned()
        + &format!(
            "{}",
            resources
                .images()
                .get_image(image_path)
                .unwrap()
                .external_key()
                .1
        )
        + &r#"
                    ),
                    stretch_size: 80×83,
                    tile_spacing: 0×0,
                    image_rendering: Auto
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(360×25 at (120,48)),
                local_clip: Rect(
                    TypedRect(360×25 at (120,48))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(360×25 at (120,48)),
                local_clip: Rect(
                    TypedRect(360×25 at (120,48))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: PushStackingContext(
                PushStackingContextDisplayItem {
                    stacking_context: StackingContext {
                        scroll_policy: Fixed,
                        transform: Some(
                            Value(
                                [
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    1,
                                    0,
                                    120,
                                    48,
                                    0,
                                    1
                                ]
                            )
                        ),
                        transform_style: Flat,
                        perspective: None,
                        mix_blend_mode: Normal
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Text(
                TextDisplayItem {
                    font_key: FontInstanceKey(
                        IdNamespace(
                            0
                        ),
                        "#.to_owned()
        + &format!(
            "{}",
            resources
                .fonts()
                .get_font_with_size("FreeSans", 12)
                .unwrap()
                .external_instance_key()
                .1
        )
        + r#"
                    ),
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 1
                    },
                    glyph_options: Some(
                        GlyphOptions {
                            render_mode: Alpha
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(66×25 at (0,0)),
                local_clip: Rect(
                    TypedRect(66×25 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    ),
    (
        GenericDisplayItem {
            item: PopStackingContext,
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    )
]"#;

    assert_eq!(stringified, expected);
}

#[test]
fn test_webrender_reflow_example_complex() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let mut tree = rsx! {
        <root style={stylesheet.take(".root")}>
            <image style={stylesheet.take(".image")} src="tests/fixtures/Quantum.png" />
            <text style={stylesheet.take(".text")}>
                Hello world!
            </text>
        </root>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::dummy();
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::dummy();
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree, width, height).serialize();
    let stringified = debug::webrender_display_list_to_string(&display_list);

    let expected = r#"[
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 1,
                        g: 0,
                        b: 0,
                        a: 1
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(500×120 at (0,0)),
                local_clip: Rect(
                    TypedRect(500×120 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(500×120 at (0,0)),
                local_clip: Rect(
                    TypedRect(500×120 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0.5019608,
                        b: 0,
                        a: 1
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Image(
                ImageDisplayItem {
                    image_key: ImageKey(
                        IdNamespace(
                            0
                        ),
                        "#.to_owned()
        + &format!(
            "{}",
            resources
                .images()
                .get_image(image_path)
                .unwrap()
                .external_key()
                .1
        )
        + &r#"
                    ),
                    stretch_size: 80×83,
                    tile_spacing: 0×0,
                    image_rendering: Auto
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(80×83 at (20,20)),
                local_clip: Rect(
                    TypedRect(80×83 at (20,20))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Rectangle(
                RectangleDisplayItem {
                    color: ColorF {
                        r: 0,
                        g: 0,
                        b: 1,
                        a: 1
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(360×25 at (120,48)),
                local_clip: Rect(
                    TypedRect(360×25 at (120,48))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Border(
                BorderDisplayItem {
                    widths: BorderWidths {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0
                    },
                    details: Normal(
                        NormalBorder {
                            left: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            right: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            top: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            bottom: BorderSide {
                                color: ColorF {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0
                                },
                                style: Solid
                            },
                            radius: BorderRadius {
                                top_left: 0×0,
                                top_right: 0×0,
                                bottom_left: 0×0,
                                bottom_right: 0×0
                            }
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(360×25 at (120,48)),
                local_clip: Rect(
                    TypedRect(360×25 at (120,48))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: PushStackingContext(
                PushStackingContextDisplayItem {
                    stacking_context: StackingContext {
                        scroll_policy: Fixed,
                        transform: Some(
                            Value(
                                [
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    1,
                                    0,
                                    120,
                                    48,
                                    0,
                                    1
                                ]
                            )
                        ),
                        transform_style: Flat,
                        perspective: None,
                        mix_blend_mode: Normal
                    }
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        []
    ),
    (
        GenericDisplayItem {
            item: Text(
                TextDisplayItem {
                    font_key: FontInstanceKey(
                        IdNamespace(
                            0
                        ),
                        "#.to_owned()
        + &format!(
            "{}",
            resources
                .fonts()
                .get_font_with_size("FreeSans", 12)
                .unwrap()
                .external_instance_key()
                .1
        )
        + r#"
                    ),
                    color: ColorF {
                        r: 1,
                        g: 1,
                        b: 0,
                        a: 1
                    },
                    glyph_options: Some(
                        GlyphOptions {
                            render_mode: Alpha
                        }
                    )
                }
            ),
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(66×25 at (0,0)),
                local_clip: Rect(
                    TypedRect(66×25 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    ),
    (
        GenericDisplayItem {
            item: PopStackingContext,
            clip_and_scroll: ClipAndScrollInfo {
                scroll_node_id: Clip(
                    0,
                    PipelineId(
                        0,
                        0
                    )
                ),
                clip_node_id: None
            },
            info: PrimitiveInfo {
                rect: TypedRect(0×0 at (0,0)),
                local_clip: Rect(
                    TypedRect(0×0 at (0,0))
                ),
                is_backface_visible: true,
                tag: None
            }
        },
        [
            GlyphInstance {
                index: 43,
                point: (0,12)
            },
            GlyphInstance {
                index: 72,
                point: (8.65625,12)
            },
            GlyphInstance {
                index: 79,
                point: (15.328125,12)
            },
            GlyphInstance {
                index: 79,
                point: (17.984375,12)
            },
            GlyphInstance {
                index: 82,
                point: (20.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (27.3125,12)
            },
            GlyphInstance {
                index: 90,
                point: (30.65625,12)
            },
            GlyphInstance {
                index: 82,
                point: (39.3125,12)
            },
            GlyphInstance {
                index: 85,
                point: (45.984375,12)
            },
            GlyphInstance {
                index: 79,
                point: (49.984375,12)
            },
            GlyphInstance {
                index: 71,
                point: (52.640625,12)
            },
            GlyphInstance {
                index: 3,
                point: (59.3125,12)
            },
            GlyphInstance {
                index: 4,
                point: (62.65625,12)
            }
        ]
    )
]"#;

    assert_eq!(stringified, expected);
}
