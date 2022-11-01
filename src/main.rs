use character::{Character, Attack, Dice};
use eframe::{run_native, epi::App, egui::{CentralPanel, Vec2, SidePanel, Ui, DragValue, Frame}, NativeOptions};

use crate::distribution::Distribution;

mod character;
mod distribution;

struct Balance{
    characters: Vec<Character>
}

impl Balance{
    fn default() -> Balance{
        Balance { characters: vec![] }
    }
    
    fn character_list(&mut self,ctx: &eframe::egui::CtxRef, ui:&mut Ui){
        let mut remove_index:Option<usize> = None;
        for (i,character) in self.characters.iter_mut().enumerate(){
            if Self::character_listing(character, ui){
                remove_index = Some(i);
            }
        }
        if let Some(i) = remove_index{
            self.characters.remove(i);
        }

        if ui.button("Add Character").clicked() {
            self.characters.push(Character::default());
        }
    }

    fn character_listing(character:&mut Character, ui:&mut Ui) -> bool{
        ui.heading(character.name.clone());
        ui.horizontal(|ui| {
            ui.label("AC: ");
            ui.add(DragValue::new(&mut character.ac));

            ui.label("HP: ");
            ui.add(DragValue::new(&mut character.hp));
        });
        
        for att in character.attacks.iter_mut(){
            Self::attack_listing(att, ui);
        }


        if ui.button("Remove").clicked() {
            return true;
        }
        false
    }

    fn attack_listing(attack:&mut Attack, ui:&mut Ui){
        match attack {
            character::Attack::Weapon(weapon) => {
                ui.label("Weapon Attack");
                ui.horizontal(|ui|{
                    ui.label("To Hit: ");
                    ui.add(DragValue::new(&mut weapon.to_hit));
                    
                    ui.label("Damage: ");
                    ui.add(DragValue::new(&mut weapon.damage.amount));
                    ui.label("d");
                    ui.add(DragValue::new(&mut weapon.damage.value));
                    ui.add(DragValue::new(&mut weapon.damage.bonus));
                });
            },
            character::Attack::Spell(_) => {
                ui.label("Spell Attack");
            }
        }
    }
}



impl App for Balance{
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            SidePanel::left("character list").show(ctx, |ui |{
                self.character_list(ctx, ui)
            });
        });
    }

    fn name(&self) -> &str {
        "Balance"
    }
}



fn main() {
    let app = Balance::default();
    let mut win_options = NativeOptions::default(); 
    win_options.initial_window_size = Some(Vec2::new(520.,960.));

    run_native(Box::new(app), win_options)
}
