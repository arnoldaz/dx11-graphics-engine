
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui, ConfigFlags};
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11DeviceContext};
use crate::imgui_dx11_renderer::Renderer;
use crate::imgui_glfw_support::{HiDpiMode, GlfwPlatform};

pub struct WindowUi {
    pub imgui: Context,
    pub platform: GlfwPlatform,
    pub renderer: Renderer,
}

impl WindowUi {
    
    pub fn new(window: &crate::window::Window, device: &ID3D11Device, device_context: &ID3D11DeviceContext) -> Result<Self, Box<dyn std::error::Error>> {
        // return Ok(WindowUi {  });

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let io = imgui.io_mut();
        io.config_flags |= ConfigFlags::NAV_ENABLE_KEYBOARD;
        io.config_flags |= ConfigFlags::NAV_ENABLE_GAMEPAD;

        // imgui.set_clipboard_backend(backend);

        let mut platform = GlfwPlatform::init(&mut imgui);

        let dpi_mode = HiDpiMode::Default;

        platform.attach_window(imgui.io_mut(), &window.window, dpi_mode);

        let renderer = unsafe { Renderer::new(&mut imgui, device, device_context).expect("Failed to initialize renderer") };
        
        Ok(WindowUi { imgui, platform, renderer })
    }


}