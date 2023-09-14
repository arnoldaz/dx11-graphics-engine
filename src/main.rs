mod window_application;

use window_application::WindowApplication;


fn main() {
    let mut application = WindowApplication::new("Test application!!!").unwrap();

    application.run();

}
