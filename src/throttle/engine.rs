use std::time::{Duration, Instant};
use tokio::time::sleep;

use super::ThrottleConfig;

/// Applies throttling to byte streams based on configured bandwidth limits
pub struct ThrottleEngine {
    config: ThrottleConfig,
}

impl ThrottleEngine {
    pub fn new(config: ThrottleConfig) -> Self {
        Self { config }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Calculates the delay needed to simulate the configured bandwidth
    pub fn calculate_delay(&self, byte_count: u64, is_upload: bool) -> Duration {
        if !self.config.enabled {
            return Duration::ZERO;
        }
        let bps = if is_upload {
            self.config.upload_bps
        } else {
            self.config.download_bps
        };
        match bps {
            Some(limit) if limit > 0 => {
                let seconds = byte_count as f64 / limit as f64;
                Duration::from_secs_f64(seconds)
            }
            _ => Duration::ZERO,
        }
    }

    /// Applies the configured latency delay (call once per request)
    pub async fn apply_latency(&self) {
        if let Some(latency) = self.config.latency {
            if self.config.enabled {
                sleep(latency).await;
            }
        }
    }

    /// Throttles a chunk of data, sleeping as needed to simulate bandwidth
    pub async fn throttle_chunk(&self, chunk: &[u8], is_upload: bool, chunk_start: Instant) {
        if !self.config.enabled {
            return;
        }
        let delay = self.calculate_delay(chunk.len() as u64, is_upload);
        let elapsed = chunk_start.elapsed();
        if delay > elapsed {
            sleep(delay - elapsed).await;
        }
    }

    pub fn config(&self) -> &ThrottleConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: ThrottleConfig) {
        self.config = config;
    }
}
