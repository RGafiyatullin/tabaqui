use futures::future;
use futures::prelude::*;

mod storage_failure;
pub use storage_failure::StorageFailure;

mod storage_inner;
pub use storage_inner::StorageInner;

mod storage_api;
pub use storage_api::StorageApi;

pub fn create(max_items: usize) -> (StorageApi, impl Future<Item = (), Error = StorageFailure>) {
    (StorageApi::new(max_items), future::empty())
}
