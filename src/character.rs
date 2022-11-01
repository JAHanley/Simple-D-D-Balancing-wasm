use std::{collections::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::distribution::{Distribution};

#[allow(clippy::upper_case_acronyms)]
#[derive(Default,Clone, Copy,PartialEq,Eq,Debug)]
pub enum AbilityScoreTypes{
    STR,
    #[default] DEX,
    CON,
    WIS,
    INT,
    CHA,
}

#[derive(Default,Debug,Hash,EnumIter,PartialEq, Eq,Clone, Copy)]
pub enum DamageTypes {
    #[default] Slashing,
    Bludgeoning,
    Piercing,
    Fire,
    Cold,
    Thunder,
    Lighting,
    Psychic,
    Radiant,
    Force,
    Necrotic,
    Poison,
    Acid,
}

#[derive(Default,Clone, Copy)]
pub struct AbilityScores{
    pub strength: i8,
    pub dex: i8,
    pub con: i8,
    pub wis: i8,
    pub int: i8,
    pub cha: i8,
}

impl From<&Vec<Character>> for AbilityScores{
    fn from(enemies: &Vec<Character>) -> Self {
        let n = enemies.len() as f32;
        let mut result = AbilityScores::default();

        for villain in enemies.iter(){
            result.strength += villain.scores.strength;
            result.dex += villain.scores.dex;
            result.con += villain.scores.con;
            result.wis += villain.scores.wis;
            result.int += villain.scores.int;
            result.cha += villain.scores.cha;
        }

        result.strength = (result.strength as f32 / n).round() as i8;
        result.con += (result.dex as f32 / n).round() as i8;
        result.dex += (result.con as f32 / n).round() as i8;
        result.int += (result.wis as f32 / n).round() as i8;
        result.wis += (result.int as f32 / n).round() as i8;
        result.cha += (result.cha as f32 / n).round() as i8;

        result
    }
}

impl From<Vec<i8>> for AbilityScores{
    fn from(input: Vec<i8>) -> Self {
        if input.len() != 6 {
            Self::default()
        } else {
            Self {
                strength: input[0],
                dex: input[1],
                con: input[2],
                wis: input[3],
                int: input[4],
                cha: input[5],
            }
        }
    }
}

impl AbilityScores {
    pub fn check_score(&self, score:AbilityScoreTypes)->i8{
        match score{
            AbilityScoreTypes::STR => self.strength,
            AbilityScoreTypes::DEX => self.dex,
            AbilityScoreTypes::CON => self.con,
            AbilityScoreTypes::WIS => self.wis,
            AbilityScoreTypes::INT => self.int,
            AbilityScoreTypes::CHA => self.cha,
        }
    }
}

pub struct Character{
    pub name: String,
    pub hp: u32,
    pub ac: u32,
    pub attacks: Vec<Attack>,
    pub scores: AbilityScores,
    pub damage_type_mods: HashMap<DamageTypes,f32>,
    pub modifier_flag: bool,
}

impl From<&Vec<Character>> for Character{
    fn from(enemies:&Vec<Character> ) -> Self{
        let n = enemies.len() as u32;
        if n == 0{
            return Self::default();
        }
        let mut modifiers: HashMap<DamageTypes,f32> = HashMap::new();
        let mut hp = 0;
        let mut ac = 0;
        for char in enemies.iter(){
            hp += char.hp;
            ac += char.ac;
            
            for (key, value) in char.damage_type_mods.iter(){
                *modifiers.entry(*key).or_insert(0.) += *value;
                if *key == DamageTypes::Slashing{
                    //print!("{:?}",value);
                }
                
            }
        }
        for key in DamageTypes::iter(){
            modifiers.entry(key).and_modify(|v| *v/=n as f32);
            
        }
        
        hp /= n;
        ac /= n;
        let scores = AbilityScores::from(enemies);

        Self { name: String::from("av"), hp, ac, attacks: vec![], scores, damage_type_mods: modifiers ,modifier_flag: false}   
        
    }
    
}

impl Character{
    pub fn default() -> Character{
        Character { name: "gregg".to_string(), hp: 10, ac: 10, attacks: vec![Attack::default()], scores: AbilityScores::default(),damage_type_mods: Self::gen_damage_type_mods(),modifier_flag: false }
    }

    fn gen_damage_type_mods() -> HashMap<DamageTypes,f32> {
        HashMap::from_iter(DamageTypes::iter().map(|t| (t,1.)))
    }

    // pub fn attack(self,ac:i32) -> i32{
    //     self.attacks.iter().fold(0,| sum,att | sum+att.attack(ac))
    // }
    pub fn attack_distribution(&self,target:&Character) -> Distribution{
        self.attacks.iter().fold(Distribution::default(),| sum,att | sum+att.attack_distribution(target))
    }

}

#[derive(PartialEq,Eq,Clone)]
pub enum Attack{
    Weapon(WeaponAttack),
    Spell(SpellAttack),
}
impl Attack{
    pub fn default() -> Attack {
        Attack::Weapon(WeaponAttack::default())
    }

    // fn attack(&self, ac:i32) -> i32 {
    //     match self{
    //         Attack::Weapon(a) => a.attack(ac),
    //         Attack::Spell(a) => 0,
    //     }
    // }
    pub fn attack_distribution(&self,target: &Character) -> Distribution{
        match self{
            Attack::Weapon(a) => *target.damage_type_mods.get(&a.damage_type).unwrap_or(&1.) * a.distribution(target.ac),
            Attack::Spell(a) => *target.damage_type_mods.get(&a.damage_type).unwrap_or(&1.) * a.distribution(target.scores),
        }
    }
}


#[derive(PartialEq,Eq,Clone,Default)]
pub struct WeaponAttack{
    pub to_hit: i32,
    pub damage: Dice,
    pub damage_type: DamageTypes,
    pub id : isize,
}

impl WeaponAttack{
    fn default() -> Self{
        Self { to_hit: 0, damage: Dice::default(), damage_type: DamageTypes::default(), id: rand::random() }
    }

    // fn attack(&self,ac:i32) ->i32 {
    //     let to_hit_roll: i32 = (20-ac + self.to_hit)/20;
    //     let crit = (self.damage.no_bonus_roll() as f32 /20.).floor() as i32;
    //     to_hit_roll * self.damage.roll()  + crit
    // }
    fn distribution(&self,ac:u32) -> Distribution{
        let real_ac = ((ac as i32) - 1 - self.to_hit) as f64;
        let miss_chance: f64;
        if real_ac >= 20.{
            miss_chance = 1.;
        } else if real_ac <=0.{
            miss_chance = 0.;
        } else {
            miss_chance = real_ac / 20.;
        }
        let dice = Distribution::from(self.damage);
        dice.attack_distribution(miss_chance)
    }
}

impl Distribution{
    fn attack_distribution(&self,miss_chance: f64) -> Self{
        let mut pdf = vec![0.;self.max+1];
        let hit_chance = 1.-miss_chance;
        pdf[0] = miss_chance;
        for (i,p_i) in self.pdf.iter().enumerate(){
            pdf[i+self.min] += p_i * hit_chance;
        }
        Self {min: 0, max: self.max, pdf}

    }
}

#[derive(PartialEq,Eq,Clone)]
pub struct SpellAttack{
    pub dc:i32,
    pub save:AbilityScoreTypes,
    pub damage:Dice,
    pub damage_type:DamageTypes,
    pub details:String,
    pub id : isize,
}
impl SpellAttack {
    pub fn default() -> Self {  
        Self {
            dc: 10,
            save: AbilityScoreTypes::default(),
            damage: Dice::default(),
            damage_type: DamageTypes::default(),
            details: String::default(),
            id: rand::random(),
        }


    }


    fn distribution(&self,saves: AbilityScores ) -> Distribution{
        let enemy_bonus = saves.check_score(self.save);
        
        
        let dc_roll = self.dc-enemy_bonus as i32;
        let fail_chance:f64;
        if dc_roll>=20{
            fail_chance=1.;
        } else if dc_roll<=1 {
            fail_chance=0.;
        } else{
            fail_chance = dc_roll as f64/20.
        }

        let dice_distro = Distribution::from(self.damage);
        dice_distro.attack_distribution(fail_chance)
    }
}

#[derive(Clone, Copy,PartialEq,Eq)]
pub struct Dice{
    pub amount: i32,
    pub value: u32,
    pub bonus: i32,
}

impl Default for Dice{
    fn default() -> Dice{
        Dice { amount: 1, value: 2, bonus: 0 }
    }
}

// impl Dice{
//     fn roll(&self)->i32{
//         self.amount * (((self.value +1) as f32 / 2.).floor() as i32) + self.bonus
//     }

//     fn no_bonus_roll(&self) ->i32 {
//         self.amount * (((self.value +1) as f32 / 2.).floor() as i32)
//     }
// }