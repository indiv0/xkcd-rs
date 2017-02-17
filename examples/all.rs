// Copyright (c) 2016 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
extern crate hyper_native_tls;
extern crate xkcd;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn main() {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    // Retrieve the latest comic ID.
    let latest_id = xkcd::comics::latest(&client).unwrap().num;

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

        all_comics.push(xkcd::comics::get(&client, i).unwrap());
        println!("Read comic: {}", i);
    }
}
