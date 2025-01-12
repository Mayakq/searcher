use eframe::egui::{self, vec2, Layout, RichText, Ui};

#[derive(Default)]
pub enum Pallet {
    #[default]
    Text,
    RegularExpression,
}
#[derive(Default)]
pub enum TypeSearch {
    #[default]
    Absolute,
    Relative,
}
impl ToString for Pallet {
    fn to_string(&self) -> String {
        let mut string = String::with_capacity(4);
        match self {
            Pallet::Text => {
                string.push_str("Name");
                string
            }
            Pallet::RegularExpression => {
                string.push_str("Regular expression");
                string
            }
        }
    }
}
impl ToString for TypeSearch {
    fn to_string(&self) -> String {
        let mut string = String::with_capacity(8);
        match self {
            TypeSearch::Absolute => {
                string.push_str("Absolute");
                string
            }
            TypeSearch::Relative => {
                string.push_str("Relative");
                string
            }
        }
    }
}
#[derive(Default)]
pub struct Settings {
    path: String,
    pallet: Pallet,
    types: TypeSearch,
    file: String,
}

impl Settings {
    pub fn update(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Write path to start search");
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut self.path);
            });
            ui.add_space(15.0);
            ui.horizontal(|ui| {
                match self.pallet {
                    Pallet::Text => {
                        ui.label("Write file/folder name for search");
                    }
                    Pallet::RegularExpression => {
                        ui.label("Write regular expression for search");
                    }
                }
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut self.file);
            });
            ui.add_space(15.0);
            ui.horizontal(|ui| {
                let info = format!(
                    "Type: {}. Pallet: {}",
                    self.types.to_string(),
                    self.pallet.to_string()
                );
                ui.label("Select type search");
                ui.label(info);
                ui.add_space(5.0);
                ui.menu_button("Select type", |ui| {
                    if ui.button("Relative").clicked() {
                        self.types = TypeSearch::Relative;
                    };
                    if ui.button("Absolute").clicked() {
                        self.types = TypeSearch::Absolute;
                    };
                });
                ui.menu_button("Kind type", |ui| {
                    if ui.button("Name file/folder").clicked() {
                        self.pallet = Pallet::Text;
                    };
                    if ui.button("Regular expression").clicked() {
                        self.pallet = Pallet::RegularExpression;
                    };
                });
            });
        });
    }
    fn get_path(&self) -> &String {
        &self.path
    }
    fn get_mut_path(&mut self) -> &mut String {
        &mut self.path
    }
    fn get_file(&mut self) -> &String {
        &self.file
    }
    fn get_mut_file(&mut self) -> &mut String {
        &mut self.file
    }
    fn wigdet() {}
}
