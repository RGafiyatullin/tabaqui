use std::sync::Arc;

use hyper::Uri;
use warp::Filter;

use crate::storage::StorageApi;

mod mgmt_api;
mod proxy;

pub fn routes(
    storage_api: Arc<StorageApi>,
    backend_root_uri: Uri,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let mgmt = warp::path("__mgmt").and(mgmt_api::routes(Arc::clone(&storage_api)));
    let proxy = proxy::routes(Arc::clone(&storage_api), backend_root_uri);

    mgmt.or(proxy)
}
