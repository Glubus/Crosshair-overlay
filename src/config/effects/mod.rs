pub mod pulse;
pub mod shake;
pub mod rainbow;

pub use pulse::PulseEffect;
pub use shake::ShakeEffect;
pub use rainbow::RainbowEffect;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub pulse: PulseEffect,
    pub shake: ShakeEffect,
    pub rainbow: RainbowEffect,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            pulse: PulseEffect::default(),
            shake: ShakeEffect::default(),
            rainbow: RainbowEffect::default(),
        }
    }
}

impl Effects {
    /// Vérifie si des effets animés sont activés
    pub fn has_animated_effects(&self) -> bool {
        self.pulse.enabled || self.shake.enabled || self.rainbow.enabled
    }
} 