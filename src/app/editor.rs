use std::path::PathBuf;

use crate::document::Document;
use eframe::egui::{CentralPanel, Key, SidePanel, TextEdit, TopBottomPanel, Ui};

pub struct Editor {
    pub document: Document,
    pub path: PathBuf,

    current_question: Option<u32>,
    current_section: Option<String>,
    ghost_state: GhostState,
}

impl Editor {
    pub const fn new(document: Document, path: PathBuf) -> Self {
        Self {
            document,
            path,
            current_question: None,
            current_section: None,
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
                    self.ghost_state = GhostState::TagGroup(String::new(), None);
                }
            });

            let mut remove_group = None;

            for (name, tags) in question.groups_mut() {
                ui.horizontal_wrapped(|ui| {
                    let label = ui.button(name);
                    
                    for tag in tags.iter() {
                        if ui.button(tag).clicked() {}
                    }

                    if ui.button("+").clicked() {
                        self.ghost_state = GhostState::Tag(String::new());
                    }

                    if let GhostState::Tag(text) = &mut self.ghost_state {
                        let resp = ui.add(TextEdit::singleline(text));
                        if resp.lost_focus() || ui.input().key_pressed(Key::Escape) {
                            if !text.is_empty() {
                                tags.insert(text.to_owned());
                            }
                            self.ghost_state = GhostState::Empty;
                        }
                        resp.request_focus();
                    }
                    
                    label.context_menu(|ui| {
                        if ui.button("Delete").clicked() {
                            // Cant reborrow question mutably, so setup vars to remove after mut borrow
                            remove_group = Some(name.clone());
                        }
                        if ui.button("Rename").clicked() {
                            self.ghost_state = GhostState::TagGroup(name.clone(), Some(name.clone()));
                        }
                    });
                    
                });
            }

            if let Some(removed_group_name) = &remove_group {
                question.remove_group(removed_group_name);
            }

            if let GhostState::TagGroup(text, original) = &mut self.ghost_state {
                let resp = ui.add(TextEdit::singleline(text));

                if resp.lost_focus() || ui.input().key_pressed(Key::Escape) {
                    if !text.is_empty() {
                        if let Some(old_name) = original {
                            question.rename_group(old_name, &text);
                        } else if !question.has_group(&text) {
                            question.add_group(&text);
                        } 
                    }
                    self.ghost_state = GhostState::Empty;
                }

                // Request after, otherwise we will never lose focus!
                resp.request_focus();
            }
        });

        ui.horizontal(|ui| {
            for (name, _) in question.sections() {
                if ui.button(name).clicked() {
                    self.current_section = Some(name.into());
                }
            }
        });

        let Some(section_name) = &self.current_section else { return; };
        let Some(section) = question.get_section_mut(section_name) else { return; };

        if let Some(text) = &mut section.text {
            ui.add(TextEdit::multiline(text));
        } else {
            if ui.button("Add Text").clicked() {
                section.text = Some(String::new());
            }
        }
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
    TagGroup(String, Option<String>),
    Tag(String),
}
