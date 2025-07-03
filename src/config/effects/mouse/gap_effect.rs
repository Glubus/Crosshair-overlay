use serde::{Deserialize, Serialize};
use super::MouseState;

/// Effet qui modifie le gap du crosshair selon les clics de souris
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapEffect {
    pub enabled: bool,
    pub mode: GapMode,
    pub intensity: f32,        // Multiplicateur de l'effet (0.0-5.0)
    pub smooth_transition: bool, // Transition progressive ou instantanée
    pub button_binding: GapButtonBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GapMode {
    Expand,    // Écarte le crosshair (augmente le gap)
    Contract,  // Rapproche le crosshair (diminue le gap)
    Toggle,    // Alterne entre écarter et rapprocher
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapButtonBinding {
    pub left_click: Option<GapMode>,   // Effet pour clic gauche
    pub right_click: Option<GapMode>,  // Effet pour clic droit  
    pub middle_click: Option<GapMode>, // Effet pour clic molette
}

impl Default for GapEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: GapMode::Expand,
            intensity: 2.0,
            smooth_transition: true,
            button_binding: GapButtonBinding::default(),
        }
    }
}

impl Default for GapButtonBinding {
    fn default() -> Self {
        Self {
            left_click: Some(GapMode::Expand),   // Clic gauche = écarter
            right_click: Some(GapMode::Contract), // Clic droit = rapprocher
            middle_click: None,                   // Molette = pas d'effet
        }
    }
}

impl GapEffect {
    /// Applique l'effet de gap selon l'état de la souris
    pub fn apply_gap(&self, original_gap: u32, original_size: u32, mouse_state: &MouseState) -> (u32, u32) {
        if !self.enabled {
            return (original_gap, original_size);
        }

        // Déterminer le mode d'effet selon les boutons pressés
        let active_mode = self.get_active_mode(mouse_state);
        
        if let Some(mode) = active_mode {
            self.calculate_modified_gap_and_size(original_gap, original_size, mode, mouse_state)
        } else {
            (original_gap, original_size)
        }
    }

    /// Détermine le mode d'effet actif selon les boutons pressés
    fn get_active_mode(&self, mouse_state: &MouseState) -> Option<GapMode> {
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

    /// Calcule le gap et la taille modifiés selon le mode et l'intensité
    fn calculate_modified_gap_and_size(&self, original_gap: u32, original_size: u32, mode: GapMode, mouse_state: &MouseState) -> (u32, u32) {
        let base_gap = original_gap as f32;
        let base_size = original_size as f32;
        let intensity_factor = if self.smooth_transition {
            // Transition progressive basée sur la durée du clic
            let duration = mouse_state.press_duration();
            (duration * 2.0).min(1.0) // Max 1.0 après 0.5 seconde
        } else {
            1.0 // Effet instantané
        };

        let (modified_gap, modified_size) = match mode {
            GapMode::Expand => {
                // Augmenter le gap et la taille proportionnellement pour maintenir les lignes visibles
                let gap_increase = base_gap * self.intensity * intensity_factor * 0.5;
                let size_increase = gap_increase; // Augmenter la taille d'autant que le gap
                (
                    base_gap + gap_increase,
                    base_size + size_increase
                )
            },
            GapMode::Contract => {
                // Diminuer le gap et réduire légèrement la taille
                let gap_reduction = base_gap * self.intensity * intensity_factor * 0.3;
                let size_reduction = gap_reduction * 0.5; // Réduction plus faible de la taille
                (
                    (base_gap - gap_reduction).max(0.0),
                    (base_size - size_reduction).max(base_size * 0.7) // Minimum 70% de la taille originale
                )
            },
            GapMode::Toggle => {
                // Alterner selon le temps
                let cycle_time = mouse_state.press_duration() % 1.0; // Cycle de 1 seconde
                if cycle_time < 0.5 {
                    let gap_increase = base_gap * self.intensity * intensity_factor * 0.5;
                    let size_increase = gap_increase;
                    (
                        base_gap + gap_increase,
                        base_size + size_increase
                    )
                } else {
                    let gap_reduction = base_gap * self.intensity * intensity_factor * 0.3;
                    let size_reduction = gap_reduction * 0.5;
                    (
                        (base_gap - gap_reduction).max(0.0),
                        (base_size - size_reduction).max(base_size * 0.7)
                    )
                }
            }
        };

        (modified_gap.round() as u32, modified_size.round() as u32)
    }

    /// Configuration rapide pour différents styles d'usage
    pub fn preset_sniper() -> Self {
        Self {
            enabled: true,
            mode: GapMode::Expand,
            intensity: 3.0,
            smooth_transition: false,
            button_binding: GapButtonBinding {
                left_click: Some(GapMode::Expand),
                right_click: Some(GapMode::Expand),
                middle_click: None,
            },
        }
    }

    pub fn preset_precision() -> Self {
        Self {
            enabled: true,
            mode: GapMode::Contract,
            intensity: 1.5,
            smooth_transition: true,
            button_binding: GapButtonBinding {
                left_click: Some(GapMode::Contract),
                right_click: Some(GapMode::Contract),
                middle_click: None,
            },
        }
    }
} 