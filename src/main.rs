use character::*;
use eframe::{App, egui::{CentralPanel, SidePanel, Ui, DragValue, plot::{Bar, Plot, Legend}, ComboBox,  ScrollArea, TopBottomPanel, Window, Context}};
use eframe::egui;
use eframe::egui::plot::BarChart;
use strum::IntoEnumIterator;
use std::collections::HashMap;

use crate::distribution::Distribution;

mod character;
mod distribution;
//mod EnemyGen;

struct Balance{
    characters: Vec<Character>,
    enemies: Vec<Character>,
    av_player: Character,
    av_enemy: Character,
    turns: i32,
    gen_popup: bool,
}

impl Balance{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn default() -> Balance{
        Balance { characters: vec![] , enemies: vec![], av_player: Character::default(), av_enemy: Character::default(),turns: 1, gen_popup:false}
    }
    fn average_display(&self, ui:&mut Ui){
        ui.horizontal(|ui|{
            egui::Frame::none().fill(egui::Color32::DARK_BLUE)
                .show(ui,|ui| {
                    ui.vertical(|ui|{
                        ui.heading("Average Player");
                        Self::display_damage_modifiers( &self.av_player.damage_type_mods, ui);
                        ui.push_id("av_player", |ui|{
                        Self::basic_character_display(&self.av_player, ui)
                        })
                    });
                });

            egui::Frame::none().fill(egui::Color32::DARK_RED)
                .show(ui,|ui| {
                    ui.vertical(|ui|{
                        ui.heading("Average Villain");
                        Self::display_damage_modifiers(&self.av_enemy.damage_type_mods, ui);
                        ui.push_id("av_villain", |ui|{
                        Self::basic_character_display(&self.av_enemy, ui)
                        })
                    });
                    

                });
        });
        
    }

    fn basic_character_display(character: &Character, ui:&mut Ui){
        
        ui.vertical(|ui|{
            ui.label(format!("HP: {}    AC: {}",character.hp,character.ac));
            ui.horizontal(|ui|{
                ui.label(format!("STR: {}",character.scores.strength));
                ui.label(format!("DEX: {}",character.scores.dex));
                ui.label(format!("CON: {}",character.scores.con));
            });
            ui.horizontal(|ui|{
                ui.label(format!("WIS: {}",character.scores.wis));
                ui.label(format!("INT: {}",character.scores.int));
                ui.label(format!("CHA: {}",character.scores.cha));
            });
        });
        
    }
    
    fn character_list(characters:&mut Vec<Character>,ctx:&Context, ui:&mut Ui){
        let mut remove_index:Option<usize> = None;
        for (i,character) in characters.iter_mut().enumerate(){
            if Self::character_listing(i,character,ctx, ui){
                remove_index = Some(i);
            }
        }
        if let Some(i) = remove_index{
            characters.remove(i);
        }

        if ui.button("Add Character").clicked() {
            characters.push(Character::default());
        }
    }

    fn character_listing(index:usize,character:&mut Character,ctx:&Context, ui:&mut Ui) -> bool{
        //ui.heading(character.name.clone());
        ui.text_edit_singleline(&mut character.name);
        ui.horizontal(|ui| {
            ui.label("AC: ");
            ui.add(DragValue::new(&mut character.ac));

            ui.label("HP: ");
            ui.add(DragValue::new(&mut character.hp));
        });
        


        ui.push_id(index,|ui|{
            ui.collapsing("Ability Scores", |ui|{

                ui.horizontal(|ui|{
                    ui.add(DragValue::new(&mut character.scores.strength).prefix("Str: ").speed(0.1));
                    ui.add(DragValue::new(&mut character.scores.dex).prefix("Dex: ").speed(0.1));
                    ui.add(DragValue::new(&mut character.scores.con).prefix("Con: ").speed(0.1));
                });
                ui.horizontal(|ui|{
                    ui.add(DragValue::new(&mut character.scores.wis).prefix("Wis: ").speed(0.1));
                    ui.add(DragValue::new(&mut character.scores.int).prefix("Int: ").speed(0.1));
                    ui.add(DragValue::new(&mut character.scores.cha).prefix("Cha: ").speed(0.1));

                });
            });
            
            Self::display_damage_modifiers(&character.damage_type_mods, ui);
            Self::edit_damage_modifiers(character,ctx, ui);

            ui.collapsing("Attacks", |ui| {
                let mut remove_index:Option<usize> = None;
                for (i,att) in character.attacks.iter_mut().enumerate(){
                    if Self::attack_listing(att, ui) {
                        remove_index = Some(i);
                    }
                }
                if let Some(i) = remove_index{
                    character.attacks.remove(i);
                }

                if ui.button("Add attack").clicked() {
                    character.attacks.push(Attack::default());
                }
                
            });
        });
        
        
        if ui.button("Remove").clicked() {
            return true;
        }
        false
    }

