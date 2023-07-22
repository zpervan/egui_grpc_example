mod proto;
mod gui;

// fn main() {
//     eframe::run_native("gRPC Test Application",
//                        eframe::NativeOptions::default(),
//                        Box::new(|cc| Box::new(gui::application::TestApp::new(cc))))
//         .expect("Couldn't start application");
// }

// #[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(gui::application::TestApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}