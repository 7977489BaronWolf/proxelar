#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::throttle::{
        config::{ThrottleConfig, ThrottleProfile},
        engine::ThrottleEngine,
    };

    #[test]
    fn test_default_config_disabled() {
        let cfg = ThrottleConfig::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.profile, ThrottleProfile::None);
    }

    #[test]
    fn test_slow3g_profile() {
        let cfg = ThrottleConfig::with_profile(ThrottleProfile::Slow3G);
        assert!(cfg.enabled);
        assert_eq!(cfg.upload_bps, Some(50_000));
        assert_eq!(cfg.download_bps, Some(50_000));
        assert_eq!(cfg.latency, Some(Duration::from_millis(400)));
    }

    #[test]
    fn test_fast3g_profile() {
        let cfg = ThrottleConfig::with_profile(ThrottleProfile::Fast3G);
        assert!(cfg.enabled);
        assert_eq!(cfg.download_bps, Some(1_500_000));
    }

    #[test]
    fn test_dsl_profile() {
        let cfg = ThrottleConfig::with_profile(ThrottleProfile::Dsl);
        assert!(cfg.enabled);
        assert_eq!(cfg.upload_bps, Some(512_000));
        assert_eq!(cfg.download_bps, Some(2_000_000));
    }

    #[test]
    fn test_engine_disabled_no_delay() {
        let cfg = ThrottleConfig::new();
        let engine = ThrottleEngine::new(cfg);
        let delay = engine.calculate_delay(1_000_000, false);
        assert_eq!(delay, Duration::ZERO);
    }

    #[test]
    fn test_engine_calculate_delay() {
        let cfg = ThrottleConfig::with_profile(ThrottleProfile::Slow3G);
        let engine = ThrottleEngine::new(cfg);
        // 50_000 bytes at 50_000 bps = 1 second
        let delay = engine.calculate_delay(50_000, false);
        assert_eq!(delay, Duration::from_secs(1));
    }

    #[test]
    fn test_engine_set_config() {
        let mut engine = ThrottleEngine::new(ThrottleConfig::new());
        assert!(!engine.is_enabled());
        engine.set_config(ThrottleConfig::with_profile(ThrottleProfile::Fast3G));
        assert!(engine.is_enabled());
    }

    #[tokio::test]
    async fn test_throttle_chunk_no_sleep_when_disabled() {
        let cfg = ThrottleConfig::new();
        let engine = ThrottleEngine::new(cfg);
        let start = Instant::now();
        engine.throttle_chunk(&vec![0u8; 1024], false, start).await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }
}
