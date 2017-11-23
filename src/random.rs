// Copyright (c) 2016-2017 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Retrieves information on a random XKCD comic.

use comics;
use error::Error;
use futures::Future;
use model::XkcdResponse;
use rand::{self, Rng};
use std::ops::Range;
use super::XkcdRequestSender;

/// Retrieves information regarding a random comic from the XKCD website in the
/// specified range.
pub fn random_in_range<'a, R>(client: &'a R, range: Range<u32>) -> Box<'a + Future<Item = XkcdResponse, Error = Error>>
    where R: XkcdRequestSender,
{
    let random_id = rand::thread_rng().gen_range(range.start, range.end);
    trace!("Randomly generated ID: {}", random_id);
    let res = comics::get(client, random_id);

    Box::new(res)
}

/// Retrieves information regarding a random comic from all comics on the XKCD
/// website.
pub fn random<'a, R>(client: &'a R) -> Box<'a + Future<Item = XkcdResponse, Error = Error>>
    where R: XkcdRequestSender,
{
    let res = comics::latest(client)
        .and_then(move |latest_comic| {
            random_in_range(client, (1..latest_comic.num + 1))
        });

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::{random, random_in_range};
    use test_helpers::MockXkcdRequestSender;
    use tokio_core::reactor::Core;

    #[test]
    fn test_random_in_range_comic_ok_response() {
        let mut core = Core::new().unwrap();

        let client = MockXkcdRequestSender::respond_with(r#"
            {
                "month": "9",
                "num": 1572,
                "link": "http:\/\/goo.gl\/forms\/pj0OhX6wfO",
                "year": "2015",
                "news": "",
                "safe_title": "xkcd Survey",
                "transcript": "Introducing the XKCD SURVEY! A search for weird correlations.\n\nNOTE: This survey is anonymous, but all responses will be posted publicly so people can play with the data.\n\nClick here to take the survey.\n\nhttp:\n\ngoo.gl\nforms\nlzZr7P9Qlm\n\nOr click here, or here. The whole comic is a link because I still haven't gotten the hang of HTML imagemaps.\n\n{{Title text: The xkcd Survey: Big Data for a Big Planet}}",
                "alt": "The xkcd Survey: Big Data for a Big Planet",
                "img": "http:\/\/imgs.xkcd.com\/comics\/xkcd_survey.png",
                "title": "xkcd Survey",
                "day": "1"
            }
        "#);
        let result = core.run(random_in_range(&client, (1..1573))).unwrap();
        assert_eq!(result.num, 1572);
    }

    #[test]
    fn test_random_comic_ok_response() {
        let mut core = Core::new().unwrap();

        let client = MockXkcdRequestSender::respond_with(r#"
            {
                "month": "9",
                "num": 1572,
                "link": "http:\/\/goo.gl\/forms\/pj0OhX6wfO",
                "year": "2015",
                "news": "",
                "safe_title": "xkcd Survey",
                "transcript": "Introducing the XKCD SURVEY! A search for weird correlations.\n\nNOTE: This survey is anonymous, but all responses will be posted publicly so people can play with the data.\n\nClick here to take the survey.\n\nhttp:\n\ngoo.gl\nforms\nlzZr7P9Qlm\n\nOr click here, or here. The whole comic is a link because I still haven't gotten the hang of HTML imagemaps.\n\n{{Title text: The xkcd Survey: Big Data for a Big Planet}}",
                "alt": "The xkcd Survey: Big Data for a Big Planet",
                "img": "http:\/\/imgs.xkcd.com\/comics\/xkcd_survey.png",
                "title": "xkcd Survey",
                "day": "1"
            }
        "#);
        let result = core.run(random(&client)).unwrap();
        assert_eq!(result.num, 1572);
    }
}
