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

use error::Result;
use model::XkcdResponse;
use super::{XkcdRequestSender, parse_xkcd_response};

/// Retrieves information regarding a specified comic from the XKCD website.
pub fn get<R>(client: &R, id: u32) -> Result<XkcdResponse>
    where R: XkcdRequestSender,
{
    let method = format!("{}/info.0.json", id);
    let response = client.send(&method)?;
    parse_xkcd_response(&response)
}

/// Retrieves information regarding the latest comic from the XKCD website.
pub fn latest<R>(client: &R) -> Result<XkcdResponse>
    where R: XkcdRequestSender,
{
    let response = client.send("info.0.json")?;
    parse_xkcd_response(&response)
}

#[cfg(test)]
mod tests {
    use super::{get, latest};
    use super::super::test_helpers::MockXkcdRequestSender;

    #[test]
    fn test_get_comic_ok_response() {
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
        let result = get(&client, 1572).unwrap();
        assert_eq!(result.num, 1572);
    }

    #[test]
    fn test_latest_comic_ok_response() {
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
        let result = latest(&client).unwrap();
        assert_eq!(result.num, 1572);
    }
}
