// Copyright (c) 2016-2017 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs, non_camel_case_types)]
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

extern crate futures;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate tokio_core;
extern crate url;
extern crate url_serde;

mod error;
#[cfg(test)]
mod util;

pub use self::error::{Error, HttpRequestError, Result};
pub use self::model::XkcdResponse;

pub mod comics;
pub mod model;
pub mod random;

use futures::Future;
use serde::Deserialize;
use std::fmt::Debug;

fn parse_xkcd_response<'de, T>(response: &'de str) -> Result<T>
    where T: Debug + Deserialize<'de>,
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
    fn send<'a>(&'a self, method: &str) -> Box<'a + Future<Item = String, Error = HttpRequestError>>;
}

#[cfg(feature = "hyper")]
mod hyper_support {
    use error::HttpRequestError;
    use futures::{Future, Stream};
    use futures::future::result;
    use hyper::{self, StatusCode, Uri};
    use hyper::client::Connect;
    use std::str::{self, FromStr};
    use super::XkcdRequestSender;
    use url::Url;

    impl<C> XkcdRequestSender for hyper::Client<C>
        where C: Connect,
    {
        fn send<'a>(&'a self, method: &str) -> Box<'a + Future<Item = String, Error = HttpRequestError>> {
            let url_string = format!("https://xkcd.com/{}", method);
            let url = url_string.parse::<Url>().expect("Unable to parse URL");
            let uri = Uri::from_str(url.as_ref()).map_err(hyper::Error::from);

            let res = result(uri)
                .and_then(move |uri| {
                    trace!("Sending query to URI: {}", uri);
                    self.get(uri)
                })
                .map_err(From::from)
                .and_then(|res| {
                    trace!("Response status: {}", res.status());

                    // Ensure we got a valid status code.
                    if let StatusCode::NotFound = res.status() {
                        Err(HttpRequestError::not_found(url))
                    } else {
                        Ok(res)
                    }
                })
                .and_then(|res| res.body().concat2().map_err(From::from))
                .and_then(|body| {
                    str::from_utf8(&body)
                        .map_err(From::from)
                        .map(|string| string.to_string())
                });

            Box::new(res)
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
    use futures::Future;
    use futures::future::result;
    use super::{HttpRequestError, XkcdRequestSender};

    pub struct MockXkcdRequestSender {
        response: String,
    }

    impl MockXkcdRequestSender {
        pub fn respond_with<S: Into<String>>(response: S) -> Self {
            MockXkcdRequestSender { response: response.into() }
        }
    }

    impl XkcdRequestSender for MockXkcdRequestSender {
        fn send(&self, _: &str) -> Box<Future<Item = String, Error = HttpRequestError>> {
            Box::new(result(Ok(self.response.clone())))
        }
    }
}

#[cfg(test)]
mod tests {
    use model::XkcdResponse;
    use super::parse_xkcd_response;
    use url::Url;
    use util::read_sample_data_from_path;

    #[test]
    fn test_parse_xkcd_response() {
        let result = read_sample_data_from_path("tests/sample-data/example.json");
        let response = parse_xkcd_response::<XkcdResponse>(result.as_str()).unwrap();
        assert_eq!(response,
                   XkcdResponse {
                       month: 9,
                       num: 1572,
                       link: "http://goo.gl/forms/pj0OhX6wfO".to_owned(),
                       year: 2015,
                       news: "".to_owned(),
                       safe_title: "xkcd Survey".to_owned(),
                       transcript: "Introducing the XKCD SURVEY! A search for weird correlations.\n\nNOTE: This survey is anonymous, but all responses will be posted publicly so people can play with the data.\n\nClick here to take the survey.\n\nhttp:\n\ngoo.gl\nforms\nlzZr7P9Qlm\n\nOr click here, or here. The whole comic is a link because I still haven't gotten the hang of HTML imagemaps.\n\n{{Title text: The xkcd Survey: Big Data for a Big Planet}}".to_owned(),
                       alt: "The xkcd Survey: Big Data for a Big Planet".to_owned(),
                       img: "http://imgs.xkcd.com/comics/xkcd_survey.png".to_owned().parse::<Url>().unwrap(),
                       title: "xkcd Survey".to_owned(),
                       day: 1,
                   }
        );
    }
}
