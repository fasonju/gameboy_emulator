#[derive(Debug, thiserror::Error)]
pub enum DeltaTimeError {
    #[error("DeltaTime has no time to compare against")]
    NoStartTime
}