
use super::Status;
use super::Headers;
use super::Body;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rs {
    status: Status,
    headers: Headers,
    body: Body,
}

impl Rs {
    field_get_copy!(status, status, Status);
    field_get_ref_mut!(status, status_mut, Status);
    field_update!(status, with_status, Status);

    field_get_ref!(headers, headers, Headers);
    field_get_ref_mut!(headers, headers_mut, Headers);
    field_update!(headers, with_headers, Headers);

    field_get_ref!(body, body, Body);
    field_get_ref_mut!(body, body_mut, Body);
    field_update!(body, with_body, Body);
}
