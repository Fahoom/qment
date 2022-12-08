mod document;
mod app;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Qment", options, Box::new(|_| Box::new(app::App::new())))
}
