
use std::{os::windows::prelude::OsStrExt, ptr};

use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*,
    Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*,
    Win32::UI::WindowsAndMessaging::*,

    Win32::Graphics::Direct3D::Fxc::*,
    Win32::Graphics::Hlsl::*
};

use directx_math::*;

use std::mem::*;

extern crate glfw;


pub struct VertexPositionColor {
    position: XMFLOAT3,
    color: XMFLOAT3,
}


pub struct WindowApplication {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,

    window_width: u32,
    window_height: u32,

    dxgi_factory: IDXGIFactory2,
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    swap_chain: IDXGISwapChain1,
    render_target: ID3D11RenderTargetView,

    vertex_layout: ID3D11InputLayout,
    triangle_vertices: ID3D11Buffer,

    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
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

        let mut device_context: Option<ID3D11DeviceContext> = Default::default();

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
                Some(&mut device_context),
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

        let mut render_target: Option<ID3D11RenderTargetView> = Default::default();

        // windows::core::IntoParam<windows::Win32::Graphics::Direct3D11::ID3D11Resource

        // expected raw pointer `*mut std::option::Option<windows::Win32::Graphics::Direct3D11::ID3D11RenderTargetView>`
        // found reference `&windows::Win32::Graphics::Direct3D11::ID3D11RenderTargetView`

        unsafe { 
            device_unwrapped.CreateRenderTargetView(
                &back_buffer,
                None,
                Some(&mut render_target),
            ).unwrap();
        };


        // let vertex_shader = Self::create_vertex_shader(&device_unwrapped, "main.vs.hlsl");
        let vertex_shader_blob = Self::compile_shader("src/main.vs.hlsl", "Main", "vs_5_0");

        let data_slice_vertex: &[u8] = unsafe {
            std::slice::from_raw_parts(vertex_shader_blob.GetBufferPointer() as *const u8, vertex_shader_blob.GetBufferSize())
        };

        let vertex_shader: *mut Option<ID3D11VertexShader> = ptr::null_mut();

        unsafe {
            device_unwrapped.CreateVertexShader(
                data_slice_vertex,
                None,
                Some(vertex_shader),
            ).unwrap();
        }

        let vertex_shader = unsafe { vertex_shader.as_ref().unwrap().to_owned().unwrap() };



        // let pixel_shader = Self::create_pixel_shader(&device_unwrapped, "main.ps.hlsl");
        let pixel_shader_blob = Self::compile_shader("AAAAAAAAAsrc/main.ps.hlsl", "Main", "ps_5_0");

        let data_slice_pixel: &[u8] = unsafe {
            std::slice::from_raw_parts(pixel_shader_blob.GetBufferPointer() as *const u8, pixel_shader_blob.GetBufferSize())
        };

        let pixel_shader: *mut Option<ID3D11PixelShader> = ptr::null_mut();

        unsafe {
            device_unwrapped.CreatePixelShader(
                data_slice_pixel,
                None,
                Some(pixel_shader),
            ).unwrap();
        }

        let pixel_shader = unsafe { pixel_shader.as_ref().unwrap().to_owned().unwrap() };



        let vertex_input_layout_info: [D3D11_INPUT_ELEMENT_DESC; 2] = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR("POSITION".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0, // core::mem::offset_of!(VertexPositionColor, position) as u32,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,

            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR("COLOR".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12, // core::mem::offset_of!(VertexPositionColor, position) as u32,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            }
        ];


        let vertex_layout: *mut Option<ID3D11InputLayout> = ptr::null_mut();
        
        unsafe { 
            device_unwrapped.CreateInputLayout(
                &vertex_input_layout_info,
                data_slice_vertex,
                Some(vertex_layout),
            ).unwrap();
        };


        let vertices: [VertexPositionColor; 3] = [
            VertexPositionColor { position: XMFLOAT3 { x:  0.0, y:  0.5, z: 0.0 }, color: XMFLOAT3 { x: 0.25, y: 0.39, z: 0.19 } },
            VertexPositionColor { position: XMFLOAT3 { x:  0.5, y: -0.5, z: 0.0 }, color: XMFLOAT3 { x: 0.44, y: 0.75, z: 0.35 } },
            VertexPositionColor { position: XMFLOAT3 { x: -0.5, y: -0.5, z: 0.0 }, color: XMFLOAT3 { x: 0.38, y: 0.55, z: 0.20 } },
        ];

        let buffer_info = D3D11_BUFFER_DESC {
            ByteWidth: size_of::<[VertexPositionColor; 3]>() as u32,
            Usage: D3D11_USAGE_IMMUTABLE,
            BindFlags: 1, //D3D11_BIND_VERTEX_BUFFER,
            ..Default::default()
            // CPUAccessFlags: todo!(),
            // MiscFlags: todo!(),
            // StructureByteStride: todo!(),
        };

        let resource_data = D3D11_SUBRESOURCE_DATA {
            pSysMem: vertices.as_ptr() as *const ::core::ffi::c_void,
            ..Default::default()
        };

        let triangle_vertices: *mut Option<ID3D11Buffer> = ptr::null_mut();

        unsafe { 
            device_unwrapped.CreateBuffer(
                &buffer_info,
                Some(&resource_data),
                Some(triangle_vertices),
            ).unwrap();
        }


        let application = WindowApplication { 
            glfw,
            window,
            events,

            window_height,
            window_width,

            dxgi_factory,
            device: device_unwrapped,
            device_context: device_context.unwrap(),
            swap_chain,
            render_target: render_target.unwrap(),

            vertex_layout: unsafe { vertex_layout.as_ref().unwrap().to_owned().unwrap() },
            triangle_vertices: unsafe { triangle_vertices.as_ref().unwrap().to_owned().unwrap() },

            vertex_shader,
            pixel_shader,
        };

        Ok(application)
    }

