
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body(Vec<u8>);

impl Body {
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl Default for Body {
    fn default() -> Self { Self::empty() }
}
