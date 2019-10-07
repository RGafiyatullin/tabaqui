use std::collections::HashMap;
use std::collections::VecDeque;

use crate::data::Rq;
use crate::data::Rs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageInner {
    pub(super) cap: usize,
    pub(super) seq: u64,
    pub(super) ids: VecDeque<u64>,
    #[serde(rename = "requests")]
    pub(super) rqs: HashMap<u64, Rq>,
    #[serde(rename = "responses")]
    pub(super) rss: HashMap<u64, Rs>,
}

impl StorageInner {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            seq: 0,
            ids: VecDeque::with_capacity(cap + 1),
            rqs: HashMap::with_capacity(cap + 1),
            rss: HashMap::with_capacity(cap + 1),
        }
    }
}
