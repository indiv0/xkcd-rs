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

    // Retrieve the latest comic ID.
    let work = xkcd::comics::latest(&client);
    let latest_id = core.run(work).unwrap().num;

    // Retrieve all the comics.
    let mut all_comics = Vec::new();
    let start = 1;
    let end = latest_id + 1;
    println!("Retrieving comics from {} to {}", start, end);
    for i in start..end {
        // Skip 404 because that's the 404 page and not a comic.
        if i == 404 {
            continue;
        }

        let work = xkcd::comics::get(&client, i);
        all_comics.push(core.run(work).unwrap());
        println!("Read comic: {}", i);
    }
}
