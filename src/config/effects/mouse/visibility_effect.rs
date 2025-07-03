use serde::{Deserialize, Serialize};
use super::{MouseState, VisibilityMask};

/// Effet qui contrôle la visibilité des parties du crosshair selon les clics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityEffect {
    pub enabled: bool,
    pub hide_mode: HideMode,
    pub fade_percentage: f32,      // Pourcentage de disparition (0.0-1.0)
    pub smooth_fade: bool,         // Transition progressive
    pub fade_speed: f32,           // Vitesse de disparition (0.1-10.0, plus élevé = plus rapide)
    pub button_binding: VisibilityButtonBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HideMode {
    Full,      // Cache tout le crosshair
    Left,      // Cache la partie gauche
    Right,     // Cache la partie droite
    Top,       // Cache la partie haute
    Bottom,    // Cache la partie basse
    Center,    // Cache le centre seulement
    Sides,     // Cache gauche + droite
    Vertical,  // Cache haut + bas
    Cross,     // Cache en forme de croix (garde les coins)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityButtonBinding {
    pub left_click: Option<HideMode>,   // Effet pour clic gauche
    pub right_click: Option<HideMode>,  // Effet pour clic droit
    pub middle_click: Option<HideMode>, // Effet pour clic molette
}

impl Default for VisibilityEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            hide_mode: HideMode::Center,
            fade_percentage: 0.8,
            smooth_fade: true,
            fade_speed: 3.0,
            button_binding: VisibilityButtonBinding::default(),
        }
    }
}

impl Default for VisibilityButtonBinding {
    fn default() -> Self {
        Self {
            left_click: Some(HideMode::Center),  // Clic gauche = cache le centre
            right_click: Some(HideMode::Sides),  // Clic droit = cache les côtés
            middle_click: None,                  // Molette = pas d'effet
        }
    }
}

impl VisibilityEffect {
    /// Applique l'effet de visibilité selon l'état de la souris
    pub fn apply_visibility(&self, original_alpha: f32, mouse_state: &MouseState) -> (f32, VisibilityMask) {
        if !self.enabled {
            return (original_alpha, VisibilityMask::default());
        }

        // Déterminer le mode d'effet selon les boutons pressés
        let active_mode = self.get_active_mode(mouse_state);
        
        if let Some(mode) = active_mode {
            let visibility_mask = self.create_visibility_mask(mode, mouse_state);
            let modified_alpha = self.calculate_alpha(original_alpha, mouse_state);
            (modified_alpha, visibility_mask)
        } else {
            (original_alpha, VisibilityMask::default())
        }
    }

    /// Détermine le mode d'effet actif selon les boutons pressés
    fn get_active_mode(&self, mouse_state: &MouseState) -> Option<HideMode> {
        // Priorité : gauche > droit > molette
        if mouse_state.left_pressed {
            return self.button_binding.left_click.clone();
        }
        
        if mouse_state.right_pressed {
            return self.button_binding.right_click.clone();
        }
        
        if mouse_state.middle_pressed {
            return self.button_binding.middle_click.clone();
        }
        
        None
    }

    /// Crée le masque de visibilité selon le mode
    fn create_visibility_mask(&self, mode: HideMode, mouse_state: &MouseState) -> VisibilityMask {
        let intensity_factor = if self.smooth_fade {
            // Transition progressive basée sur la durée du clic et la vitesse configurée
            let duration = mouse_state.press_duration();
            (duration * self.fade_speed).min(1.0) // Vitesse configurable
        } else {
            1.0 // Effet instantané
        };

        let fade_alpha = 1.0 - (self.fade_percentage * intensity_factor);

        match mode {
            HideMode::Full => VisibilityMask {
                show_full: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Left => VisibilityMask {
                show_left: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Right => VisibilityMask {
                show_right: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Top => VisibilityMask {
                show_top: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Bottom => VisibilityMask {
                show_bottom: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Center => VisibilityMask {
                show_center: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Sides => VisibilityMask {
                show_left: false,
                show_right: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Vertical => VisibilityMask {
                show_top: false,
                show_bottom: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
            HideMode::Cross => VisibilityMask {
                show_left: false,
                show_right: false,
                show_top: false,
                show_bottom: false,
                alpha_multiplier: fade_alpha,
                ..Default::default()
            },
        }
    }

    /// Calcule l'alpha modifié
    fn calculate_alpha(&self, original_alpha: f32, mouse_state: &MouseState) -> f32 {
        let intensity_factor = if self.smooth_fade {
            let duration = mouse_state.press_duration();
            (duration * self.fade_speed).min(1.0) // Utiliser la même vitesse que pour le masque
        } else {
            1.0
        };

        let alpha_reduction = self.fade_percentage * intensity_factor;
        (original_alpha * (1.0 - alpha_reduction)).max(0.0)
    }

    /// Presets pour différents styles d'usage
    pub fn preset_sniper_clarity() -> Self {
        Self {
            enabled: true,
            hide_mode: HideMode::Center,
            fade_percentage: 1.0, // Cache complètement
            smooth_fade: false,
            fade_speed: 10.0, // Instantané
            button_binding: VisibilityButtonBinding {
                left_click: Some(HideMode::Center),
                right_click: Some(HideMode::Center),
                middle_click: None,
            },
        }
    }

    pub fn preset_peripheral_vision() -> Self {
        Self {
            enabled: true,
            hide_mode: HideMode::Sides,
            fade_percentage: 0.7,
            smooth_fade: true,
            fade_speed: 2.0, // Vitesse modérée
            button_binding: VisibilityButtonBinding {
                left_click: Some(HideMode::Sides),
                right_click: Some(HideMode::Vertical),
                middle_click: Some(HideMode::Full),
            },
        }
    }

    pub fn preset_minimal() -> Self {
        Self {
            enabled: true,
            hide_mode: HideMode::Cross,
            fade_percentage: 0.9,
            smooth_fade: true,
            fade_speed: 4.0, // Vitesse rapide
            button_binding: VisibilityButtonBinding {
                left_click: Some(HideMode::Cross),
                right_click: Some(HideMode::Full),
                middle_click: None,
            },
        }
    }
} 