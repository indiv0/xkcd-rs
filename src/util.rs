// Copyright (c) 2017 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_sample_data_from_path<P>(path: P) -> String
    where P: AsRef<Path>,
{
    let mut file = File::open(path).unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    body
}
