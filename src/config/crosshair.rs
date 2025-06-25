use serde::{Deserialize, Serialize};
use crate::config::effects::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crosshair {
    pub size: u32,
    pub thickness: u32,
    pub gap: u32,
    pub color: String,          // Format hex: "#00FF00"
    pub alpha: f32,             // Transparence 0.0-1.0
    pub rotation: f32,          // Rotation en degrés
    pub center_dot: CenterDot,
    pub style: CrosshairStyle,
    pub outline: Outline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterDot {
    pub enabled: bool,
    pub size: u32,
    pub color: String,          // Format hex: "#FF0000"
    pub alpha: f32,             // Transparence 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub enabled: bool,
    pub thickness: u32,
    pub color: String,          // Format hex: "#000000"
    pub alpha: f32,             // Transparence 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CrosshairStyle {
    Classic,    // Lignes droites avec gap
    Dot,        // Juste un point central
    Cross,      // Croix pleine sans gap
    Circle,     // Cercle
    T,          // Forme T
    Plus,       // Signe plus épais
    X,          // Croix en X (diagonale)
}

impl Default for Crosshair {
    fn default() -> Self {
        Self {
            size: 25,
            thickness: 2,
            gap: 5,
            color: "#00FF00".to_string(),  // Vert
            alpha: 1.0,
            rotation: 0.0,
            center_dot: CenterDot::default(),
            style: CrosshairStyle::Classic,
            outline: Outline::default(),
        }
    }
}

impl Default for CenterDot {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 2,
            color: "#FF0000".to_string(),  // Rouge
            alpha: 1.0,
        }
    }
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            enabled: false,
            thickness: 1,
            color: "#000000".to_string(),  // Noir
            alpha: 0.8,
        }
    }
}

