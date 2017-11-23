// Copyright (c) 2016-2017 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Retrieves information on XKCD comics.
//!
//! For more information, see [XKCD's API
//! documentation](https://xkcd.com/json.html).

use error::Error;
use futures::Future;
use model::XkcdResponse;
use super::{XkcdRequestSender, parse_xkcd_response};

/// Retrieves information regarding a specified comic from the XKCD website.
pub fn get<'a, R>(client: &'a R, id: u32) -> Box<'a + Future<Item = XkcdResponse, Error = Error>>
    where R: XkcdRequestSender,
{
    let method = format!("{}/info.0.json", id);
    let res = client.send(&method)
        .map_err(From::from)
        .and_then(|res| {
            parse_xkcd_response(&res)
        })
        .map_err(From::from);

    Box::new(res)
}

/// Retrieves information regarding the latest comic from the XKCD website.
pub fn latest<'a, R>(client: &'a R) -> Box<'a + Future<Item = XkcdResponse, Error = Error>>
    where R: XkcdRequestSender,
{
    let res = client.send("info.0.json")
        .map_err(From::from)
        .and_then(|res| {
            parse_xkcd_response(&res)
        })
        .map_err(From::from);

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::{get, latest};
    use test_helpers::MockXkcdRequestSender;
    use tokio_core::reactor::Core;
    use util::read_sample_data_from_path;

    #[test]
    fn test_get_comic_ok_response() {
        let mut core = Core::new().unwrap();

        let response = read_sample_data_from_path("tests/sample-data/example.json");
        let client = MockXkcdRequestSender::respond_with(response);
        let result = core.run(get(&client, 1572)).unwrap();
        assert_eq!(result.num, 1572);
    }

    #[test]
    fn test_latest_comic_ok_response() {
        let mut core = Core::new().unwrap();

        let response = read_sample_data_from_path("tests/sample-data/example.json");
        let client = MockXkcdRequestSender::respond_with(response);
        let result = core.run(latest(&client)).unwrap();
        assert_eq!(result.num, 1572);
    }
}
