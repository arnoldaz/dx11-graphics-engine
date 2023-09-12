
use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*,
    Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*,
    Win32::UI::WindowsAndMessaging::*,
};

extern crate glfw;

pub struct WindowApplication {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,

    dxgi_factory: IDXGIFactory2,
}

impl WindowApplication {
    pub fn new(title: &'static str) -> Result<Self> {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        let primary_monitor = unsafe { glfw::ffi::glfwGetPrimaryMonitor() };
        let video_mode = unsafe { glfw::ffi::glfwGetVideoMode(primary_monitor) };
        let screen_width = unsafe { u32::try_from((*video_mode).width).unwrap() };
        let screen_height = unsafe { u32::try_from((*video_mode).height).unwrap() };

        let window_width = (screen_width as f32 * 0.8).round() as u32;
        let window_height = (screen_height as f32 * 0.8).round() as u32;

        glfw.window_hint(glfw::WindowHint::ScaleToMonitor(false));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = glfw
            .create_window(window_width, window_height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        let window_left = screen_width / 2 - window_width / 2;
        let window_top = screen_height / 2 - window_height / 2;
        
        window.set_pos(window_left as i32, window_top as i32);
        
        // window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        let dxgi_factory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
    
        let mut device: Option<ID3D11Device> = None;

        // pub unsafe fn D3D11CreateDevice<P0, P1>(
        //     padapter: P0,
        //     drivertype: super::Direct3D::D3D_DRIVER_TYPE,
        //     software: P1,
        //     flags: D3D11_CREATE_DEVICE_FLAG,
        //     pfeaturelevels: ::core::option::Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>,
        //     sdkversion: u32,
        //     ppdevice: ::core::option::Option<*mut ::core::option::Option<ID3D11Device>>,
        //     pfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>,
        //     ppimmediatecontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>
        // ) -> ::windows_core::Result<()>

        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HINSTANCE::default(),
                D3D11_CREATE_DEVICE_FLAG(0),
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                None,
            ).unwrap();
        };

        let device_unwrapped = device.unwrap();

        let swap_chain_descriptor = DXGI_SWAP_CHAIN_DESC1 {
            Width: window_width,
            Height: window_height,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            ..Default::default()
        };

        let swap_chain_fullscreen_descriptor = DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            Windowed: BOOL(1),
            ..Default::default()
        };


        let window_handle = window.get_win32_window();
        let hwnd: HWND = unsafe { std::mem::transmute(window_handle) };

        let swap_chain: IDXGISwapChain1;

        unsafe { 
            swap_chain = dxgi_factory.CreateSwapChainForHwnd(
                &device_unwrapped,
                hwnd,
                &swap_chain_descriptor,
                Some(&swap_chain_fullscreen_descriptor),
                None
            ).unwrap();
        };

        // CreateSwapchainResources

        let back_buffer: ID3D11Resource = unsafe { swap_chain.GetBuffer(0).unwrap() };

        let render_target: ID3D11RenderTargetView;

        // windows::core::IntoParam<windows::Win32::Graphics::Direct3D11::ID3D11Resource

        // expected raw pointer `*mut std::option::Option<windows::Win32::Graphics::Direct3D11::ID3D11RenderTargetView>`
        // found reference `&windows::Win32::Graphics::Direct3D11::ID3D11RenderTargetView`

        unsafe { 
            device_unwrapped.CreateRenderTargetView(
                &back_buffer,
                None,
                None, // Some(&render_target),
            ).unwrap();
        };
        
        // let device_context: *mut ID3D11DeviceContext;
    


        Ok(WindowApplication { glfw, window, events, dxgi_factory })
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("Got event: {:?}", event);
    
                match event {
                    glfw::WindowEvent::FramebufferSize(_width, _height) => {}
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => {}
                };
            }
        
            self.render();
        
            // window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn render(&self) {

    }

    fn _cleanup(&self) {

    }

}