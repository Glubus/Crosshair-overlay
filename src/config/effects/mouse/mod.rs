use serde::{Deserialize, Serialize};

// Modules pour chaque type d'effet de souris
pub mod gap_effect;
pub mod visibility_effect;
pub mod capture;

pub use gap_effect::GapEffect;
pub use visibility_effect::VisibilityEffect;
pub use capture::{initialize_global_mouse_capture, shutdown_global_mouse_capture, get_global_mouse_state, has_mouse_state_changed};

/// Configuration principale des effets de souris
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEffect {
    pub enabled: bool,
    pub gap_effect: GapEffect,
    pub visibility_effect: VisibilityEffect,
}

impl Default for MouseEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            gap_effect: GapEffect::default(),
            visibility_effect: VisibilityEffect::default(),
        }
    }
}

/// État global des boutons de souris pour les effets
#[derive(Debug, Clone, Default)]
pub struct MouseState {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub middle_pressed: bool,
    pub press_time: Option<std::time::Instant>,
}

impl MouseState {
    pub fn any_pressed(&self) -> bool {
        self.left_pressed || self.right_pressed || self.middle_pressed
    }
    
    pub fn press_duration(&self) -> f32 {
        self.press_time
            .map(|time| time.elapsed().as_secs_f32())
            .unwrap_or(0.0)
    }
}

impl MouseEffect {
    /// Applique les effets de souris aux propriétés du crosshair
    pub fn apply_effects(&self, 
        original_gap: u32,
        original_size: u32,
        original_alpha: f32,
        mouse_state: &MouseState
    ) -> (u32, u32, f32, VisibilityMask) {
        if !self.enabled || !mouse_state.any_pressed() {
            return (original_gap, original_size, original_alpha, VisibilityMask::default());
        }

        // Appliquer l'effet de gap (qui peut aussi modifier la taille)
        let (modified_gap, modified_size) = self.gap_effect.apply_gap(original_gap, original_size, mouse_state);
        
        // Appliquer l'effet de visibilité
        let (modified_alpha, visibility_mask) = self.visibility_effect.apply_visibility(
            original_alpha, 
            mouse_state
        );

        (modified_gap, modified_size, modified_alpha, visibility_mask)
    }
}

/// Masque de visibilité pour contrôler quelles parties du crosshair sont visibles
#[derive(Debug, Clone)]
pub struct VisibilityMask {
    pub show_full: bool,
    pub show_left: bool,
    pub show_right: bool,
    pub show_top: bool,
    pub show_bottom: bool,
    pub show_center: bool,
    pub alpha_multiplier: f32,
}

impl Default for VisibilityMask {
    fn default() -> Self {
        Self {
            show_full: true,
            show_left: true,
            show_right: true,
            show_top: true,
            show_bottom: true,
            show_center: true,
            alpha_multiplier: 1.0,
        }
    }
}

impl VisibilityMask {
    /// Vérifie si un point doit être visible selon sa position relative
    pub fn should_show_point(&self, dx: f32, dy: f32, center_x: f32, center_y: f32) -> bool {
        if !self.show_full {
            return false;
        }

        let x_pos = dx + center_x;
        let y_pos = dy + center_y;

        // Centre (proche de 0,0)
        if dx.abs() <= 3.0 && dy.abs() <= 3.0 {
            return self.show_center;
        }

        // Gauche
        if x_pos < center_x && !self.show_left {
            return false;
        }

        // Droite  
        if x_pos > center_x && !self.show_right {
            return false;
        }

        // Haut
        if y_pos < center_y && !self.show_top {
            return false;
        }

        // Bas
        if y_pos > center_y && !self.show_bottom {
            return false;
        }

        true
    }
} 