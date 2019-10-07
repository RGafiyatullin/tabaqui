#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Headers {}

impl Headers {
    pub fn empty() -> Self {
        Self {}
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::empty()
    }
}
