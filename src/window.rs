
pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,

    pub window_width: u32,
    pub window_height: u32,
}

impl Window {

    pub fn new(title: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut glfw = glfw::init(glfw::LOG_ERRORS)?;

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

        Ok(Window { glfw, window, events, window_width, window_height })
    }

    pub fn get_win32(&self) -> windows::Win32::Foundation::HWND {
        let window_handle = self.window.get_win32_window();
        unsafe { std::mem::transmute(window_handle) }
    }

    pub fn run<'a>(&mut self, render_callback: Box<dyn Fn((u32, u32)) + 'a>) {
        while !self.window.should_close() {
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("Got window event: {:?}", event);
    
                match event {
                    glfw::WindowEvent::FramebufferSize(_width, _height) => {
                        
                    }
                    glfw::WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, _) => {
                        self.window.with_window_mode(|mode| {
                            match mode {
                                glfw::WindowMode::Windowed => println!("Windowed"),
                                glfw::WindowMode::FullScreen(monitor) => println!("FullScreen({:?})", monitor.get_name()),
                            }
                        });

                        let monitor = glfw::Monitor::from_window(&self.window);
                        println!("got monitor {:?}", monitor);

                        self.glfw.with_connected_monitors(|_, monitors| {
                            for monitor in monitors.iter() {
                                println!("{:?}: {:?}", monitor.get_name(), monitor.get_video_mode());
                            }
                        });

                        // let video_mode = monitor.get_video_mode();
                        // println!("got video mode");
                        // let video_mode = video_mode.expect("asdsd");
                        // println!("got video mode2");
                        
                        // self.window.set_pos(0, 0);
                        // self.window.set_size(2560, 1440);

                        // self.window.set_monitor(glfw::WindowMode::FullScreen(&monitor), 0, 0, video_mode.width, video_mode.height, Some(video_mode.refresh_rate));
                        // println!("got set monitor");
                    }
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        println!("Closing the window.");
                        self.window.set_should_close(true);
                    }
                    _ => {}
                };
            }
        
            render_callback((self.window_width, self.window_height));
        
            // window.swap_buffers();
            self.glfw.poll_events();
        }
    }


    // fn get_current_monitor<'a>(glfw: &mut glfw::Glfw, window: &glfw::Window) -> &'a glfw::Monitor {
    //     let window_pos = window.get_pos();
    //     let window_size = window.get_size();

    //     let mut highest_overlap = 0;
    //     // let mut best_monitor: Option<&glfw::Monitor> = None;
    //     let mut best_monitor: Option<&'a glfw::Monitor> = None;
        
    //     glfw.with_connected_monitors(|_, monitors| {

    //         for monitor in monitors.iter() {
    //             let video_mode = monitor.get_video_mode().unwrap();
    //             let monitor_pos = monitor.get_pos();

    //             let overlap_area = Self::intersecting_area(window_pos, window_size, monitor_pos, (video_mode.width as i32, video_mode.height as i32));
    //             println!("Intersecting area for monitor {:?}: {:?}", monitor.get_name(), overlap_area);

    //             if overlap_area > highest_overlap {
    //                 highest_overlap = overlap_area;
    //                 best_monitor = Some(monitor);
    //             }
    //         }

    //         best_monitor.unwrap()
    //     })
    // }

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