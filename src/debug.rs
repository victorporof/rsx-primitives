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

#[cfg(feature = "webrender-display-list")]
pub fn webrender_display_list_to_string(display_list: &::webrender::api::BuiltDisplayList) -> String {
    let mut iter = display_list.iter();
    let mut ret = vec![];

    while let Some(item) = iter.next() {
        ret.push((
            *item.display_item(),
            display_list.get(item.glyphs()).collect::<Vec<_>>()
        ));
    }

    format!("{:#?}", ret)
}
