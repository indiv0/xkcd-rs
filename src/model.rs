// Copyright (c) 2016 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Struct and enum definitions of values in the XKCD API model.
//!
//! For more information, see [XKCD's API
//! documentation](https://xkcd.com/json.html).

#[cfg(feature = "nightly")]
include!("model.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/model.rs"));

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use super::XkcdResponse;
    use super::super::parse_xkcd_response;

    #[test]
    fn test_parse_xkcd_response() {
        let result = r#"
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
        "#;
        let response = from_str::<XkcdResponse>(&result).unwrap();
        assert_eq!(response, XkcdResponse {
            month: 9,
            num: 1572,
            link: "http://goo.gl/forms/pj0OhX6wfO".to_owned(),
            year: 2015,
            news: "".to_owned(),
            safe_title: "xkcd Survey".to_owned(),
            transcript: "Introducing the XKCD SURVEY! A search for weird correlations.\n\nNOTE: This survey is anonymous, but all responses will be posted publicly so people can play with the data.\n\nClick here to take the survey.\n\nhttp:\n\ngoo.gl\nforms\nlzZr7P9Qlm\n\nOr click here, or here. The whole comic is a link because I still haven't gotten the hang of HTML imagemaps.\n\n{{Title text: The xkcd Survey: Big Data for a Big Planet}}".to_owned(),
            alt: "The xkcd Survey: Big Data for a Big Planet".to_owned(),
            img: "http://imgs.xkcd.com/comics/xkcd_survey.png".to_owned(),
            title: "xkcd Survey".to_owned(),
            day: 1,
        });
    }

    #[test]
    fn test_parse_xkcd_response_no_link() {
        let result = r#"
            {
                "month": "6",
                "num": 1698,
                "link": "",
                "year": "2016",
                "news": "",
                "safe_title": "Theft Quadrants",
                "transcript": "",
                "alt": "TinyURL was the most popular link shortener for long enough that it made it into a lot of printed publications. I wonder what year the domain will finally lapse and get picked up by a porn site.",
                "img": "http:\/\/imgs.xkcd.com\/comics\/theft_quadrants.png",
                "title": "Theft Quadrants",
                "day": "24"
            }
        "#;
        let response = parse_xkcd_response::<XkcdResponse>(&result).unwrap();
        assert_eq!(response, XkcdResponse {
            month: 6,
            num: 1698,
            link: "".to_owned(),
            year: 2016,
            news: "".to_owned(),
            safe_title: "Theft Quadrants".to_owned(),
            transcript: "".to_owned(),
            alt: "TinyURL was the most popular link shortener for long enough that it made it into a lot of printed publications. I wonder what year the domain will finally lapse and get picked up by a porn site.".to_owned(),
            img: "http://imgs.xkcd.com/comics/theft_quadrants.png".to_owned(),
            title: "Theft Quadrants".to_owned(),
            day: 24,
        });
    }
}
