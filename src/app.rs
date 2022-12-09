use crate::document::Document;
use eframe::{
    self,
    egui::{self, Context, Ui},
};

use self::editor::Editor;

mod editor;

pub struct App {
    editor: Option<Editor>,
}

impl App {
    pub const fn new() -> Self {
        Self { editor: None }
    }

    fn draw_content(&mut self, ui: &mut Ui) {
        if let Some(editor) = &mut self.editor {
            editor.draw(ui);
        } else {
            ui.centered_and_justified(|ui| ui.heading("No Project Open"));
        }
    }

    fn draw_menubar(&mut self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("New Project").clicked() {
                self.new_project()
            }
        });
    }

    fn draw_statusbar(&mut self, ui: &mut Ui) {}

    pub fn new_project(&mut self) {
        if let Some(_) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .save_file()
        {
            self.editor = Some(Editor::new(Document::new()));
        }
    }

    // TODO
    fn handle_shortcuts(&mut self, ctx: &Context) {}
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("MenuBar")
            .show(ctx, |ui| egui::menu::bar(ui, |ui| self.draw_menubar(ui)));
        egui::TopBottomPanel::bottom("StatusBar").show(ctx, |ui| self.draw_statusbar(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.draw_content(ui));

        self.handle_shortcuts(ctx);
    }
}
