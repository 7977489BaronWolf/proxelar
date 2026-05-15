use std::time::Duration;

/// Throttle preset profiles for common network conditions
#[derive(Debug, Clone, PartialEq)]
pub enum ThrottleProfile {
    None,
    Slow3G,
    Fast3G,
    Dsl,
    Custom,
}

/// Configuration for bandwidth throttling simulation
#[derive(Debug, Clone)]
pub struct ThrottleConfig {
    pub profile: ThrottleProfile,
    /// Maximum bytes per second (upload)
    pub upload_bps: Option<u64>,
    /// Maximum bytes per second (download)
    pub download_bps: Option<u64>,
    /// Additional artificial latency per request
    pub latency: Option<Duration>,
    pub enabled: bool,
}

impl ThrottleConfig {
    pub fn new() -> Self {
        Self {
            profile: ThrottleProfile::None,
            upload_bps: None,
            download_bps: None,
            latency: None,
            enabled: false,
        }
    }

    pub fn with_profile(profile: ThrottleProfile) -> Self {
        let mut cfg = Self::new();
        match profile {
            ThrottleProfile::Slow3G => {
                cfg.upload_bps = Some(50_000);
                cfg.download_bps = Some(50_000);
                cfg.latency = Some(Duration::from_millis(400));
                cfg.enabled = true;
            }
            ThrottleProfile::Fast3G => {
                cfg.upload_bps = Some(750_000);
                cfg.download_bps = Some(1_500_000);
                cfg.latency = Some(Duration::from_millis(100));
                cfg.enabled = true;
            }
            ThrottleProfile::Dsl => {
                cfg.upload_bps = Some(512_000);
                cfg.download_bps = Some(2_000_000);
                cfg.latency = Some(Duration::from_millis(20));
                cfg.enabled = true;
            }
            _ => {}
        }
        cfg.profile = profile;
        cfg
    }
}

impl Default for ThrottleConfig {
    fn default() -> Self {
        Self::new()
    }
}
