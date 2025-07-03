use super::{Crosshair, CrosshairRenderer};
use crate::config::effects::Effects;

/// Renderer pour le style Square - carré simple avec gap au centre
pub struct SquareCrosshair;

impl CrosshairRenderer for SquareCrosshair {
    fn draw(&self, crosshair: &Crosshair, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32) {
        // Appliquer l'effet rainbow si activé
        let mut color = if effects.rainbow.enabled {
            effects.rainbow.get_color(time, crosshair.alpha)
        } else {
            crosshair.get_color()
        };

        // Appliquer l'effet pulse si activé
        if effects.pulse.enabled {
            color = effects.pulse.apply(color, time);
        }

        let outline_color = crosshair.get_outline_color();
        
        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);
        
        for y in 0..height {
            for x in 0..width {
                // Appliquer le shake en décalant les coordonnées
                let adjusted_x = (x as f32 - shake_x) as usize;
                let adjusted_y = (y as f32 - shake_y) as usize;
                
                if self.is_on_square(crosshair, adjusted_x, adjusted_y, width, height) {
                    let mut final_color = color;
                    
                    // Dessiner l'outline si activé
                    if crosshair.outline.enabled {
                        // Vérifier si c'est un pixel de bordure
                        let is_edge = !self.is_on_square(crosshair, adjusted_x.saturating_sub(1), adjusted_y, width, height) ||
                                     !self.is_on_square(crosshair, adjusted_x + 1, adjusted_y, width, height) ||
                                     !self.is_on_square(crosshair, adjusted_x, adjusted_y.saturating_sub(1), width, height) ||
                                     !self.is_on_square(crosshair, adjusted_x, adjusted_y + 1, width, height);
                        
                        if is_edge {
                            final_color = outline_color;
                        }
                    }
                    
                    buffer[y * width + x] = final_color;
                }
            }
        }
    }
}

impl SquareCrosshair {
    /// Vérifie si un point fait partie du carré simple avec gap
    fn is_on_square(&self, crosshair: &Crosshair, x: usize, y: usize, width: usize, height: usize) -> bool {
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        
        // Appliquer la rotation inverse pour tester dans l'espace non-rotaté
        let (rot_x, rot_y) = crosshair.rotate_point(x as f32, y as f32, center_x, center_y);
        
        let dx = (rot_x - center_x).abs();
        let dy = (rot_y - center_y).abs();
        let size = crosshair.size as f32;
        let gap = crosshair.gap as f32;
        
        // Zone vide au centre (gap)
        if dx <= gap && dy <= gap {
            return false;
        }
        
        // Dans les limites du carré
        dx <= size && dy <= size
    }
} 