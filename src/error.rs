// Copyright (c) 2016 Nikita Pekin and the xkcd_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

use serde_json;
use url::Url;

/// A convenient alias type for results for `xkcd`.
pub type Result<T> = StdResult<T, Error>;

/// Represents errors which occur while using the XKCD API.
#[derive(Debug)]
pub enum Error {
    /// Error sending a HTTP request to the XKCD API.
    HttpRequest(HttpRequestError),
    /// An IO error was encountered.
    Io(io::Error),
    /// Error while serializing or deserializing JSON.
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpRequest(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::Json(ref e) => e.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpRequest(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::HttpRequest(ref e) => e.cause(),
            Error::Io(ref e) => e.cause(),
            Error::Json(ref e) => e.cause(),
        }
    }
}

impl From<HttpRequestError> for Error {
    fn from(error: HttpRequestError) -> Error {
        Error::HttpRequest(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}

// Implement `PartialEq` manually, since `std::io::Error` does not implement it.
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use self::Error::*;

        match (self, other) {
            (&HttpRequest(_), &HttpRequest(_)) |
                (&Io(_), &Io(_)) |
                (&Json(_), &Json(_)) => true,
            _ => false,
        }
    }
}

/// A convenient alias type for results of HTTP requests.
pub type HttpRequestResult<T> = StdResult<T, HttpRequestError>;

/// Represents errors which occur when sending an HTTP request to the XKCD API.
#[derive(Debug)]
pub enum HttpRequestError {
    /// An error occuring during network IO operations.
    Io(io::Error),
    /// Error which occurs when a resource was not found at the specified URL.
    NotFound(Url),
    /// Any other error occuring during an HTTP request.
    Other(Box<StdError>),
}

impl HttpRequestError {
    /// Create an error from when the specified resource was not found.
    pub fn not_found(url: Url) -> HttpRequestError {
        HttpRequestError::NotFound(url)
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpRequestError::Io(ref e) => e.fmt(f),
            HttpRequestError::Other(ref e) => e.fmt(f),
            ref other => write!(f, "{}", other.description()),
        }
    }
}

impl StdError for HttpRequestError {
    fn description(&self) -> &str {
        match *self {
            HttpRequestError::Io(ref e) => e.description(),
            HttpRequestError::NotFound(_) => "comic was not found",
            HttpRequestError::Other(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            HttpRequestError::Io(ref e) => e.cause(),
            HttpRequestError::Other(ref e) => e.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for HttpRequestError {
    fn from(error: io::Error) -> HttpRequestError {
        HttpRequestError::Io(error)
    }
}
