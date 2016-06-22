// Copyright (c) 2016 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
extern crate xkcd;

fn main() {
    let client = hyper::Client::new();

    // Retrieve the latest comic.
    let latest_comic = xkcd::comics::latest(&client);
    println!("Latest comic: {:?}", latest_comic);
}
