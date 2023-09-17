mod window_application;
mod window;

use window::Window;
use window_application::WindowApplication;


fn main() {

    let mut window = Window::new("Test application!!!").unwrap();
    let application = WindowApplication::new(&window).unwrap();

    window.run(
        Box::new(|viewport_size: (u32, u32)| { application.render(viewport_size); }),
        Box::new(|viewport_size: (u32, u32)| { application.on_resize(viewport_size); }),
    );


}
