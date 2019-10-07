use std::sync::Arc;
use std::sync::RwLock;

use json_utils::json::JsValue;

use crate::data;

use super::StorageInner;

#[derive(Clone)]
pub struct StorageApi {
    max_items: usize,
    inner: Arc<RwLock<StorageInner>>,
}

impl StorageApi {
    pub fn new(max_items: usize) -> Self {
        let inner = StorageInner::new(max_items);
        let inner = RwLock::new(inner);
        let inner = Arc::new(inner);

        Self { max_items, inner }
    }

    pub fn get_data(&self) -> JsValue {
        let inner = self.inner.read().expect("Failed to r-lock the state");
        serde_json::to_value(&*inner).expect("Failed to serialize the state")
    }

    pub fn store_request(&self, rq: &data::Rq) -> u64 {
        let mut inner = self.inner.write().expect("Failed to w-lock the state");
        let id = inner.seq;
        inner.seq = id + 1;

        inner.ids.push_back(id);
        inner.rqs.insert(id, rq.clone());

        if inner.ids.len() > inner.cap {
            if let Some(id_to_remove) = inner.ids.pop_front() {
                inner.rqs.remove(&id_to_remove);
                inner.rss.remove(&id_to_remove);
            }
        }

        id
    }
    pub fn store_response(&self, id: u64, rs: &data::Rs) {
        let mut inner = self.inner.write().expect("Faild to w-lock the state");
        inner.rss.insert(id, rs.clone());
    }
}
