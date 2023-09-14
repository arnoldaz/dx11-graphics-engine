mod window_application;
mod extensions;




use window_application::WindowApplication;






fn main() {
    let mut application = WindowApplication::new("Test application!!!").unwrap();

    application.run();

}
