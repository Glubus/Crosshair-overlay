use std::num::NonZeroU32;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId, WindowAttributes},
    dpi::{LogicalPosition, LogicalSize},
};
use softbuffer::{Context, Surface};

mod config;
mod crosshair;

use config::CrosshairConfig;
use config::effects::mouse::{get_global_mouse_state, initialize_global_mouse_capture, shutdown_global_mouse_capture, has_mouse_state_changed};

struct App {
    window: Option<std::sync::Arc<Window>>,
    surface: Option<Surface<std::sync::Arc<Window>, std::sync::Arc<Window>>>,
    context: Option<Context<std::sync::Arc<Window>>>,
    config: CrosshairConfig,
    start_time: Instant,
    last_frame_time: Instant,
    frame_rate_limit: std::time::Duration,
    needs_redraw: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Obtenir la taille de l'écran pour centrer la fenêtre
        let screen_size = event_loop.primary_monitor()
            .unwrap()
            .size();
        
        let window_size = LogicalSize::new(self.config.window.size, self.config.window.size);
        
        let window_pos = if self.config.window.position.center_screen {
            LogicalPosition::new(
                (screen_size.width as f64 - window_size.width as f64) / 2.0,
                (screen_size.height as f64 - window_size.height as f64) / 2.0,
            )
        } else {
            LogicalPosition::new(
                self.config.window.position.x.unwrap_or(0) as f64,
                self.config.window.position.y.unwrap_or(0) as f64,
            )
        };

        // Créer les attributs de la fenêtre
        let window_attributes = WindowAttributes::default()
            .with_title("Crosshair Overlay Pro")
            .with_inner_size(window_size)
            .with_position(window_pos)
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
            .with_resizable(false);

        let window = std::sync::Arc::new(
            event_loop.create_window(window_attributes).unwrap()
        );

        // Activer le click-through - les clics passent à travers la fenêtre !
        if let Err(e) = window.set_cursor_hittest(false) {
            eprintln!("Attention: Click-through non supporté sur cette plateforme: {:?}", e);
        }

        // Initialiser softbuffer pour le rendu
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);

        // Démarrer la capture de souris
        if let Err(e) = initialize_global_mouse_capture() {
            eprintln!("❌ Erreur lors du démarrage de la capture de souris: {}", e);
        }

        // Premier rendu
        self.needs_redraw = true;
        self.redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key: Key::Named(NamedKey::Escape),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key: Key::Named(NamedKey::F5),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                // Recharger la configuration avec F5
                println!("🔄 Rechargement de la configuration...");
                self.config = CrosshairConfig::load_or_default();
                
                self.needs_redraw = true;
                self.redraw();
            },
            WindowEvent::RedrawRequested => {
                self.redraw();
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let has_animations = self.has_animated_effects();
        
        if has_animations {
            // Effets animés continus (pulse, shake, rainbow) - 30 FPS
            let next_frame = self.last_frame_time + self.frame_rate_limit;
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_frame));
            
            let now = Instant::now();
            if now >= next_frame {
                self.needs_redraw = true;
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
                self.last_frame_time = now;
            }
        } else {
            // Pas d'animation continue - attendre indéfiniment jusqu'au prochain événement
            // Les effets de souris seront gérés par des redraws déclenchés lors des clics
            event_loop.set_control_flow(ControlFlow::Wait);
            
            // Vérifier s'il y a eu un changement de souris et redessiner si nécessaire
            if self.config.effects.has_mouse_effects() && has_mouse_state_changed() {
                self.needs_redraw = true;
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
        }
    }
}

impl App {
    fn new() -> Self {
        let config = CrosshairConfig::load_or_default();
        
        // Limiter à 30 FPS pour les animations (au lieu de redessiner en continu)
        let frame_rate_limit = std::time::Duration::from_millis(33); // ~30 FPS
        
        Self {
            window: None,
            surface: None,
            context: None,
            config,
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            frame_rate_limit,
            needs_redraw: true, // Initialiser à true pour le premier dessin
        }
    }

