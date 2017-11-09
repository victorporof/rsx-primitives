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

#![cfg(feature = "json-display-list")]
#![feature(proc_macro)]

extern crate rsx;
#[macro_use]
extern crate rsx_primitives;
extern crate serde_json;

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
fn test_diff_json_1() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::new(());
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::new(());
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree.generate_layout_tree(&resources);

    let width = 100.0;
    let height = 100.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list_1 = DisplayList::new();
    let display_list_2 = DisplayList::from(&mut tree);
    let diff = display_list_1.diff(&display_list_2);
    let stringified: String = diff.into();

    let expected = r#"
[
  {
    "AddRect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 66,
          "height": 100
        }
      },
      "display": {
        "color": {
          "red": 0,
          "green": 0,
          "blue": 0,
          "alpha": 0
        }
      }
    }
  },
  {
    "AddBorder": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 66,
          "height": 100
        }
      },
      "display": {
        "widths": [
          0,
          0,
          0,
          0
        ],
        "colors": [
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          }
        ],
        "styles": [
          "Solid",
          "Solid",
          "Solid",
          "Solid"
        ]
      }
    }
  },
  {
    "AddText": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 66,
          "height": 100
        }
      },
      "display": {
        "color": {
          "red": 0,
          "green": 0,
          "blue": 0,
          "alpha": 255
        },
        "source_text": [
          {
            "Static": "Hello world !"
          }
        ],
        "shaped_text": [
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 4224,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 43,
                "x_64": 0,
                "y_64": 768
              },
              {
                "glyph_index": 72,
                "x_64": 554,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 981,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 1151,
                "y_64": 768
              },
              {
                "glyph_index": 82,
                "x_64": 1321,
                "y_64": 768
              },
              {
                "glyph_index": 3,
                "x_64": 1748,
                "y_64": 768
              },
              {
                "glyph_index": 90,
                "x_64": 1962,
                "y_64": 768
              },
              {
                "glyph_index": 82,
                "x_64": 2516,
                "y_64": 768
              },
              {
                "glyph_index": 85,
                "x_64": 2943,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 3199,
                "y_64": 768
              },
              {
                "glyph_index": 71,
                "x_64": 3369,
                "y_64": 768
              },
              {
                "glyph_index": 3,
                "x_64": 3796,
                "y_64": 768
              },
              {
                "glyph_index": 4,
                "x_64": 4010,
                "y_64": 768
              }
            ],
            "generation_id": 14732137242825294601
          }
        ]
      }
    }
  }
]
"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_diff_json_2() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree_1 = rsx! {
        <div style={stylesheet.get_copy(".foo").unwrap()}>
            Hello world!
        </div>
    };

    let mut tree_2 = rsx! {
        <div style={stylesheet.get_copy(".foo").unwrap()}>
            Hello world!
        </div>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::new(());
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::new(());
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree_1.generate_layout_tree(&resources);
    tree_2.generate_layout_tree(&resources);

    let width = 100.0;
    let height = 100.0;
    tree_1.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);
    tree_2.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list_1 = DisplayList::from(&mut tree_1);
    let display_list_2 = DisplayList::from(&mut tree_2);
    let diff = display_list_1.diff(&display_list_2);
    let stringified: String = diff.into();

    let expected = "[]";

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_diff_json_3() {
    let mut stylesheet = css!("tests/fixtures/test_1.css");

    let mut tree_1 = rsx! {
        <div style={stylesheet.get_copy(".foo").unwrap()}>
            Hello world!
        </div>
    };

    let mut tree_2 = rsx! {
        <div style={stylesheet.get_copy(".foo").unwrap()}>
            Goodbye cruel world!
        </div>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::new(());
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::new(());
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree_1.generate_layout_tree(&resources);
    tree_2.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree_1.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);
    tree_2.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list_1 = DisplayList::from(&mut tree_1);
    let display_list_2 = DisplayList::from(&mut tree_2);
    let diff = display_list_1.diff(&display_list_2);
    let stringified: String = diff.into();

    let expected = r#"
[
  {
    "M": [
      0,
      [
        {
          "Z": {
            "W": 122
          }
        }
      ]
    ]
  },
  {
    "M": [
      1,
      [
        {
          "Z": {
            "W": 122
          }
        }
      ]
    ]
  },
  {
    "M": [
      2,
      [
        {
          "Z": {
            "W": 122
          }
        },
        {
          "T": {
            "T": "Goodbye\ncruel world !"
          }
        }
      ]
    ]
  }
]
"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_diff_json_4() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let mut tree_1 = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let mut tree_2 = rsx! {
        <div style={stylesheet.take(".bar")}>
            Hello world!
        </div>
    };

    let mut files = FileCache::new().unwrap();

    let image_path = "tests/fixtures/Quantum.png";
    assert!(files.add_file(image_path).is_ok());

    let image_keys = ImageKeysAPI::new(());
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::new(());
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree_1.generate_layout_tree(&resources);
    tree_2.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree_1.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);
    tree_2.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list_1 = DisplayList::from(&mut tree_1);
    let display_list_2 = DisplayList::from(&mut tree_2);
    let diff = display_list_1.diff(&display_list_2);
    let stringified: String = diff.into();

    let expected = r#"
