use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rq {
    at: SystemTime,
    method: String,
    path: String,
    query: Option<String>,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Rq {
    pub fn new() -> Self {
        Self {
            at: SystemTime::now(),
            method: "GET".to_owned(),
            path: "/".to_owned(),
            query: None,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    field_get_copy!(at, at, SystemTime);
    field_get_ref_mut!(at, at_mut, SystemTime);
    field_update!(at, with_at, SystemTime);

    field_get_ref!(method, method, String);
    field_get_ref_mut!(method, method_mut, String);
    field_update!(method, with_method, String);

    field_get_ref!(path, path, String);
    field_get_ref_mut!(path, path_mut, String);
    field_update!(path, with_path, String);

    field_get_opt_ref!(query, query, String);
    field_update_opt_none!(query, without_query);
    field_update_opt_some!(query, with_query, String);

    field_get_ref!(headers, headers, HashMap<String, String>);
    field_get_ref_mut!(headers, headers_mut, HashMap<String, String>);
    field_update!(headers, with_headers, HashMap<String, String>);

    field_get_ref!(body, body, Vec<u8>);
    field_get_ref_mut!(body, body_mut, Vec<u8>);
    field_update!(body, with_body, Vec<u8>);
}
