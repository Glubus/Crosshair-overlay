use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub size: u32,
    pub position: Position,
    pub opacity: f32,           // Opacité globale de la fenêtre 0.0-1.0
    pub background: Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub center_screen: bool,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub follow_cursor: bool,    // Suivre le curseur de la souris
    pub offset_x: i32,          // Décalage par rapport au curseur
    pub offset_y: i32,          // Décalage par rapport au curseur
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub enabled: bool,
    pub color: String,          // Couleur de fond
    pub alpha: f32,             // Transparence du fond 0.0-1.0
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: 100,
            position: Position::default(),
            opacity: 1.0,
            background: Background::default(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            center_screen: true,
            x: None,
            y: None,
            follow_cursor: false,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self {
            enabled: false,
            color: "#000000".to_string(),  // Noir
            alpha: 0.1,
        }
    }
} 