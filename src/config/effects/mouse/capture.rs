use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx,
        HC_ACTION, HHOOK, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
        WM_RBUTTONDOWN, WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    },
};
use super::MouseState;

// Variables atomiques pour l'état des boutons
static LEFT_PRESSED: AtomicBool = AtomicBool::new(false);
static RIGHT_PRESSED: AtomicBool = AtomicBool::new(false);
static MIDDLE_PRESSED: AtomicBool = AtomicBool::new(false);

// Variable pour détecter les changements d'état (pour optimiser les redraws)
static STATE_CHANGED: AtomicBool = AtomicBool::new(false);

// Timing pour les effets
static PRESS_START_TIME: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();

pub struct MouseCapture {
    hook: Option<HHOOK>,
}

impl MouseCapture {
    pub fn new() -> Self {
        Self { hook: None }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.hook.is_some() {
            return Ok(()); // Déjà démarré
        }

        unsafe {
            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                HINSTANCE::default(),
                0,
            );

            if hook.is_err() {
                return Err("Échec de l'installation du hook de souris".to_string());
            }

            self.hook = Some(hook.unwrap());
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(hook) = self.hook.take() {
            unsafe {
                UnhookWindowsHookEx(hook);
            }
        }
        
        // Réinitialiser l'état des boutons
        LEFT_PRESSED.store(false, Ordering::Relaxed);
        RIGHT_PRESSED.store(false, Ordering::Relaxed);
        MIDDLE_PRESSED.store(false, Ordering::Relaxed);
        
        // Réinitialiser le temps de clic
        if let Some(time_mutex) = PRESS_START_TIME.get() {
            if let Ok(mut time) = time_mutex.lock() {
                *time = None;
            }
        }
    }

    /// Récupère l'état actuel de la souris pour les effets
    pub fn get_mouse_state(&self) -> MouseState {
        let left = LEFT_PRESSED.load(Ordering::Relaxed);
        let right = RIGHT_PRESSED.load(Ordering::Relaxed);
        let middle = MIDDLE_PRESSED.load(Ordering::Relaxed);
        
        // Récupérer le temps de début du clic
        let press_time = PRESS_START_TIME
            .get_or_init(|| Mutex::new(None))
            .lock()
            .ok()
            .and_then(|guard| *guard);

        MouseState {
            left_pressed: left,
            right_pressed: right,
            middle_pressed: middle,
            press_time,
        }
    }

    /// Vérifie si un bouton est actuellement pressé
    pub fn is_any_button_pressed(&self) -> bool {
        LEFT_PRESSED.load(Ordering::Relaxed) ||
        RIGHT_PRESSED.load(Ordering::Relaxed) ||
        MIDDLE_PRESSED.load(Ordering::Relaxed)
    }
}

impl Drop for MouseCapture {
    fn drop(&mut self) {
        self.stop();
    }
}

// Fonction de callback pour le hook de souris
unsafe extern "system" fn mouse_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code == HC_ACTION as i32 {
        match wparam.0 as u32 {
            // Bouton gauche
            WM_LBUTTONDOWN => {
                set_button_state(true, true, false, false);
            }
            WM_LBUTTONUP => {
                set_button_state(false, true, false, false);
            }
            
            // Bouton droit
            WM_RBUTTONDOWN => {
                set_button_state(true, false, true, false);
            }
            WM_RBUTTONUP => {
                set_button_state(false, false, true, false);
            }
            
            // Bouton molette
            WM_MBUTTONDOWN => {
                set_button_state(true, false, false, true);
            }
            WM_MBUTTONUP => {
                set_button_state(false, false, false, true);
            }
            
            _ => {}
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

/// Helper pour gérer l'état des boutons et le timing
fn set_button_state(pressed: bool, is_left: bool, is_right: bool, is_middle: bool) {
    // Vérifier l'état actuel
    let any_was_pressed = LEFT_PRESSED.load(Ordering::Relaxed) ||
                         RIGHT_PRESSED.load(Ordering::Relaxed) ||
                         MIDDLE_PRESSED.load(Ordering::Relaxed);
    
    // Mettre à jour l'état du bouton
    if is_left {
        LEFT_PRESSED.store(pressed, Ordering::Relaxed);
    } else if is_right {
        RIGHT_PRESSED.store(pressed, Ordering::Relaxed);
    } else if is_middle {
        MIDDLE_PRESSED.store(pressed, Ordering::Relaxed);
    }
    
    // Vérifier l'état après modification
    let any_is_pressed = LEFT_PRESSED.load(Ordering::Relaxed) ||
                        RIGHT_PRESSED.load(Ordering::Relaxed) ||
                        MIDDLE_PRESSED.load(Ordering::Relaxed);
    
    // Marquer qu'il y a eu un changement d'état
    STATE_CHANGED.store(true, Ordering::Relaxed);
    
    // Gérer le timing
    if let Some(time_mutex) = PRESS_START_TIME.get() {
        if let Ok(mut time_guard) = time_mutex.lock() {
            if !any_was_pressed && any_is_pressed {
                // Début d'un clic
                *time_guard = Some(Instant::now());
            } else if any_was_pressed && !any_is_pressed {
                // Fin de tous les clics
                *time_guard = None;
            }
        }
    }
}

// Instance globale de la capture de souris (pour éviter les problèmes de lifetime)
static mut GLOBAL_MOUSE_CAPTURE: Option<MouseCapture> = None;

/// Initialise la capture globale de souris
pub fn initialize_global_mouse_capture() -> Result<(), String> {
    unsafe {
        if GLOBAL_MOUSE_CAPTURE.is_none() {
            let mut capture = MouseCapture::new();
            capture.start()?;
            GLOBAL_MOUSE_CAPTURE = Some(capture);
        }
    }
    Ok(())
}

/// Arrête la capture globale de souris
pub fn shutdown_global_mouse_capture() {
    unsafe {
        if let Some(mut capture) = GLOBAL_MOUSE_CAPTURE.take() {
            capture.stop();
        }
    }
}

/// Récupère l'état de la souris depuis la capture globale
pub fn get_global_mouse_state() -> MouseState {
    unsafe {
        GLOBAL_MOUSE_CAPTURE
            .as_ref()
            .map(|capture| capture.get_mouse_state())
            .unwrap_or_default()
    }
}

/// Vérifie si un bouton est pressé depuis la capture globale
pub fn is_any_button_pressed() -> bool {
    unsafe {
        GLOBAL_MOUSE_CAPTURE
            .as_ref()
            .map(|capture| capture.is_any_button_pressed())
            .unwrap_or(false)
    }
}

/// Vérifie si l'état de la souris a changé (pour optimiser les redraws)
pub fn has_mouse_state_changed() -> bool {
    STATE_CHANGED.swap(false, Ordering::Relaxed)
} 