    fn edit_weapon_attack(weapon: &mut WeaponAttack,ui:&mut Ui){
        ui.horizontal(|ui|{
            ui.label("Weapon Attack");
            ui.push_id(weapon.id, |ui|{
                ComboBox::from_label("Damage")
                .selected_text(format!("{:?}", &weapon.damage_type))
                .show_ui(ui, |ui|{
                    for dam in DamageTypes::iter(){
                        ui.selectable_value(&mut weapon.damage_type, dam, format!("{:?}",dam));
                    }
                });
            });
            
        });
        
        ui.horizontal(|ui|{
            ui.label("To Hit: ");
            ui.add(DragValue::new(&mut weapon.to_hit));
            
            ui.label("Damage: ");
            ui.add(DragValue::new(&mut weapon.damage.amount).clamp_range(1..=20));
            ui.label("d");
            ui.add(DragValue::new(&mut weapon.damage.value).clamp_range(2..=100));
            ui.add(DragValue::new(&mut weapon.damage.bonus));
        });
    }

    fn edit_spell_attack(spell:&mut SpellAttack,ui:&mut Ui){
        ui.horizontal(|ui|{
            ui.label("Spell Attack");
            ui.push_id(spell.id, |ui|{
                ComboBox::from_label("Damage")
                .selected_text(format!("{:?}", &spell.damage_type))
                .show_ui(ui, |ui|{
                    for dam in DamageTypes::iter(){
                        ui.selectable_value(&mut spell.damage_type, dam, format!("{:?}",dam));
                    }
                });
            });
            
        });


        ui.horizontal(|ui|{
            ui.label("DC: ");
            ui.add(DragValue::new(&mut spell.dc));

            ComboBox::from_label("Save")
                .selected_text(format!("{:?}",spell.save)).width(10.)
                .show_ui(ui, |ui|{
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::STR, "STR");
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::DEX, "DEX");
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::CON, "CON");
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::WIS, "WIS");
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::INT, "INT");
                    ui.selectable_value(&mut spell.save, AbilityScoreTypes::CHA, "CHA");
                    
                });
        });
        ui.horizontal(|ui|{
            ui.label("Damage: ");
            ui.add(DragValue::new(&mut spell.damage.amount).clamp_range(1..=20));
            ui.label("d");
            ui.add(DragValue::new(&mut spell.damage.value).clamp_range(2..=100));
            ui.add(DragValue::new(&mut spell.damage.bonus));
        });
        ui.text_edit_multiline(&mut spell.details);
    }


    fn attack_listing(attack:&mut Attack, ui:&mut Ui) -> bool{
        ui.horizontal(|ui|{
            ui.label("Type");
            ui.radio_value(attack, Attack::Weapon(WeaponAttack::default()), "Weapon");
            ui.radio_value(attack, Attack::Spell(SpellAttack::default()), "Spell");
        });
        
        match attack {
            character::Attack::Weapon(weapon) => {
                Self::edit_weapon_attack(weapon, ui)
            },
            character::Attack::Spell(spell) => {
                Self::edit_spell_attack(spell, ui)
            }
        }
        if ui.button("Remove Attack").clicked(){
            return true;
        }
        false
    }

    fn main_dashboard(&mut self, ui:&mut Ui){
        ui.push_id("player_dam_distro", |ui|{
            ui.heading("Player Damage distribution");
            ui.add(DragValue::new(&mut self.turns).clamp_range(1..=20).prefix("Turns: "));
            ui.label(format!("Average enemy AC: {:?}",self.av_enemy.ac));

            let mut attack_distro = Distribution::default();
            for _ in 0..self.turns{
                attack_distro =attack_distro + self.characters.iter().fold(
                    Distribution::default(),
                    | result, c| { result + c.attack_distribution(&self.av_enemy)});
            }
            ui.label(format!("Probability the enemies are defeated by turn {}: {:.4}",self.turns, attack_distro.prob_greater_than(self.av_enemy.hp as usize)));

            attack_distro.plot(ui);
        });

        ui.push_id("enemy_dam_distro", |ui|{
            ui.heading("Incoming Damage per round");

            self.enemies.iter().fold(
                Distribution::default(),
                | result, c| {
                    result + c.attack_distribution(&self.av_player)
                }).plot(ui);
        });
    }


    fn enemy_generator_panel(&mut self,ctx:&Context, ui:&mut Ui){
        if ui.button("Generate Villains").clicked(){
            self.gen_popup = ! self.gen_popup;
        }

        if self.gen_popup{
            Window::new("Enemy Generator")
                .show(ctx, |ui|{
                    ui.heading("Generate Enemies");
                    ui.label("Under Construction"); 
                });
        }
        
    }

    fn display_damage_modifiers(modifiers:&HashMap<DamageTypes,f32>,ui:&mut Ui){
        let mut resistances = String::default();
        let mut immunities = String::default();
        let mut vulnerabilities = String::default();

        for (key,value) in modifiers.iter(){
            let string_key = format!("{:?},",key);
            if value > &1. {
                vulnerabilities.push_str(&string_key);
            } else if value == &0.{
                immunities.push_str(&string_key); 
            } else if value < &1. {
                resistances.push_str(&string_key);
            }
        }
        ui.label(format!("Resistances: {}", resistances));
        ui.label(format!("Immunities: {}", immunities));
        ui.label(format!("Vulnerabilities {}",vulnerabilities));
    }

    fn edit_damage_modifiers(character: &mut Character,ctx:&Context,ui:&mut Ui){
        let modifiers =&mut character.damage_type_mods;
        if ui.button("Change").clicked(){
            character.modifier_flag =  ! character.modifier_flag ;
        }
        Window::new("Modifiers").collapsible(true)
                .open(&mut character.modifier_flag)
                .show(ctx, |ui|{
                    let mods_str = ["0","0.5","1","2"];
                    let mods = [0.,0.5,1.,2.];
                    let mut i = 0;
                    ui.heading("Modifiers");

                    for dam in DamageTypes::iter(){
                        ui.push_id(format!("{:?}",dam), |ui|{
                            let val = *modifiers.get(&dam).unwrap_or(&1.);
                            if val == 0. {
                                i = 0;
                            } else if val < 1.{
                                i = 1;
                            } else if val >1. {
                                i = 3;
                            } else {
                                i=2;
                            }

                            ComboBox::from_label(format!("{:?}",dam))
                                .show_index(ui, &mut i, mods.len(), |i| mods_str[i].to_owned());
                            
                            modifiers.insert(dam, mods[i]);
                        });
                        
                 
                    }

                    
        });
        
    }

}

