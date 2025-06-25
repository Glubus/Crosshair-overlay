use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct MouseCapture {
    #[cfg(windows)]
    hook: Option<windows::Win32::UI::WindowsAndMessaging::HHOOK>,
    active: bool,
}

impl MouseCapture {
    pub fn new() -> Self {
        Self {
            #[cfg(windows)]
            hook: None,
            active: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(windows)]
        {
            self.start_windows_hook()
        }
        
        #[cfg(not(windows))]
        {
            println!("⚠️  Capture de souris non supportée sur cette plateforme");
            Ok(())
        }
    }
    
    #[cfg(windows)]
    fn start_windows_hook(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use windows::{
            core::*,
            Win32::{
                Foundation::*,
                System::LibraryLoader::*,
                UI::WindowsAndMessaging::*,
            },
        };
        
        unsafe {
            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                GetModuleHandleW(PCWSTR::null())?,
                0,
            )?;
            
            self.hook = Some(hook);
            self.active = true;
            
            println!("🖱️  Capture de souris activée (optimisée)");
            Ok(())
        }
    }
}

impl Drop for MouseCapture {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            if let Some(_hook) = self.hook.take() {
                unsafe {
                    self.active = false;
                    println!("🖱️  Capture de souris désactivée");
                }
            }
        }
    }
}

#[cfg(windows)]
unsafe extern "system" fn mouse_hook_proc(
    n_code: i32,
    w_param: windows::Win32::Foundation::WPARAM,
    l_param: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    use windows::Win32::UI::WindowsAndMessaging::*;
    
    if n_code >= 0 {
        match w_param.0 as u32 {
            WM_LBUTTONDOWN => {
                println!("🖱️  Clic gauche détecté");
            },
            WM_RBUTTONDOWN => {
                println!("🖱️  Clic droit détecté");
            },
            WM_MBUTTONDOWN => {
                println!("🖱️  Clic molette détecté");
            },
            WM_LBUTTONUP => {
                println!("🖱️  Relâchement clic gauche");
            },
            WM_RBUTTONUP => {
                println!("🖱️  Relâchement clic droit");
            },
            WM_MBUTTONUP => {
                println!("🖱️  Relâchement clic molette");
            },
            _ => {}
        }
    }

    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
} 