mod proto;
mod gui;

fn main() {
    eframe::run_native("gRPC Test Application",
                       eframe::NativeOptions::default(),
                       Box::new(|cc| Box::new(gui::application::TestApp::new(cc))))
        .expect("Couldn't start application");
}