impl Distribution{
    fn plot(self, ui:&mut Ui){
        let plot = BarChart::new(
            self.pdf.iter().enumerate().map(|(damage,attack)| Bar::new(damage as f64 + 0.5,*attack).width(1.)).collect()
        );
        


        Plot::new("Damage chart")
            .legend(Legend::default())
            .data_aspect((self.max - self.min) as f32 + 0.5).width(1000.).height(300.).include_x((self.max + self.min) as f32*0.5)
            .show(ui, |plot_ui|{
                plot_ui.bar_chart(plot);
            });
    }
}


impl App for Balance{
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.av_player = Character::from(&self.characters);
        self.av_enemy = Character::from(&self.enemies);


        SidePanel::left("character list").show(ctx, |ui |{
            ScrollArea::vertical().show(ui, |ui|{
                ui.heading("Players");
                Self::character_list(&mut self.characters, ctx,ui);
            });
                    
        });
        CentralPanel::default().show(ctx, |ui| {
            self.main_dashboard(ui);
        });
        SidePanel::right("Enemy List").show(ctx, |ui|{
            ui.heading("Villains");
            self.enemy_generator_panel(ctx,ui);
            Self::character_list(&mut self.enemies, ctx,ui);
        });
        TopBottomPanel::bottom("av").show(ctx, |ui|{
            self.average_display(ui)
        });

        
    }
}



#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let win_options = eframe::NativeOptions::default(); 
    //win_options.initial_window_size = Some(Vec2::new(520.,960.));

    eframe::run_native("D&D Balance", win_options, Box::new(|cc| Box::new(Balance::new(cc))));
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "D&D Balance", // hardcode it
        web_options,
        Box::new(|cc| Box::new(Balance::new(cc))),
    )
    .expect("failed to start eframe");
}