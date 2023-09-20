
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use crate::imgui_dx11_renderer::Renderer;
use crate::imgui_glfw_support::{HiDpiMode, GlfwPlatform};

pub struct WindowUi {
    pub imgui: Context,
    pub platform: GlfwPlatform,
    pub renderer: Renderer,
}

impl WindowUi {
    
    pub fn new(window: &crate::window::Window, device: &ID3D11Device) -> Result<Self, Box<dyn std::error::Error>> {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // imgui.set_clipboard_backend(backend);

        let mut platform = GlfwPlatform::init(&mut imgui);

        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window.window, dpi_mode);

        let renderer = unsafe { Renderer::new(&mut imgui, device).expect("Failed to initialize renderer") };
        
        Ok(WindowUi { imgui, platform, renderer })
    }


}