[
  {
    "M": [
      2,
      [
        {
          "T": {
            "C": [
              255,
              0,
              255,
              255
            ]
          }
        }
      ]
    ]
  }
]
"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_diff_json_5() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let mut tree_1 = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let mut tree_2 = rsx! {
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

    let image_keys = ImageKeysAPI::new(());
    let mut images = ImageCache::new(image_keys).unwrap();

    let image_id = ImageId::new(image_path);
    let image_bytes = files.get_file(image_path).unwrap();
    images.add_raw(image_id, image_bytes).unwrap();

    let font_path = "tests/fixtures/FreeSans.ttf";
    assert!(files.add_file(font_path).is_ok());

    let font_keys = FontKeysAPI::new(());
    let mut fonts = FontCache::new(font_keys).unwrap();

    let font_id = FontId::new("FreeSans");
    let font_bytes = files.get_file(font_path).unwrap();
    fonts.add_raw(font_id, font_bytes, 0).unwrap();

    let resources = ResourceGroup::new(files, images, fonts);
    tree_1.generate_layout_tree(&resources);
    tree_2.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree_1.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);
    tree_2.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list_1 = DisplayList::from(&mut tree_1);
    let display_list_2 = DisplayList::from(&mut tree_2);
    let diff = display_list_1.diff(&display_list_2);
    let stringified: String = diff.into();

    let expected = r#"
[
  {
    "M": [
      0,
      [
        {
          "Z": {
            "W": 500
          }
        },
        {
          "Z": {
            "H": 120
          }
        },
        {
          "R": {
            "C": [
              255,
              0,
              0,
              255
            ]
          }
        }
      ]
    ]
  },
  {
    "M": [
      1,
      [
        {
          "Z": {
            "W": 500
          }
        },
        {
          "Z": {
            "H": 120
          }
        }
      ]
    ]
  },
  {
    "IntoRect": [
      2,
      {
        "bounds": {
          "position": {
            "left": 20,
            "top": 20
          },
          "size": {
            "width": 80,
            "height": 83
          }
        },
        "display": {
          "color": {
            "red": 0,
            "green": 128,
            "blue": 0,
            "alpha": 255
          }
        }
      }
    ]
  },
  {
    "AddBorder": {
      "bounds": {
        "position": {
          "left": 20,
          "top": 20
        },
        "size": {
          "width": 80,
          "height": 83
        }
      },
      "display": {
        "widths": [
          0,
          0,
          0,
          0
        ],
        "colors": [
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          }
        ],
        "styles": [
          "Solid",
          "Solid",
          "Solid",
          "Solid"
        ]
      }
    }
  },
  {
    "AddImage": {
      "bounds": {
        "position": {
          "left": 20,
          "top": 20
        },
        "size": {
          "width": 80,
          "height": 83
        }
      },
      "display": {
        "image_src": {
          "Static": "tests/fixtures/Quantum.png"
        },
        "measured_image": {
          "image_key": 0,
          "size": [
            512,
            529
          ]
        }
      }
    }
  },
  {
    "AddRect": {
      "bounds": {
        "position": {
          "left": 120,
          "top": 48
        },
        "size": {
          "width": 360,
          "height": 25
        }
      },
      "display": {
        "color": {
          "red": 0,
          "green": 0,
          "blue": 255,
          "alpha": 255
        }
      }
    }
  },
  {
    "AddBorder": {
      "bounds": {
        "position": {
          "left": 120,
          "top": 48
        },
        "size": {
          "width": 360,
          "height": 25
        }
      },
      "display": {
        "widths": [
          0,
          0,
          0,
          0
        ],
        "colors": [
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          },
          {
            "red": 0,
            "green": 0,
            "blue": 0,
            "alpha": 0
          }
        ],
        "styles": [
          "Solid",
          "Solid",
          "Solid",
          "Solid"
        ]
      }
    }
  },
  {
    "AddText": {
      "bounds": {
        "position": {
          "left": 120,
          "top": 48
        },
        "size": {
          "width": 66,
          "height": 25
        }
      },
      "display": {
        "color": {
          "red": 255,
          "green": 255,
          "blue": 0,
          "alpha": 255
        },
        "source_text": [
          {
            "Static": "Hello world !"
          }
        ],
        "shaped_text": [
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 4224,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 43,
                "x_64": 0,
                "y_64": 768
              },
              {
                "glyph_index": 72,
                "x_64": 554,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 981,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 1151,
                "y_64": 768
              },
              {
                "glyph_index": 82,
                "x_64": 1321,
                "y_64": 768
              },
              {
                "glyph_index": 3,
                "x_64": 1748,
                "y_64": 768
              },
              {
                "glyph_index": 90,
                "x_64": 1962,
                "y_64": 768
              },
              {
                "glyph_index": 82,
                "x_64": 2516,
                "y_64": 768
              },
              {
                "glyph_index": 85,
                "x_64": 2943,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 3199,
                "y_64": 768
              },
              {
                "glyph_index": 71,
                "x_64": 3369,
                "y_64": 768
              },
              {
                "glyph_index": 3,
                "x_64": 3796,
                "y_64": 768
              },
              {
                "glyph_index": 4,
                "x_64": 4010,
                "y_64": 768
              }
            ],
            "generation_id": 14732137242825294601
          }
        ]
      }
    }
  }
]
"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}
