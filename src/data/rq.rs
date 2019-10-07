
use super::Method;
use super::Headers;
use super::Body;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rq {
    method: Method,
    headers: Headers,
    body: Option<Body>,
}

impl Rq {
    field_get_ref!(method, method, Method);
    field_get_ref_mut!(method, method_mut, Method);
    field_update!(method, with_method, Method);

    field_get_ref!(headers, headers, Headers);
    field_get_ref_mut!(headers, headers_mut, Headers);
    field_update!(headers, with_headers, Headers);
}
