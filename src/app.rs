use eframe::{self, egui::{Ui, Context, self}};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    fn draw_content(&mut self, ui: &mut Ui) {}
    fn draw_menubar(&mut self, ui: &mut Ui) {}
    fn draw_statusbar(&mut self, ui: &mut Ui) {}

    fn handle_shortcuts(&mut self, ctx: &Context) {}
}

impl eframe::App for App  {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| egui::menu::bar(ui, |ui| self.draw_menubar(ui)));
        egui::TopBottomPanel::bottom("StatusBar").show(ctx, |ui| self.draw_statusbar(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.draw_content(ui));
        
        self.handle_shortcuts(ctx);
    }
}