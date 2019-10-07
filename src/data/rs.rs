use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rs {
    at: SystemTime,
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Rs {
    pub fn new() -> Self {
        Self {
            at: SystemTime::now(),
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    field_get_copy!(at, at, SystemTime);
    field_get_ref_mut!(at, at_mut, SystemTime);
    field_update!(at, with_at, SystemTime);

    field_get_copy!(status, status, u16);
    field_get_ref_mut!(status, status_mut, u16);
    field_update!(status, with_status, u16);

    field_get_ref!(headers, headers, HashMap<String, String>);
    field_get_ref_mut!(headers, headers_mut, HashMap<String, String>);
    field_update!(headers, with_headers, HashMap<String, String>);

    field_get_ref!(body, body, Vec<u8>);
    field_get_ref_mut!(body, body_mut, Vec<u8>);
    field_update!(body, with_body, Vec<u8>);
}