impl Crosshair {
    /// Convertit une couleur hex en format RGBA u32 avec alpha
    pub fn parse_color_with_alpha(hex: &str, alpha: f32) -> u32 {
        let hex = hex.trim_start_matches('#');
        let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u32;
        
        if hex.len() == 6 {
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                return (alpha_u8 << 24) | rgb;
            }
        }
        (alpha_u8 << 24) | 0xFFFFFF // Blanc par défaut avec alpha
    }

    /// Obtient la couleur du crosshair en format u32 avec alpha
    pub fn get_color(&self) -> u32 {
        Self::parse_color_with_alpha(&self.color, self.alpha)
    }

    /// Obtient la couleur du point central en format u32 avec alpha
    pub fn get_center_dot_color(&self) -> u32 {
        Self::parse_color_with_alpha(&self.center_dot.color, self.center_dot.alpha)
    }

    /// Obtient la couleur de l'outline en format u32 avec alpha
    pub fn get_outline_color(&self) -> u32 {
        Self::parse_color_with_alpha(&self.outline.color, self.outline.alpha)
    }

    /// Applique la rotation à un point
    fn rotate_point(&self, x: f32, y: f32, center_x: f32, center_y: f32) -> (f32, f32) {
        if self.rotation == 0.0 {
            return (x, y);
        }
        
        let angle = self.rotation.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        let dx = x - center_x;
        let dy = y - center_y;
        
        let rotated_x = dx * cos_a - dy * sin_a + center_x;
        let rotated_y = dx * sin_a + dy * cos_a + center_y;
        
        (rotated_x, rotated_y)
    }

    /// Vérifie si un point fait partie de la ligne du crosshair (avec rotation)
    fn is_on_crosshair_line(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        
        // Appliquer la rotation inverse pour tester dans l'espace non-rotaté
        let (rot_x, rot_y) = self.rotate_point(x as f32, y as f32, center_x, center_y);
        
        let dx = (rot_x - center_x).abs();
        let dy = (rot_y - center_y).abs();
        let size = self.size as f32;
        let thickness = self.thickness as f32 / 2.0;
        let gap = self.gap as f32;
        
        match self.style {
            CrosshairStyle::Classic => {
                // Ligne horizontale avec gap
                let on_horizontal = dy <= thickness && dx <= size && dx >= gap;
                // Ligne verticale avec gap
                let on_vertical = dx <= thickness && dy <= size && dy >= gap;
                on_horizontal || on_vertical
            },
            CrosshairStyle::Cross => {
                // Ligne horizontale complète
                let on_horizontal = dy <= thickness && dx <= size;
                // Ligne verticale complète
                let on_vertical = dx <= thickness && dy <= size;
                on_horizontal || on_vertical
            },
            CrosshairStyle::Plus => {
                // Plus épais
                let thick = self.thickness as f32;
                let on_horizontal = dy <= thick && dx <= size;
                let on_vertical = dx <= thick && dy <= size;
                on_horizontal || on_vertical
            },
            CrosshairStyle::T => {
                // Ligne horizontale
                let on_horizontal = dy <= thickness && dx <= size;
                // Ligne verticale seulement vers le bas
                let on_vertical = dx <= thickness && rot_y >= center_y && dy <= size;
                on_horizontal || on_vertical
            },
            CrosshairStyle::X => {
                // Lignes diagonales
                let diag1 = (dx - dy).abs() <= thickness && dx <= size && dy <= size;
                let diag2 = (dx + dy - 2.0 * center_y).abs() <= thickness && dx <= size && dy <= size;
                diag1 || diag2
            },
            _ => false, // Dot et Circle sont gérés séparément
        }
    }

    /// Dessine le crosshair sur le buffer selon sa configuration
    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32) {
        // Appliquer l'effet rainbow si activé
        let mut color = if effects.rainbow.enabled {
            effects.rainbow.get_color(time, self.alpha)
        } else {
            self.get_color()
        };

        // Appliquer l'effet pulse si activé
        if effects.pulse.enabled {
            color = effects.pulse.apply(color, time);
        }

        // Dessiner le crosshair selon son style
        match self.style {
            CrosshairStyle::Dot => {
                self.draw_dot_with_effects(buffer, width, height, effects, time, color);
            },
            CrosshairStyle::Circle => {
                self.draw_circle_with_effects(buffer, width, height, effects, time, color);
            },
            _ => {
                self.draw_lines_with_effects(buffer, width, height, effects, time, color);
            }
        }

        // Dessiner le point central si activé (par-dessus tout)
        if self.center_dot.enabled {
            self.draw_center_dot_with_effects(buffer, width, height, effects, time);
        }
    }

    fn draw_lines_with_effects(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32, base_color: u32) {
        let outline_color = self.get_outline_color();
        
        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);
        
        for y in 0..height {
            for x in 0..width {
                // Appliquer le shake en décalant les coordonnées
                let adjusted_x = (x as f32 - shake_x) as usize;
                let adjusted_y = (y as f32 - shake_y) as usize;
                
                if self.is_on_crosshair_line(adjusted_x, adjusted_y, width, height) {
                    let mut final_color = base_color;
                    
                    // Dessiner l'outline si activé
                    if self.outline.enabled {
                        // Vérifier si c'est un pixel de bordure
                        let is_edge = !self.is_on_crosshair_line(adjusted_x.saturating_sub(1), adjusted_y, width, height) ||
                                     !self.is_on_crosshair_line(adjusted_x + 1, adjusted_y, width, height) ||
                                     !self.is_on_crosshair_line(adjusted_x, adjusted_y.saturating_sub(1), width, height) ||
                                     !self.is_on_crosshair_line(adjusted_x, adjusted_y + 1, width, height);
                        
                        if is_edge {
                            final_color = outline_color;
                        }
                    }
                    
                    buffer[y * width + x] = final_color;
                }
            }
        }

        // Point central
        if self.center_dot.enabled {
            self.draw_center_dot_with_effects(buffer, width, height, effects, time);
        }
    }

    fn draw_dot_with_effects(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32, base_color: u32) {
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = self.size as f32;

        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);

        // Dessiner un cercle plein centré
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - (center_x as f32 + shake_x);
                let dy = y as f32 - (center_y as f32 + shake_y);
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    buffer[y * width + x] = base_color;
                }
            }
        }

        // Optionnellement ajouter le center_dot par-dessus si activé
        if self.center_dot.enabled {
            self.draw_center_dot_with_effects(buffer, width, height, effects, time);
        }
    }

    fn draw_circle_with_effects(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32, base_color: u32) {
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = self.size as i32;

        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - (center_x as i32 + shake_x as i32);
                let dy = y as i32 - (center_y as i32 + shake_y as i32);
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                
                if distance >= (radius - self.thickness as i32) as f32 && distance <= radius as f32 {
                    buffer[y * width + x] = base_color;
                }
            }
        }

        if self.center_dot.enabled {
            self.draw_center_dot_with_effects(buffer, width, height, effects, time);
        }
    }

    fn draw_center_dot_with_effects(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32) {
        let center_x = width / 2;
        let center_y = height / 2;
        
        // Couleur du point central avec effets possibles
        let mut dot_color = if effects.rainbow.enabled {
            // Version plus sombre du rainbow pour le centre
            let rainbow = effects.rainbow.get_color(time * 1.5, self.alpha);
            (rainbow & 0x00FFFFFF) | ((self.center_dot.alpha * 255.0) as u32) << 24
        } else {
            self.get_center_dot_color()
        };

        if effects.pulse.enabled {
            dot_color = effects.pulse.apply(dot_color, time * 1.2);
        }

        let radius = self.center_dot.size as f32;

        // Calculer l'offset de shake
        let (shake_x, shake_y) = effects.shake.get_offset(time);

        // Dessiner un petit cercle centré
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - (center_x as f32 + shake_x);
                let dy = y as f32 - (center_y as f32 + shake_y);
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    buffer[y * width + x] = dot_color;
                }
            }
        }
    }
} 