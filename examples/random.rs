// Copyright (c) 2016-2017 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate xkcd;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    // Retrieve a random comic.
    let work = xkcd::random::random(&client);
    let random_comic = core.run(work).unwrap();
    println!("Random comic: {:?}\n", random_comic);

    // Retrieve a random comic in the range `start <= x < end`.
    let work = xkcd::random::random_in_range(&client, (1..50));
    let random_comic_in_range = core.run(work).unwrap();
    println!("Random comic in range: {:?}", random_comic_in_range);
}
