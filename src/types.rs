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

use rsx_dom::types::{DOMTagName, KnownElementName};
use rsx_layout::types::LayoutClientPosition;
use rsx_shared::traits::TDOMNode;
#[cfg(feature = "webrender-display-list")]
use webrender::api::{LayoutSize, PipelineId};

use build::types::{BuiltDisplayList, DisplayListBuilder};
use compare::traits::TDisplayListDiffer;
use compare::types::DisplayListDiff;
use convert::{push_image_dom_node, push_inline_text_node, push_view_dom_node};
use prelude::{DOMNodeId, DOMTree};
use traits::TDisplayListBuilder;

pub struct DisplayList {
    builder: DisplayListBuilder
}

#[cfg(feature = "json-display-list")]
impl Default for DisplayList {
    fn default() -> Self {
        DisplayList::new()
    }
}

impl DisplayList {
    #[cfg(feature = "json-display-list")]
    pub fn new() -> Self {
        DisplayList {
            builder: DisplayListBuilder(Vec::with_capacity(65535))
        }
    }

    #[cfg(feature = "webrender-display-list")]
    pub fn new(pipeline_id: PipelineId, layout_size: LayoutSize) -> Self {
        DisplayList {
            builder: DisplayListBuilder::new(pipeline_id, layout_size)
        }
    }

    #[cfg(feature = "json-display-list")]
    pub fn from(tree: &mut DOMTree) -> Self {
        let mut display_list = DisplayList::new();
        let root = tree.root().id();
        display_list.push(LayoutClientPosition::default(), tree, root);
        display_list
    }

    #[cfg(all(feature = "webrender-display-list", feature = "dummy-api-mode"))]
    pub fn from(tree: &mut DOMTree, width: f32, height: f32) -> Self {
        let mut display_list = DisplayList::new(PipelineId::dummy(), LayoutSize::new(width, height));
        let root = tree.root().id();
        display_list.push(LayoutClientPosition::default(), tree, root);
        display_list
    }

    #[cfg(all(feature = "webrender-display-list", not(feature = "dummy-api-mode")))]
    pub fn from(tree: &mut DOMTree, pipeline_id: PipelineId, layout_size: LayoutSize) -> Self {
        let mut display_list = DisplayList::new(pipeline_id, layout_size);
        let root = tree.root().id();
        display_list.push(LayoutClientPosition::default(), tree, root);
        display_list
    }

    pub fn push(&mut self, client_offsets: LayoutClientPosition, tree: &mut DOMTree, id: DOMNodeId) {
        use self::DOMTagName::*;
        use self::KnownElementName::*;

        let bounding_client_rect;
        let mut next_sibling_id;

        {
            let node_ref = tree.get(id);
            next_sibling_id = node_ref.first_child_id();
            bounding_client_rect = node_ref.get_local_bounding_client_rect() + client_offsets;

            match node_ref.data().tag() {
                None => push_inline_text_node(&mut self.builder, bounding_client_rect, &node_ref),
                Some(&KnownName(Image)) => push_image_dom_node(&mut self.builder, bounding_client_rect, &node_ref),
                Some(&KnownName(Fragment)) => {} // noop
                Some(_) => push_view_dom_node(&mut self.builder, bounding_client_rect, &node_ref)
            }
        }

        while let Some(child_id) = next_sibling_id {
            self.push(bounding_client_rect.position, tree, child_id);
            next_sibling_id = tree.get(child_id).next_sibling_id();
        }

        let mut node_mut = tree.get_mut(id);
        node_mut.set_computed_client_position(client_offsets);
    }

    pub fn diff(&self, other: &Self) -> DisplayListDiff<DisplayListBuilder> {
        self.builder.diff(&other.builder)
    }

    pub fn serialize(self) -> BuiltDisplayList {
        self.builder.serialize()
    }
}
