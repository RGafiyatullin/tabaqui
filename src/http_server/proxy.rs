use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use bytes::Buf;
use futures::prelude::*;

use hyper::Uri;
use warp::body::FullBody;
use warp::http::HeaderMap;
use warp::http::Method;
use warp::http::StatusCode;
use warp::path::FullPath;
use warp::Filter;

use crate::data::Rq;
use crate::data::Rs;
use crate::storage::StorageApi;

pub fn routes(
    storage_api: Arc<StorageApi>,
    backend_base_uri: Uri,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::method()
        .and(warp::path::full())
        .and(
            warp::query::raw()
                .map(Option::<String>::Some)
                .recover(|_| Ok(Option::<String>::None))
                .unify(),
        )
        .and(warp::header::headers_cloned().map(collect_headers))
        .and(warp::body::concat())
        .map(
            |method: Method,
             path: FullPath,
             query_opt: Option<String>,
             headers: HashMap<String, String>,
             body: FullBody| {
                Rq::new()
                    .with_method(method.as_str().to_owned())
                    .with_path(path.as_str().to_owned())
                    .with_query(query_opt)
                    .with_headers(headers)
                    .with_body(body.bytes().to_owned())
            },
        )
        .map({
            let storage_api = Arc::clone(&storage_api);
            move |rq| {
                let id = storage_api.store_request(&rq);
                (id, rq)
            }
        })
        .and_then(move |(id, rq)| {
            query_backend(backend_base_uri.clone(), rq)
                .map(move |rs| (id, rs))
                .map_err(warp_wrap_error)
        })
        .map({
            let storage_api = Arc::clone(&storage_api);
            move |(id, rs)| {
                storage_api.store_response(id, &rs);
                rs
            }
        })
        .map(|rs: Rs| {
            let mut response_builder = warp::http::Response::builder();
            response_builder.status(rs.status());
            for (header_key, header_val) in rs.headers() {
                response_builder.header(header_key, header_val);
            }

            let response = response_builder
                .body(rs.body().clone())
                .expect("Failed to build Response"); // FIXME:

            response
        })
}

fn collect_headers(header_map: HeaderMap) -> HashMap<String, String> {
    header_map
        .into_iter()
        .filter_map(|(name_opt, value)| name_opt.map(move |name| (name.as_str().to_owned(), value)))
        .filter_map(|(name_str, value)| {
            value
                .to_str()
                .ok()
                .map(move |value_str| (name_str, value_str.to_owned()))
        })
        .collect()
}

fn reject<E>(_e: E) -> warp::Rejection {
    warp::reject()
}

fn query_backend(base_uri: Uri, rq: Rq) -> impl Future<Item = Rs, Error = ProxyError> {
    info!("query_backend[ base_uri: {:?}; rq: {:?} ]", base_uri, rq);

    let uri_fut = {
        let scheme = base_uri.scheme().unwrap_or("http");
        let authority = base_uri.authority().unwrap_or("localhost");
        let path = rq.path();
        let query = rq
            .query()
            .map(|q| format!("?{}", q))
            .unwrap_or("".to_owned());
        let uri_str = format!("{}://{}{}{}", scheme, authority, path, query);
        info!("query_backend [uri: {:?}]", uri_str);
        Uri::from_str(&uri_str).map_err(ProxyError::InvalidUri)
    }
    .into_future();

    let method = hyper::Method::from_str(rq.method()).unwrap();
    let body = hyper::Body::from(rq.body().clone());

    let client = hyper::Client::new();

    let mut request_builder = hyper::Request::builder();

    let request_headers = request_builder.headers_mut().unwrap();
    for (k, v) in rq.headers() {
        let header_key = hyper::header::HeaderName::from_str(k).unwrap();
        let header_value = hyper::header::HeaderValue::from_str(v).unwrap();
        request_headers.insert(header_key, header_value);
    }

    let request_fut =
        uri_fut.map(move |uri| request_builder.uri(uri).method(method).body(body).unwrap());
    let response_fut = request_fut
        .and_then(move |request| client.request(request).map_err(ProxyError::HyperError));

    response_fut
        .map(|response| {
            let status = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(header_key, header_val)| {
                    (
                        header_key.as_str().to_owned(),
                        header_val
                            .to_str()
                            .ok()
                            .unwrap_or("[INVALID-UTF8]")
                            .to_owned(),
                    )
                })
                .collect();
            let body = response.into_body();

            let rs = Rs::new().with_status(status).with_headers(headers);
            (rs, body)
        })
        .and_then(|(rs, body)| {
            body.concat2()
                .map_err(ProxyError::HyperError)
                .map(move |body| rs.with_body(body.into_bytes().into_iter().collect()))
        })
}

#[derive(Debug, Fail)]
enum ProxyError {
    #[fail(display = "ProxyError::InvalidUri: {}", _0)]
    InvalidUri(#[cause] hyper::http::uri::InvalidUri),

    #[fail(display = "ProxyError::HyperError: {}", _0)]
    HyperError(#[cause] hyper::Error),

    #[fail(display = "ProxyError::Unimplemented")]
    Unimplemented,
}

pub fn warp_wrap_error<E: Into<failure::Error>>(err: E) -> warp::Rejection {
    let failure_err: failure::Error = err.into();
    warp::reject::custom(failure_err)
}
