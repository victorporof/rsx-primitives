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
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 0,
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
    "Border": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 0,
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
  }
]
"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_json_reflow_simple() {
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

    let display_list = DisplayList::from(&mut tree).serialize();
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
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
    "Border": {
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
    "Text": {
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
fn test_json_reflow_example() {
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

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree).serialize();
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
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
    "Border": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
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
    "Rect": {
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
          "green": 0,
          "blue": 0,
          "alpha": 0
        }
      }
    }
  },
  {
    "Border": {
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
    "Image": {
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
    "Rect": {
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
          "blue": 0,
          "alpha": 0
        }
      }
    }
  },
  {
    "Border": {
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
    "Text": {
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
fn test_json_reflow_example_complex() {
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

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree).serialize();
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
        }
      },
      "display": {
        "color": {
          "red": 255,
          "green": 0,
          "blue": 0,
          "alpha": 255
        }
      }
    }
  },
  {
    "Border": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
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
    "Rect": {
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
  },
  {
    "Border": {
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
    "Image": {
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
    "Rect": {
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
    "Border": {
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
    "Text": {
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

#[test]
fn test_json_reflow_example_compound_1() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let mut tree = rsx! {
        <root style={stylesheet.take(".root")}>
            <image style={stylesheet.take(".image")} src="tests/fixtures/Quantum.png" />
            <text style={stylesheet.take(".text")}>
                {"Hello "}{"world !"}
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
    tree.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree).serialize();
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
        }
      },
      "display": {
        "color": {
          "red": 255,
          "green": 0,
          "blue": 0,
          "alpha": 255
        }
      }
    }
  },
  {
    "Border": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
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
    "Rect": {
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
  },
  {
    "Border": {
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
    "Image": {
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
    "Rect": {
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
    "Border": {
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
    "Text": {
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
            "Static": "Hello "
          },
          {
            "Static": "world !"
          }
        ],
        "shaped_text": [
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 1962,
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
              }
            ],
            "generation_id": 12598805059617695050
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 2262,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 90,
                "x_64": 0,
                "y_64": 768
              },
              {
                "glyph_index": 82,
                "x_64": 554,
                "y_64": 768
              },
              {
                "glyph_index": 85,
                "x_64": 981,
                "y_64": 768
              },
              {
                "glyph_index": 79,
                "x_64": 1237,
                "y_64": 768
              },
              {
                "glyph_index": 71,
                "x_64": 1407,
                "y_64": 768
              },
              {
                "glyph_index": 3,
                "x_64": 1834,
                "y_64": 768
              },
              {
                "glyph_index": 4,
                "x_64": 2048,
                "y_64": 768
              }
            ],
            "generation_id": 8713082891591790693
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
fn test_json_reflow_example_compound_2() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let mut tree = rsx! {
        <root style={stylesheet.take(".root")}>
            <image style={stylesheet.take(".image")} src="tests/fixtures/Quantum.png" />
            <text style={stylesheet.take(".text")}>
                {"H"}{"e"}{"l"}{"l"}{"o"}{" "}{"w"}{"o"}{"r"}{"l"}{"d"}{" "}{"!"}
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
    tree.generate_layout_tree(&resources);

    let width = 1024.0;
    let height = 768.0;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);

    let display_list = DisplayList::from(&mut tree).serialize();
    let stringified: String = display_list.into();

    let expected = r#"
[
  {
    "Rect": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
        }
      },
      "display": {
        "color": {
          "red": 255,
          "green": 0,
          "blue": 0,
          "alpha": 255
        }
      }
    }
  },
  {
    "Border": {
      "bounds": {
        "position": {
          "left": 0,
          "top": 0
        },
        "size": {
          "width": 500,
          "height": 120
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
    "Rect": {
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
  },
  {
    "Border": {
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
    "Image": {
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
    "Rect": {
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
    "Border": {
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
    "Text": {
      "bounds": {
        "position": {
          "left": 120,
          "top": 48
        },
        "size": {
          "width": 65,
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
            "Static": "H"
          },
          {
            "Static": "e"
          },
          {
            "Static": "l"
          },
          {
            "Static": "l"
          },
          {
            "Static": "o"
          },
          {
            "Static": " "
          },
          {
            "Static": "w"
          },
          {
            "Static": "o"
          },
          {
            "Static": "r"
          },
          {
            "Static": "l"
          },
          {
            "Static": "d"
          },
          {
            "Static": " "
          },
          {
            "Static": "!"
          }
        ],
        "shaped_text": [
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 554,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 43,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 659716905384036824
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 427,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 72,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 616369758961961485
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 170,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 79,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 625376958218562972
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 170,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 79,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 625376958218562972
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 427,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 82,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 625939908172017779
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 214,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 3,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 560074763608722560
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 554,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 90,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 633821207521520427
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 427,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 82,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 625939908172017779
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 256,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 85,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 638324807149726558
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 170,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 79,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 625376958218562972
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 427,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 71,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 617495658869060324
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 214,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 3,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 560074763608722560
          },
          {
            "font_key": 0,
            "font_instance_key": 1,
            "width_64": 214,
            "height_64": 1088,
            "glyphs": [
              {
                "glyph_index": 4,
                "x_64": 0,
                "y_64": 768
              }
            ],
            "generation_id": 558948863701623721
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