    fn redraw(&mut self) {
        // Ne redessiner que si nécessaire
        if !self.needs_redraw {
            return;
        }

        // Calculer le temps écoulé pour les animations
        let elapsed = self.start_time.elapsed().as_secs_f32();
        
        // Créer des copies pour éviter les problèmes d'emprunt
        let crosshair = self.config.crosshair.clone();
        let effects = self.config.effects.clone();
        let background_enabled = self.config.window.background.enabled;
        let background_color = self.config.window.background.color.clone();
        let background_alpha = self.config.window.background.alpha;
        
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            let size = window.inner_size();
            if size.width > 0 && size.height > 0 {
                surface.resize(
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                ).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                
                // Remplir avec du noir transparent ou la couleur de fond
                if background_enabled {
                    let bg_color = Self::parse_color_with_alpha(&background_color, background_alpha);
                    buffer.fill(bg_color);
                } else {
                    buffer.fill(0x00000000);
                }

                // Dessiner le crosshair selon la configuration avec effets
                crosshair.draw(
                    &mut buffer, 
                    size.width as usize, 
                    size.height as usize, 
                    &effects,
                    elapsed
                );

                buffer.present().unwrap();
            }
        }

        // Ne redessiner que si des effets animés sont activés
        self.needs_redraw = self.has_animated_effects();
    }

    fn parse_color_with_alpha(hex: &str, alpha: f32) -> u32 {
        let hex = hex.trim_start_matches('#');
        let alpha_u8 = (alpha.clamp(0.0, 1.0) * 255.0) as u32;
        
        if hex.len() == 6 {
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                return (alpha_u8 << 24) | rgb;
            }
        }
        alpha_u8 << 24 // Noir transparent par défaut
    }

    fn has_animated_effects(&self) -> bool {
        self.config.effects.has_animated_effects()
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    // Utiliser Wait pour économiser le CPU - ne se réveille que sur événements
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    
    // Arrêter la capture de souris quand l'application se termine
    std::panic::set_hook(Box::new(|_| {
        shutdown_global_mouse_capture();
    }));
    
    println!("🎯 Crosshair Overlay Pro - Version Performance Maximale !");
    println!("📋 Fonctionnalités :");
    println!("   ✅ Configuration modulaire via config.toml");
    println!("   ✅ Fenêtre transparente configurable");
    println!("   ✅ Crosshair personnalisable (style: {:?})", app.config.crosshair.style);
    println!("   ✅ Effets visuels (pulse, shake, rainbow)");
    println!("   ✅ Rotation et alpha configurables");
    println!("   ✅ Outline et centre dot avancés");
    println!("   ✅ Click-through activé");
    println!("   🖱️  Capture de souris (clic gauche, droit, molette)");
    println!("   ⚡ Performance maximale - usage CPU ultra minimal");
    println!();
    println!("🎨 Styles disponibles : classic, dot, cross, circle, t, plus, x");
    println!("✨ Effets disponibles :");
    println!("   • Pulse: {:?} (alpha {:.1}-{:.1})", 
        app.config.effects.pulse.enabled, 
        app.config.effects.pulse.min_alpha, 
        app.config.effects.pulse.max_alpha);
    println!("   • Shake: {:?} (intensité: {:.1}, vitesse: {:.1})", 
        app.config.effects.shake.enabled,
        app.config.effects.shake.intensity,
        app.config.effects.shake.speed);
    println!("   • Rainbow: {:?} (saturation: {:.1}, luminosité: {:.1})", 
        app.config.effects.rainbow.enabled,
        app.config.effects.rainbow.saturation,
        app.config.effects.rainbow.brightness);
    println!("   • Effets Souris: {:?}", app.config.effects.mouse.enabled);
    println!();
    println!("⌨️  Contrôles :");
    println!("   • Échap : Quitter");
    println!("   • F5 : Recharger la configuration");
    println!();
    println!("💡 Modifiez config.toml pour explorer toutes les options !");
    println!("💡 Pour activer shake: [effects.shake] enabled = true");
    
    event_loop.run_app(&mut app).unwrap();
}

