mod window_application;
mod window;
mod window_ui;

use window::Window;
use window_application::WindowApplication;
use window_ui::WindowUi;

mod imgui_glfw_support;
mod imgui_winit_support;
// mod imgui_glfw_rs;

mod imgui_dx11_renderer;

fn main() {

    let mut window = Window::new("Test application!!!").unwrap();
    let application = WindowApplication::new(&window).unwrap();

    let mut window_ui = WindowUi::new(&window, &application.device).expect("Window UI failed to init");

    window.run(
        &application,
        // Box::new(|viewport_size: (u32, u32)| { application.render(viewport_size); }),
        // Box::new(|viewport_size: (u32, u32)| { application.on_resize(viewport_size); }),
        &mut window_ui
    );


}
