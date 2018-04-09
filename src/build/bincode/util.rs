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

use std::io;
use std::io::Write;
use std::ptr;

use bincode;
use serde::Serialize;

struct UnsafeVecWriter(*mut u8);

impl Write for UnsafeVecWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), self.0, buf.len());
            self.0 = self.0.offset(buf.len() as isize);
        }
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), self.0, buf.len());
            self.0 = self.0.offset(buf.len() as isize);
        }
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn serialize_fast<T: Serialize>(vec: &mut Vec<u8>, e: &T) {
    let size = bincode::serialized_size(&e).unwrap() as usize;
    vec.reserve(size);

    let old_len = vec.len();
    let ptr = unsafe { vec.as_mut_ptr().offset(old_len as isize) };

    let mut w = UnsafeVecWriter(ptr);
    bincode::serialize_into(&mut w, e).unwrap();

    unsafe {
        vec.set_len(old_len + size);
    }

    debug_assert_eq!(((w.0 as usize) - (vec.as_ptr() as usize)), vec.len());
}
