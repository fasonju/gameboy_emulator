#[derive(Debug, thiserror::Error)]
pub enum DeltaTimeError {
    #[error("DeltaTime has no start time yet")]
    NoStartTime
}