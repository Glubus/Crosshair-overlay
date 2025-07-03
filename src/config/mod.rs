use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod effects;
pub mod window;

use crate::crosshair::Crosshair;
pub use effects::Effects;
pub use window::WindowConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrosshairConfig {
    pub crosshair: Crosshair,
    pub effects: Effects,
    pub window: WindowConfig,
}

impl Default for CrosshairConfig {
    fn default() -> Self {
        Self {
            crosshair: Crosshair::default(),
            effects: Effects::default(),
            window: WindowConfig::default(),
        }
    }
}

impl CrosshairConfig {
    /// Charge la configuration depuis config.toml ou crée une configuration par défaut
    pub fn load_or_default() -> Self {
        Self::load_from_file("config.toml").unwrap_or_else(|e| {
            eprintln!("⚠️  Erreur lors du chargement de config.toml: {}", e);
            eprintln!("📝 Utilisation de la configuration par défaut");
            let default_config = Self::default();
            
            // Essayer de créer le fichier de config par défaut
            if let Err(save_error) = default_config.save_to_file("config.toml") {
                eprintln!("⚠️  Impossible de créer config.toml: {}", save_error);
            } else {
                println!("✅ Fichier config.toml créé avec la configuration par défaut");
            }
            
            default_config
        })
    }

    /// Charge la configuration depuis un fichier TOML
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Sauvegarde la configuration dans un fichier TOML
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    /// Valide la configuration et retourne les erreurs éventuelles
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // Validation du crosshair
        if self.crosshair.size == 0 {
            errors.push("La taille du crosshair ne peut pas être 0".to_string());
        }

        if self.crosshair.thickness == 0 {
            errors.push("L'épaisseur du crosshair ne peut pas être 0".to_string());
        }

        if !(0.0..=1.0).contains(&self.crosshair.alpha) {
            errors.push("L'alpha du crosshair doit être entre 0.0 et 1.0".to_string());
        }

        // Validation des effets
        if self.effects.pulse.enabled && !(0.0..=1.0).contains(&self.effects.pulse.min_alpha) {
            errors.push("L'alpha minimum du pulse doit être entre 0.0 et 1.0".to_string());
        }

        if self.effects.pulse.enabled && !(0.0..=1.0).contains(&self.effects.pulse.max_alpha) {
            errors.push("L'alpha maximum du pulse doit être entre 0.0 et 1.0".to_string());
        }

        // Validation de la fenêtre
        if self.window.size == 0 {
            errors.push("La taille de la fenêtre ne peut pas être 0".to_string());
        }

        errors
    }
} 