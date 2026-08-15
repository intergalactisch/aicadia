#[cfg(test)]
use std::collections::VecDeque;
#[cfg(test)]
use std::sync::Mutex;

use rand::{TryRngCore, rngs::OsRng};

pub(super) const PLACE_ACTIVITY_WINDOW: i64 = 48;
const P_MAX: f64 = 0.5;
const P_MIN: f64 = 0.1;
const HALF_LIFE: f64 = 6.0;

pub(super) struct ChancePolicy;

impl ChancePolicy {
    pub(super) fn probability(discovery_count: u32) -> f64 {
        P_MIN + (P_MAX - P_MIN) * 2_f64.powf(-(f64::from(discovery_count) / HALF_LIFE))
    }
}

pub(in crate::world) trait ChanceSource: Send + Sync {
    fn draw(&self) -> Result<f64, ()>;
}

pub(in crate::world) struct OsChance;

impl ChanceSource for OsChance {
    fn draw(&self) -> Result<f64, ()> {
        let value = OsRng.try_next_u64().map_err(|_| ())? >> 11;
        Ok((value as f64) * (1.0 / 9_007_199_254_740_992.0))
    }
}

#[cfg(test)]
pub(in crate::world) struct ScriptedChance {
    draw: Mutex<VecDeque<f64>>,
}

#[cfg(test)]
impl ScriptedChance {
    pub(in crate::world) fn new(draw: Vec<f64>) -> Self {
        Self {
            draw: Mutex::new(draw.into()),
        }
    }
}

#[cfg(test)]
impl ChanceSource for ScriptedChance {
    fn draw(&self) -> Result<f64, ()> {
        self.draw.lock().map_err(|_| ())?.pop_front().ok_or(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn probability_has_exact_bounds_and_monotonic_decay() {
        assert_eq!(ChancePolicy::probability(0), P_MAX);
        let values = (0..=48).map(ChancePolicy::probability).collect::<Vec<_>>();
        assert!(values.windows(2).all(|pair| pair[0] > pair[1]));
        assert!(values.iter().all(|value| *value > P_MIN && *value <= P_MAX));
        assert!((ChancePolicy::probability(6) - 0.3).abs() < f64::EPSILON);
    }
}
