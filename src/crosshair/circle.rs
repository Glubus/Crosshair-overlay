use super::{Crosshair, CrosshairRenderer};
use crate::config::effects::Effects;

/// Renderer pour le style Circle - cercle plein ou contour
pub struct CircleCrosshair;

impl CrosshairRenderer for CircleCrosshair {
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
        let radius = crosshair.size as i32;
        let gap = crosshair.gap as f32;

        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - (center_x as i32 + shake_x as i32);
                let dy = y as i32 - (center_y as i32 + shake_y as i32);
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                
                if crosshair.filled_circle {
                    // Mode cercle plein avec gap au centre
                    let outer_radius = radius as f32;
                    
                    if distance >= gap && distance <= outer_radius {
                        // Outline pour le cercle plein
                        if crosshair.outline.enabled {
                            let outline_inner = gap - crosshair.outline.thickness as f32;
                            let outline_outer = outer_radius + crosshair.outline.thickness as f32;
                            
                            if (distance >= outline_inner.max(0.0) && distance <= gap) || 
                               distance >= outline_outer {
                                buffer[y * width + x] = crosshair.get_outline_color();
                            } else {
                                buffer[y * width + x] = color;
                            }
                        } else {
                            buffer[y * width + x] = color;
                        }
                    }
                } else {
                    // Mode cercle contour (creux) avec gap au centre
                    let inner_radius = ((radius - crosshair.thickness as i32).max(gap as i32)).max(0) as f32;
                    let outer_radius = radius as f32;
                    
                    if distance >= inner_radius && distance <= outer_radius {
                        // Outline pour le cercle creux
                        if crosshair.outline.enabled {
                            let outline_inner = inner_radius - crosshair.outline.thickness as f32;
                            let outline_outer = outer_radius + crosshair.outline.thickness as f32;
                            
                            if distance <= outline_inner.max(0.0) || distance >= outline_outer {
                                buffer[y * width + x] = crosshair.get_outline_color();
                            } else {
                                buffer[y * width + x] = color;
                            }
                        } else {
                            buffer[y * width + x] = color;
                        }
                    }
                }
            }
        }
    }
} 