    fn compile_shader(file_name: &str, entry_point: &str, profile: &str) -> ID3DBlob {
        let compiled_shader: *mut Option<ID3DBlob> = ptr::null_mut();
        let error_messages: *mut Option<ID3DBlob> = ptr::null_mut();

        // pub unsafe fn D3DCompileFromFile<P0, P1, P2, P3>(
        //     pfilename: P0, 
        //     pdefines: ::core::option::Option<*const super::D3D_SHADER_MACRO>,
        //      pinclude: P1,
        //      pentrypoint: P2,
        //      ptarget: P3,
        //      flags1: u32,
        //      flags2: u32,
        //      ppcode: *mut ::core::option::Option<super::ID3DBlob>,
        //      pperrormsgs: ::core::option::Option<*mut ::core::option::Option<super::ID3DBlob>>) -> ::windows_core::Result<()>
        // where
        //     P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        //     P1: ::windows_core::IntoParam<super::ID3DInclude>,
        //     P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
        //     P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
        // {

            

        // let wide_file_name: Vec<u16> = std::ffi::OsStr::new(&file_name)
        //     .encode_wide()
        //     .chain(Some(0).into_iter()) // Null-terminate the wide string
        //     .collect();

        let path = std::path::Path::new(file_name);
        let os_str = path.as_os_str();

        let h_string = HSTRING::from(os_str);


        unsafe { 
            D3DCompileFromFile(
                &h_string,
                None,
                None, // D3D_COMPILE_STANDARD_FILE_INCLUDE,
                PCSTR::from_raw(entry_point.as_ptr()),
                PCSTR::from_raw(profile.as_ptr()),
                D3DCOMPILE_ENABLE_STRICTNESS,
                0,
                compiled_shader,
                Some(error_messages),
            ).unwrap();
        }

        let shader = unsafe { std::ptr::read(compiled_shader) };

        shader.as_ref().unwrap().to_owned()
    }

    fn create_vertex_shader(device: &ID3D11Device, file_name: &'static str) -> ID3D11VertexShader {
        let vertex_shader_blob = Self::compile_shader(file_name, "Main", "vs_5_0");

        let data_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(vertex_shader_blob.GetBufferPointer() as *const u8, vertex_shader_blob.GetBufferSize())
        };

        let vertex_shader: *mut Option<ID3D11VertexShader> = ptr::null_mut();

        unsafe {
            device.CreateVertexShader(
                data_slice,
                None,
                Some(vertex_shader),
            ).unwrap();
        }

        unsafe { vertex_shader.as_ref().unwrap().to_owned().unwrap() }
    }

    fn create_pixel_shader(device: &ID3D11Device, file_name: &'static str) -> ID3D11PixelShader {
        let pixel_shader_blob = Self::compile_shader(file_name, "Main", "ps_5_0");

        let data_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(pixel_shader_blob.GetBufferPointer() as *const u8, pixel_shader_blob.GetBufferSize())
        };

        let pixel_shader: *mut Option<ID3D11PixelShader> = ptr::null_mut();

        unsafe {
            device.CreatePixelShader(
                data_slice,
                None,
                Some(pixel_shader),
            ).unwrap();
        }

        unsafe { pixel_shader.as_ref().unwrap().to_owned().unwrap() }
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

        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0f32,
            TopLeftY: 0f32,
            Width: self.window_width as f32,
            Height: self.window_height as f32,
            MinDepth: 0f32,
            MaxDepth: 1f32,
        };

        unsafe { 
            self.device_context.ClearRenderTargetView(&self.render_target, &[0.1f32, 0.1f32, 0.1f32, 0.1f32]);


            self.device_context.IASetInputLayout(&self.vertex_layout);

            let vertex_stride: u32 = size_of::<VertexPositionColor>() as u32;
            let vertex_offset: u32 = 0;

            self.device_context.IASetVertexBuffers(
                0,
                1,
                Some(&Some(self.triangle_vertices.clone())),
                Some(&vertex_stride),
                Some(&vertex_offset)
            );

            self.device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            self.device_context.RSSetViewports(Some(&[viewport]));

            self.device_context.VSSetShader(&self.vertex_shader, None);
            self.device_context.PSSetShader(&self.pixel_shader, None);

            self.device_context.OMSetRenderTargets(Some(&[Some(self.render_target.clone())]), None);

            self.device_context.Draw(3, 0);

            let _ = self.swap_chain.Present(1, 0); 
        };

    }

    fn _cleanup(&self) {

    }

}