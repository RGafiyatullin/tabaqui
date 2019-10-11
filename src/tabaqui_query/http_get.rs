use futures::prelude::*;

use hyper::error::Error as HttpClientError;
use hyper::http::StatusCode;
use hyper::http::Uri;
use serde::de::DeserializeOwned;
use serde_json::Error as JsonParseError;

#[derive(Debug, Fail)]
pub enum HttpGetFailure {
    // #[fail(display = "HttpGetFailure::Unimplemented")]
    // Unimplemented,
    #[fail(display = "HttpGetFailure::HttpClientError: {}", _0)]
    HttpClientError(#[cause] HttpClientError),

    #[fail(display = "HttpGetFailure::HttpResponseError: {}", _0)]
    HttpResponseError(StatusCode),

    #[fail(display = "HttpGetFailure::JsonParseError: {}", _0)]
    JsonParseError(#[cause] JsonParseError),
}

pub fn http_get<T: DeserializeOwned>(uri: Uri) -> impl Future<Item = T, Error = HttpGetFailure> {
    let http_client = hyper::Client::new();

    http_client
        .get(uri)
        .map_err(HttpGetFailure::HttpClientError)
        .map(|response| (response.status(), response))
        .and_then(|(status_code, response)| {
            if status_code.is_success() {
                Ok(response)
            } else {
                Err(HttpGetFailure::HttpResponseError(status_code))
            }
        })
        .and_then(|response| {
            response
                .into_body()
                .concat2()
                .map_err(|err| HttpGetFailure::HttpClientError(err))
        })
        .and_then(|response_body| {
            serde_json::from_slice::<T>(&response_body)
                .map_err(|err| HttpGetFailure::JsonParseError(err))
        })
}
