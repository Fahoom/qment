use crate::document::Document;
use eframe::egui::{CentralPanel, Key, SidePanel, TextEdit, TopBottomPanel, Ui};

pub struct Editor {
    document: Document,
    current_question: Option<u32>,
    ghost_state: GhostState,
}

impl Editor {
    pub const fn new(document: Document) -> Self {
        Self {
            document,
            current_question: None,
            ghost_state: GhostState::Empty,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        SidePanel::left("QuestionExplorer").show_inside(ui, |ui| self.draw_sidebar(ui));
        CentralPanel::default().show_inside(ui, |ui| self.draw_content(ui));
    }

    pub fn draw_content(&mut self, ui: &mut Ui) {
        let Some(num) = self.current_question else { return; };
        let Some(question) = self.document.get_question_mut(num) else { return; };

        TopBottomPanel::top("TagPanel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tags");
                if ui.button("+").clicked() {
                    self.ghost_state = GhostState::TagGroup(String::new())
                }
            });

            for (name, tags) in question.groups_mut() {
                ui.horizontal_wrapped(|ui| {
                    let label = ui.label(name);
                    for tag in tags.iter() {
                        if ui.button(tag).clicked() {}
                    }
                });
            }

            if let GhostState::TagGroup(text) = &mut self.ghost_state {
                let resp = ui.add(TextEdit::singleline(text));

                if resp.lost_focus() || ui.input().key_pressed(Key::Escape) {
                    if !text.is_empty() && !question.has_group(&text) {
                        question.add_group(&text);
                    }
                    self.ghost_state = GhostState::Empty;
                }

                // Request after, otherwise we will never lose focus!
                resp.request_focus();
            }
        });

        ui.horizontal(|ui| {
            for (name, _) in question.sections() {
                ui.button(name);
            }
        });
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
                if let GhostState::Sidebar(text) = &mut self.ghost_state {
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
