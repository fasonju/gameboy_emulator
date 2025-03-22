use std::time::Duration;

use super::errors::DeltaTimeError;

pub struct DeltaTime {
    last_time: Option<std::time::Instant>,
}

/// DeltaTime is a utility for measuring the time between updates
impl DeltaTime {
    /// create a new DeltaTime
    pub fn new() -> Self {
        Self { last_time: None }
    }

    /// update the start time
    pub fn update(&mut self) {
        self.last_time = Some(std::time::Instant::now());
    }

    /// reset the start time
    pub fn reset(&mut self) {
        self.last_time = None;
    }

    /// get the duration since the last update
    pub fn diff(&self) -> Result<Duration, DeltaTimeError> {
        if let Some(last_time) = self.last_time {
            let now = std::time::Instant::now();
            let duration = now.duration_since(last_time);
            Ok(duration)
        } else {
            Err(DeltaTimeError::NoStartTime)
        }
    }

    /// wait until the duration has passed since the last update
    pub fn wait(&self, duration: Duration) {
        if let Ok(diff) = self.diff() {
            if diff < duration {
                std::thread::sleep(duration - diff);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut dt = DeltaTime::new();
        dt.update();
        assert!(dt.last_time.is_some());
    }

    #[test]
    fn test_reset() {
        let mut dt = DeltaTime::new();
        dt.update();
        dt.reset();
        assert!(dt.last_time.is_none());
    }

    #[test]
    fn test_diff() {
        let mut dt = DeltaTime::new();
        dt.update();
        std::thread::sleep(Duration::from_millis(100));
        let diff = dt.diff().unwrap();
        assert!(diff >= Duration::from_millis(100));
    }

    #[test]
    fn test_wait() {
        let mut dt = DeltaTime::new();
        dt.update();
        let duration = Duration::from_millis(100);
        dt.wait(duration);
        let diff = dt.diff().unwrap();
        assert!(diff >= duration);
    }
}
