#[derive(Debug, Fail)]
pub enum StorageFailure {
    #[fail(display = "Unimplemented")]
    Unimplemented,
}
