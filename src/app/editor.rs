use std::path::PathBuf;

use crate::document::Document;
use eframe::egui::{
    Button, CentralPanel, Frame, Key, ScrollArea, Separator, SidePanel, TextEdit, TopBottomPanel,
    Ui,
};

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

        TopBottomPanel::top("TagPanel")
            .frame(Frame::default())
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Tags");
                    if ui.add(Button::new("+").frame(false)).clicked() {
                        self.ghost_state = GhostState::TagGroup(String::new(), None);
                    }
                });

                ui.add_space(5.0);

                let mut remove_group = None;

                for (name, tags) in question.groups_mut() {
                    ui.horizontal_wrapped(|ui| {
                        let label = ui.add(Button::new(name).frame(false));
                        ui.add(Separator::default().vertical());

                        let mut remove_tag = None;
                        for tag in tags.iter() {
                            if ui.add(Button::new(tag).frame(false)).clicked() {
                                remove_tag = Some(tag.to_owned());
                            }
                        }

                        if let Some(remove_tag_name) = remove_tag {
                            tags.remove(&remove_tag_name);
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
                                self.ghost_state =
                                    GhostState::TagGroup(name.clone(), Some(name.clone()));
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

        ui.add(Separator::default().horizontal().spacing(20.0));

        ui.horizontal(|ui| {
            for (name, _) in question.sections() {
                if ui.button(name).clicked() {
                    self.current_section = Some(name.into());
                }
            }
        });

        CentralPanel::default().show_inside(ui, |ui| {
            let Some(section_name) = &self.current_section else { return; };
            let Some(section) = question.get_section_mut(section_name) else { return; };

            if let Some(text) = &mut section.text {
                ui.add(TextEdit::multiline(text).desired_width(ui.available_width()));
            } else {
                if ui.button("Add Text").clicked() {
                    section.text = Some(String::new());
                }
            }
        });
    }

    pub fn draw_sidebar(&mut self, ui: &mut Ui) {
        TopBottomPanel::top("SidebarHeader").show_inside(ui, |ui| {
            let highest = self.document.questions().fold(u32::MIN, |a, b| a.max(*b.0));

            ui.horizontal(|ui| {
                ui.label("Questions");

                if ui.button("+").clicked() {
                    self.ghost_state = GhostState::Sidebar((highest + 1).to_string());
                }
            })
        });

        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
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
