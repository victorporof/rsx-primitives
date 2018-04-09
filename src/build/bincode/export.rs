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

use std::mem;
use std::os::raw::c_uchar;

use build::bincode_types::BuiltDisplayList;

impl Into<Vec<u8>> for BuiltDisplayList {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl Into<(*mut c_uchar, usize)> for BuiltDisplayList {
    fn into(self) -> (*mut c_uchar, usize) {
        let mut vec: Vec<u8> = self.into();
        vec.shrink_to_fit();

        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        mem::forget(vec);

        (ptr, len)
    }
}
