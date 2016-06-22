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

    // Retrieve a random comic.
    let random_comic = xkcd::random::random(&client);
    println!("Random comic: {:?}\n", random_comic);

    // Retrieve a random comic in the range `start <= x < end`.
    let random_comic_in_range = xkcd::random::random_in_range(&client, (1..50));
    println!("Random comic in range: {:?}", random_comic_in_range);
}
