use super::{Crosshair, CrosshairRenderer};
use crate::config::effects::Effects;

/// Renderer pour le style Dot - juste un point central circulaire
pub struct DotCrosshair;

impl CrosshairRenderer for DotCrosshair {
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

        let center_x = width / 2;
        let center_y = height / 2;
        let radius = crosshair.size as f32;

        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);

        // Dessiner un cercle plein centré
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - (center_x as f32 + shake_x);
                let dy = y as f32 - (center_y as f32 + shake_y);
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    // Outline pour le dot
                    if crosshair.outline.enabled && distance > radius - crosshair.outline.thickness as f32 {
                        buffer[y * width + x] = crosshair.get_outline_color();
                    } else {
                        buffer[y * width + x] = color;
                    }
                }
            }
        }
    }
} 