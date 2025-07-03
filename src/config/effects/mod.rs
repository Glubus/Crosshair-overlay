pub mod pulse;
pub mod shake;
pub mod rainbow;
pub mod mouse;

pub use pulse::PulseEffect;
pub use shake::ShakeEffect;
pub use rainbow::RainbowEffect;
pub use mouse::MouseEffect;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub pulse: PulseEffect,
    pub shake: ShakeEffect,
    pub rainbow: RainbowEffect,
    pub mouse: MouseEffect,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            pulse: PulseEffect::default(),
            shake: ShakeEffect::default(),
            rainbow: RainbowEffect::default(),
            mouse: MouseEffect::default(),
        }
    }
}

impl Effects {
    /// Vérifie si des effets animés sont activés (qui nécessitent un redraw continu)
    pub fn has_animated_effects(&self) -> bool {
        self.pulse.enabled || self.shake.enabled || self.rainbow.enabled
        // Note: mouse.enabled ne nécessite pas d'animation continue, seulement lors des clics
    }
    
    /// Vérifie si des effets de souris sont activés (nécessitent un redraw lors des clics)
    pub fn has_mouse_effects(&self) -> bool {
        self.mouse.enabled
    }
} 