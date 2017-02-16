// Copyright (c) 2016 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]
#![deny(non_camel_case_types)]
#![deny(warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! A library providing Rust bindings for the XKCD web API.
//!
//! The library provides a `XkcdRequestSender` trait which can be implemented by
//! various request senders. These implementations may then be used to execute
//! requests to the API.
//!
//! If the `hyper` feature is enabled during compilation, then this library
//! provides an implementation of the `XkcdRequestSender` trait for the
//! `hyper::Client` of the [`hyper`](https://github.com/hyperium/hyper) library.
//!
//! Response bodies are deserialized from JSON into structs via the
//! [`serde_json`](https://github.com/serde-rs/json) library.

#[cfg(feature = "hyper")]
extern crate hyper;

#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod error;

pub use self::error::{Error, HttpRequestError, HttpRequestResult, Result};
pub use self::model::XkcdResponse;

pub mod comics;
pub mod model;
pub mod random;

use serde::Deserialize;
use std::fmt::Debug;

fn parse_xkcd_response<T>(response: &str) -> Result<T>
    where T: Debug + Deserialize,
{
    let parsed_response = serde_json::from_str(response)?;
    trace!("Parsed response: {:?}", parsed_response);
    Ok(parsed_response)
}

/// Functionality for sending requests to the XKCD API via HTTPS.
///
/// Should be implemented for clients to send requests to the XKCD API.
pub trait XkcdRequestSender {
    /// Performs an API call to the XKCD web API.
    fn send(&self, method: &str) -> HttpRequestResult<String>;
}

#[cfg(feature = "hyper")]
mod hyper_support {

    use error::{HttpRequestError, HttpRequestResult};
    use hyper;
    use hyper::status::StatusCode;
    use std::io::Read;
    use super::XkcdRequestSender;
    use url::Url;

    impl XkcdRequestSender for hyper::Client {
        fn send(&self, method: &str) -> HttpRequestResult<String> {
            let url_string = format!("https://xkcd.com/{}", method);
            let url = url_string.parse::<Url>().expect("Unable to parse URL");

            trace!("Sending query to url: {}", url);
            let mut response = self.get(url.clone()).send()?;

            trace!("Status code: {}", response.status);
            // Ensure we got a valid status code.
            if let StatusCode::NotFound = response.status {
                return Err(HttpRequestError::not_found(url));
            }

            let mut result = String::new();
            response.read_to_string(&mut result)?;
            trace!("Query result: {}", result);

            Ok(result)
        }
    }

    impl From<hyper::Error> for HttpRequestError {
        fn from(error: hyper::Error) -> HttpRequestError {
            match error {
                hyper::Error::Io(e) => HttpRequestError::Io(e),
                e => HttpRequestError::Other(Box::new(e)),
            }
        }
    }
}

#[cfg(feature = "hyper")]
pub use hyper_support::*;

#[cfg(test)]
mod test_helpers {
    use super::{HttpRequestError, XkcdRequestSender};

    pub struct MockXkcdRequestSender {
        response: String,
    }

    impl MockXkcdRequestSender {
        pub fn respond_with<S: Into<String>>(response: S) -> Self {
            MockXkcdRequestSender { response: response.into(), }
        }
    }

    impl XkcdRequestSender for MockXkcdRequestSender {
        fn send(&self, _: &str) -> Result<String, HttpRequestError> {
            Ok(self.response.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use model::XkcdResponse;
    use super::parse_xkcd_response;
    use url::Url;

    #[test]
    fn test_parse_xkcd_response() {
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
        let response = parse_xkcd_response::<XkcdResponse>(result).unwrap();
        assert_eq!(response,
                   XkcdResponse {
                       month: 6,
                       num: 1698,
                       link: "".to_owned(),
                       year: 2016,
                       news: "".to_owned(),
                       safe_title: "Theft Quadrants".to_owned(),
                       transcript: "".to_owned(),
                       alt: "TinyURL was the most popular link shortener for long enough that it \
                             made it into a lot of printed publications. I wonder what year the \
                             domain will finally lapse and get picked up by a porn site."
                           .to_owned(),
                       img: "http://imgs.xkcd.com/comics/theft_quadrants.png"
                           .to_owned()
                           .parse::<Url>()
                           .unwrap(),
                       title: "Theft Quadrants".to_owned(),
                       day: 24,
                   });
    }
}
