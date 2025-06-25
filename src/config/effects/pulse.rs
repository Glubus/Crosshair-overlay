use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseEffect {
    pub enabled: bool,
    pub speed: f32,        // Vitesse de pulsation (cycles par seconde)
    pub min_alpha: f32,    // Alpha minimum (0.0-1.0)
    pub max_alpha: f32,    // Alpha maximum (0.0-1.0)
}

impl Default for PulseEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 2.0,      // 2 pulsations par seconde
            min_alpha: 0.3,  // Minimum 30% d'opacité
            max_alpha: 1.0,  // Maximum 100% d'opacité
        }
    }
}

impl PulseEffect {
    /// Applique l'effet pulse à une couleur selon le temps
    pub fn apply(&self, color: u32, time: f32) -> u32 {
        if !self.enabled {
            return color;
        }

        // Calcul du facteur de pulsation (oscillation sinusoïdale)
        let pulse_factor = (time * self.speed * 2.0 * std::f32::consts::PI).sin();
        let pulse_factor = (pulse_factor + 1.0) / 2.0; // Normaliser entre 0 et 1
        
        // Interpolation entre min_alpha et max_alpha
        let alpha_multiplier = self.min_alpha + (self.max_alpha - self.min_alpha) * pulse_factor;
        
        // Extraire les composants RGBA
        let original_alpha = ((color >> 24) & 0xFF) as f32 / 255.0;
        let r = (color >> 16) & 0xFF;
        let g = (color >> 8) & 0xFF;
        let b = color & 0xFF;
        
        // Appliquer le multiplicateur d'alpha
        let new_alpha = (original_alpha * alpha_multiplier * 255.0) as u32;
        
        (new_alpha << 24) | (r << 16) | (g << 8) | b
    }
} 