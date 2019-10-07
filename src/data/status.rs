
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status(u16);

impl Default for Status {
    fn default() -> Self {
        Self(200)
    }
}
