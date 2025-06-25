use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShakeEffect {
    pub enabled: bool,
    pub intensity: f32,    // Intensité du tremblement (en pixels)
    pub speed: f32,        // Vitesse du tremblement (cycles par seconde)
}

impl Default for ShakeEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 2.0,  // 2 pixels de tremblement
            speed: 10.0,     // 10 cycles par seconde
        }
    }
}

impl ShakeEffect {
    /// Calcule l'offset de tremblement selon le temps
    pub fn get_offset(&self, time: f32) -> (f32, f32) {
        if !self.enabled {
            return (0.0, 0.0);
        }

        // Utiliser des fréquences légèrement différentes pour X et Y
        // pour un mouvement plus naturel
        let x_freq = self.speed * 2.0 * std::f32::consts::PI;
        let y_freq = self.speed * 1.7 * std::f32::consts::PI; // Fréquence légèrement différente
        
        let shake_x = (time * x_freq).sin() * self.intensity;
        let shake_y = (time * y_freq).cos() * self.intensity; // Cosinus pour un mouvement différent
        
        (shake_x, shake_y)
    }
} 