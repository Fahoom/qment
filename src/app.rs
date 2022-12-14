use std::rc::Rc;

use crate::document::{preset::Preset, Document};
use eframe::{
    self,
    egui::{self, Context, Ui},
};

use self::editor::Editor;

mod editor;

pub struct App {
    editor: Option<Editor>,
    preset: Rc<Preset>,
}

impl App {
    pub fn new() -> Self {
        Self {
            editor: None,
            preset: Rc::new(Preset::default()),
        }
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

            if ui.button("Open Project").clicked() {
                self.open_project()
            }

            if ui.button("Save").clicked() {
                self.save()
            }

            if ui.button("Close").clicked() {
                self.editor = None
            }
        });

        ui.menu_button("Presets", |ui| {
            if ui.button("New Preset").clicked() {
                let preset = Preset::default();
                self.preset = Rc::new(preset);
            }

            if ui.button("Load Preset").clicked() {
                if let Some(file_path) = rfd::FileDialog::new().pick_file() {
                    if let Ok(text) = std::fs::read_to_string(&file_path) {
                        if let Ok(preset) = serde_json::from_str::<Preset>(&text) {
                            self.preset = Rc::new(preset);
                        }
                    }
                }
            }

            if ui.button("Save Preset").clicked() {
                if let Some(file_path) = rfd::FileDialog::new().save_file() {
                    if let Ok(file) = std::fs::File::create(file_path) {
                        if let Ok(_) = serde_json::to_writer_pretty(file, &Preset::default()) {
                            // Tada!
                        }
                    }
                }
            }
        });
    }

    fn draw_statusbar(&mut self, ui: &mut Ui) {}

    pub fn new_project(&mut self) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .save_file()
        {
            self.editor = Some(Editor::new(Document::new(), file_path, self.preset.clone()));
        }
    }

    pub fn open_project(&mut self) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            if let Ok(text) = std::fs::read_to_string(&file_path) {
                if let Ok(document) = serde_json::from_str::<Document>(&text) {
                    self.editor = Some(Editor::new(document, file_path, self.preset.clone()));
                }
            }
        }
    }

    pub fn save(&mut self) {
        if let Some(editor) = &self.editor {
            if let Ok(file) = std::fs::File::create(&editor.path) {
                let Ok(json) = serde_json::to_writer_pretty(file, &editor.document) else { return; };
            }
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
