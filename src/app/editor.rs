use eframe::egui::{Ui, CentralPanel, TopBottomPanel, TextEdit, Key, SidePanel};
use crate::document::Document;

pub struct Editor {
    document: Document,
    current_question: Option<u32>,
    ghost_state: GhostState
}

impl Editor {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            current_question: None, 
            ghost_state: GhostState::Empty
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        SidePanel::left("QuestionExplorer").show_inside(ui, |ui| self.draw_sidebar(ui));
        CentralPanel::default().show_inside(ui, |ui| self.draw_content(ui));
    }

    pub fn draw_content(&mut self, ui: &mut Ui) {
        
    }

    pub fn draw_sidebar(&mut self, ui: &mut Ui) {
        TopBottomPanel::top("SidebarHeader").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Questions");

                if ui.button("+").clicked() {
                    self.ghost_state = GhostState::Sidebar(String::new());
                }
            })
        });

        CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                for (num, _) in self.document.questions() {
                    if ui.button(num.to_string()).clicked() {
                        self.current_question = Some(*num)
                    }
                }

                // Ghost Rendering
                match &mut self.ghost_state {
                    GhostState::Sidebar(text) => {
                        let resp = ui.add(TextEdit::singleline(text));

                        if resp.lost_focus() || ui.input().key_pressed(Key::Escape) {
                            if let Ok(num) = text.parse::<u32>() {
                                self.document.add_question(num);
                            }

                            self.ghost_state = GhostState::Empty;
                        }

                        // Request after, otherwise we will never lose focus!
                        resp.request_focus();
                    }

                    _ => {}
                }
            });
        });
    }
}

#[derive(Default)]
enum GhostState {
    #[default]
    Empty,
    Sidebar(String),
    TagGroup(String),
    Tag(u32, String),
}
