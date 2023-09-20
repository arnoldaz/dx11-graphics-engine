


pub struct WindowUi {


}

impl WindowUi {
    
    pub fn new(title: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // imgui.set_clipboard_backend(backend);


        Ok(WindowUi {  })
    }


}