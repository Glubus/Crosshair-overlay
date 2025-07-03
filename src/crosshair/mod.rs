use serde::{Deserialize, Serialize};
use crate::config::effects::Effects;
use crate::config::effects::mouse::VisibilityMask;

// Modules pour chaque style de crosshair
pub mod dot;
pub mod classic;
pub mod circle;
pub mod t_shape;
pub mod x_shape;
pub mod square;
pub mod diamond;

// Re-exports pour faciliter l'utilisation
pub use dot::DotCrosshair;
pub use classic::ClassicCrosshair;
pub use circle::CircleCrosshair;
pub use t_shape::TShapeCrosshair;
pub use x_shape::XShapeCrosshair;
pub use square::SquareCrosshair;
pub use diamond::DiamondCrosshair;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CrosshairStyle {
    Classic,    // Lignes droites avec gap
    Dot,        // Juste un point central
    Circle,     // Cercle
    T,          // Forme T avec gap
    X,          // Croix en X (diagonale)
    Square,     // Carré avec gap
    Diamond,    // Losange avec gap
}

impl Default for CrosshairStyle {
    fn default() -> Self {
        CrosshairStyle::Classic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterDot {
    pub enabled: bool,
    pub size: u32,
    pub color: String,          // Format hex: "#FF0000"
    pub alpha: f32,             // Transparence 0.0-1.0
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub enabled: bool,
    pub thickness: u32,
    pub color: String,          // Format hex: "#000000"
    pub alpha: f32,             // Transparence 0.0-1.0
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

/// Structure principale du crosshair avec toutes ses propriétés
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
    pub triangle_bars: bool,    // Pour style classic : triangles au lieu de rectangles
    pub filled_circle: bool,    // Pour style circle : plein ou juste contour
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
            triangle_bars: false,  // Rectangles par défaut
            filled_circle: false,  // Contour par défaut
        }
    }
}

/// Trait commun pour tous les styles de crosshair
pub trait CrosshairRenderer {
    fn draw(&self, crosshair: &Crosshair, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32);
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
    pub fn rotate_point(&self, x: f32, y: f32, center_x: f32, center_y: f32) -> (f32, f32) {
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

    /// Dessine le crosshair selon son style
    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32) {
        use crate::config::effects::mouse::{get_global_mouse_state, VisibilityMask};
        
        // Récupérer l'état de la souris
        let mouse_state = get_global_mouse_state();
        
        // Appliquer les effets de souris
        let (modified_gap, modified_size, modified_alpha, visibility_mask) = effects.mouse.apply_effects(
            self.gap,
            self.size,
            self.alpha,
            &mouse_state
        );
        
        // Créer une version modifiée du crosshair avec les effets appliqués
        let mut modified_crosshair = self.clone();
        modified_crosshair.gap = modified_gap;
        modified_crosshair.size = modified_size;
        modified_crosshair.alpha = modified_alpha;
        // Sélectionner le renderer selon le style avec masque de visibilité
        match self.style {
            CrosshairStyle::Classic => {
                let renderer = ClassicCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::Dot => {
                let renderer = DotCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::Circle => {
                let renderer = CircleCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::T => {
                let renderer = TShapeCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::X => {
                let renderer = XShapeCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::Square => {
                let renderer = SquareCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
            CrosshairStyle::Diamond => {
                let renderer = DiamondCrosshair;
                self.draw_with_visibility_mask(&renderer, &modified_crosshair, buffer, width, height, effects, time, &visibility_mask);
            },
        }

        // Dessiner le point central si activé (par-dessus tout)
        if self.center_dot.enabled && visibility_mask.show_center {
            self.draw_center_dot_with_effects(buffer, width, height, effects, time);
        }
    }

    /// Dessine avec le masque de visibilité appliqué
    fn draw_with_visibility_mask<T: CrosshairRenderer>(
        &self,
        renderer: &T,
        modified_crosshair: &Crosshair,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        effects: &Effects,
        time: f32,
        visibility_mask: &VisibilityMask
    ) {
        if !visibility_mask.show_full {
            return; // Ne rien dessiner si show_full est false
        }

        // Créer un buffer temporaire pour appliquer le masque de visibilité
        let mut temp_buffer = vec![0u32; buffer.len()];
        
        // Dessiner sur le buffer temporaire
        renderer.draw(modified_crosshair, &mut temp_buffer, width, height, effects, time);
        
        // Appliquer le masque de visibilité
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        
        for y in 0..height {
            for x in 0..width {
                let pixel = temp_buffer[y * width + x];
                if pixel != 0 { // Si le pixel n'est pas transparent
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    
                    if visibility_mask.should_show_point(dx, dy, center_x, center_y) {
                        // Appliquer le multiplicateur d'alpha
                        let alpha = ((pixel >> 24) & 0xFF) as f32 / 255.0;
                        let modified_alpha = (alpha * visibility_mask.alpha_multiplier * 255.0) as u32;
                        let modified_pixel = (modified_alpha << 24) | (pixel & 0x00FFFFFF);
                        buffer[y * width + x] = modified_pixel;
                    }
                }
            }
        }
    }

    /// Dessine le point central avec effets
    pub fn draw_center_dot_with_effects(&self, buffer: &mut [u32], width: usize, height: usize, effects: &Effects, time: f32) {
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