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

use std::ffi::CString;
use std::os::raw::c_char;

use build::json_types::BuiltDisplayList;

impl Into<String> for BuiltDisplayList {
    fn into(self) -> String {
        self.0
    }
}

impl Into<*mut c_char> for BuiltDisplayList {
    fn into(self) -> *mut c_char {
        let string: String = self.into();
        unsafe { CString::from_vec_unchecked(string.into_bytes()) }.into_raw()
    }
}
