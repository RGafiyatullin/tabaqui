
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Method(String);

impl Default for Method {
    fn default() -> Self { Self("GET".to_owned()) }
}


