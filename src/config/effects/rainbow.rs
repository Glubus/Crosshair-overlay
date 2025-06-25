use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RainbowEffect {
    pub enabled: bool,
    pub speed: f32,        // Vitesse de rotation des couleurs (cycles par seconde)
    pub saturation: f32,   // Saturation des couleurs (0.0-1.0)
    pub brightness: f32,   // Luminosité des couleurs (0.0-1.0)
}

impl Default for RainbowEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 1.0,        // 1 cycle complet par seconde
            saturation: 1.0,   // Saturation maximale
            brightness: 1.0,   // Luminosité maximale
        }
    }
}

impl RainbowEffect {
    /// Génère une couleur rainbow selon le temps
    pub fn get_color(&self, time: f32, base_alpha: f32) -> u32 {
        if !self.enabled {
            return 0; // Retourner transparent si désactivé
        }

        // Calcul de la teinte basée sur le temps
        let hue = (time * self.speed * 360.0) % 360.0;
        
        // Conversion HSV vers RGB
        let (r, g, b) = self.hsv_to_rgb(hue, self.saturation, self.brightness);
        
        // Appliquer l'alpha
        let alpha = (base_alpha * 255.0) as u32;
        
        (alpha << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Convertit HSV vers RGB
    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> (f32, f32, f32) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (
            ((r_prime + m) * 255.0),
            ((g_prime + m) * 255.0),
            ((b_prime + m) * 255.0),
        )
    }
} 