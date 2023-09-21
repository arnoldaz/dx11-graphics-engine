use crate::{window_ui::WindowUi, window_application::WindowApplication};


pub struct Window {
    glfw: glfw::Glfw,
    pub window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,

    pub window_width: u32,
    pub window_height: u32,
    pub is_fullscreen: bool,

    last_window_position: (i32, i32),
    last_window_size: (u32, u32),
}

impl Window {

    pub fn new(title: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut glfw = glfw::init_no_callbacks()?;

        let primary_monitor = glfw::Monitor::from_primary();
        let video_mode = primary_monitor.get_video_mode()
            .expect("Failed to find primary monitor video mode");

        let window_width = (video_mode.width as f32 * 0.8).round() as u32;
        let window_height = (video_mode.height as f32 * 0.8).round() as u32;

        glfw.window_hint(glfw::WindowHint::ScaleToMonitor(false));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = glfw
            .create_window(window_width, window_height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        let window_left = video_mode.width / 2 - window_width / 2;
        let window_top = video_mode.height / 2 - window_height / 2;
        
        window.set_pos(window_left as i32, window_top as i32);
        
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        Ok(Window { glfw, window, events, window_width, window_height, is_fullscreen: false, 
            last_window_position: (window_left as i32, window_top as i32),
            last_window_size: (window_width, window_height)
        })
    }

    pub fn get_win32(&self) -> windows::Win32::Foundation::HWND {
        let window_handle = self.window.get_win32_window();
        unsafe { std::mem::transmute(window_handle) }
    }
    
    // render_callback: Box<dyn Fn((u32, u32)) + 'a>, resize_callback: Box<dyn Fn((u32, u32)) + 'a>
    pub fn run<'a>(&mut self, window_application: &WindowApplication, window_ui: &mut WindowUi) {
        let mut last_frame = std::time::Instant::now();

        while !self.window.should_close() {
            let now = std::time::Instant::now();
            window_ui.imgui
                .io_mut()
                .update_delta_time(now.duration_since(last_frame));
            last_frame = now;

            for (_, event) in glfw::flush_messages(&self.events) {
                println!("Got window event: {:?}", event);
    
                window_ui.platform.handle_event(window_ui.imgui.io_mut(), &self.window, &event);

                match event {
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        self.window_width = width as u32;
                        self.window_height = height as u32;

                        window_application.on_resize((self.window_width, self.window_height));
                    }
                    glfw::WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, _) => {
                        if self.is_fullscreen {
                            self.window.set_monitor(glfw::WindowMode::Windowed,
                                self.last_window_position.0, self.last_window_position.1,
                                self.last_window_size.0, self.last_window_size.1,
                                None,
                            );
                        } else {
                            let window_position = self.window.get_pos();
                            let window_size = self.window.get_size();

                            self.last_window_position = window_position;
                            self.last_window_size = (window_size.0 as u32, window_size.1 as u32);

                            self.glfw.with_connected_monitors(|_, monitors| {
                                let mut highest_overlap = 0;
                                let mut best_monitor: Option<&glfw::Monitor> = None;
                    
                                for monitor in monitors.iter() {
                                    let video_mode = monitor.get_video_mode().unwrap();
                                    let monitor_position = monitor.get_pos();

                                    let overlap_area = Self::intersecting_area(window_position, window_size, monitor_position, (video_mode.width as i32, video_mode.height as i32));
                                    println!("Intersecting area for monitor {:?}: {:?}", monitor.get_name(), overlap_area);

                                    if overlap_area > highest_overlap {
                                        highest_overlap = overlap_area;
                                        best_monitor = Some(monitor);
                                    }
                                }
    
                                let monitor = best_monitor.unwrap();
                                let video_mode = monitor.get_video_mode().unwrap();

                                self.window.set_monitor(glfw::WindowMode::FullScreen(monitor),
                                    0, 0,
                                    video_mode.width, video_mode.height,
                                    Some(video_mode.refresh_rate)
                                );
                            });
                        }

                        self.is_fullscreen = !self.is_fullscreen;
                    }
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => {}
                };
            }

            



            window_ui.platform.prepare_frame(window_ui.imgui.io_mut(), &mut self.window).unwrap();
            let ui = window_ui.imgui.frame();

            // ui.window("Hello world")
            //     .size([1000.0, 1000.0], imgui::Condition::FirstUseEver)
            //     .build(|| {
            //         ui.text("Hello world!");
            //         ui.separator();
            //         let mouse_pos = ui.io().mouse_pos;
            //         ui.text(format!(
            //             "Mouse Position: ({:.1},{:.1})",
            //             mouse_pos[0], mouse_pos[1]
            //         ));
            //     });

            ui.show_demo_window(&mut true);
            window_ui.platform.prepare_render(ui, &mut self.window);
            
            window_application.render((self.window_width, self.window_height), window_ui);
            
            // window.swap_buffers();
            self.glfw.poll_events();
        }
    }



    /// Calculates shared intersecting area between 2 rectangles defined by their top left points and sizes.
    fn intersecting_area(rectangle_1_position: (i32, i32), rectangle_1_size: (i32, i32), rectangle_2_position: (i32, i32), rectangle_2_size: (i32, i32)) -> i32 {
        use std::cmp::{min, max};

        let rectangle_1_bottom_right = (
            rectangle_1_position.0 + rectangle_1_size.0,
            rectangle_1_position.1 + rectangle_1_size.1
        );
        let rectangle_2_bottom_right = (
            rectangle_2_position.0 + rectangle_2_size.0,
            rectangle_2_position.1 + rectangle_2_size.1
        );

        max(0,
            min(rectangle_2_bottom_right.0, rectangle_1_bottom_right.0) 
            - max(rectangle_2_position.0, rectangle_1_position.0))
        * max(0,
            min(rectangle_2_bottom_right.1, rectangle_1_bottom_right.1)
            - max(rectangle_2_position.1, rectangle_1_position.1))
    }

}