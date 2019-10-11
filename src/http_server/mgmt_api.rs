use std::sync::Arc;

use warp::Filter;

use crate::storage::StorageApi;

pub fn routes(
    storage_api: Arc<StorageApi>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let index = warp::path::end().and(warp::get2()).map({
        let storage_api = Arc::clone(&storage_api);
        move || warp::reply::json({ &storage_api.ids() })
    });

    let get_by_id = path!(u64).and(warp::get2()).map({
        let storage_api = Arc::clone(&storage_api);
        move |id: u64| {
            warp::reply::json({
                let (rq_opt, rs_opt) = storage_api.get_by_id(id);
                &json!({"id": id, "rq": rq_opt, "rs": rs_opt})
            })
        }
    });

    let all = path!("dump-state").and(warp::get2()).map({
        let storage_api = Arc::clone(&storage_api);
        move || warp::reply::json(&storage_api.get_data())
    });

    all.or(index).or(get_by_id)
}
