use std::sync::Arc;

use warp::Filter;

use crate::storage::StorageApi;

pub fn routes(
    storage_api: Arc<StorageApi>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let all = warp::path::end().map({ move || warp::reply::json(&storage_api.get_data()) });

    all
}
