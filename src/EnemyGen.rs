use eframe::epaint::vec2;
use rand::{seq::SliceRandom, rngs::ThreadRng, Rng};

use crate::character::{Character, AbilityScores, AbilityScoreTypes};

pub enum Class {
    Martial,
    Mage,
    SpellSword,
}

pub struct EnemyGenerator{
    adversaries : Character,
    characters: Vec<Character>,
    goons: Vec<Character>,
    health_p: f64,
    hit_p : f64,
    no_enemies:(usize,usize),
    class: Class,
}

impl From<&Character> for EnemyGenerator{
    fn from(av_character: &Character) -> Self {
        let pre_gen = Self {adversaries: av_character,
                                            health_p: 0.5,
                                            hit_p : 0.5,
                                            no_enemies:(1,0),
                                            class: Class::Fighter,
                                            goons: vec![],
                                            characters: vec![Character::default()]};
        pre_gen.generate()
    }
}

impl EnemyGenerator{
    fn generate(self) -> Self{
        let target_hp = self.health_p * self.adversaries.hp as f64;
        let to_hit = self.adversaries.ac - 20(1-self.hit_p);
        let dc = 8 + to_hit.clone();

        match self.class {
            Class::Martial => todo!(),
            Class::Mage => todo!(),
            Class::SpellSword => todo!(),
        }
    }

    fn gen_martial(self, to_hit:u32) -> Self{
        let mut rng = rand::thread_rng();
        let mut scores_mod = [0,0,-1,-2,-3,-4];
        let prof_bonus = ((rng.gen()*4.).round() as i8 +3);
        let primary_scores:Vec<i8> = vec![0,1,2].shuffle(&mut rng);
        let secondary_scores:Vec<i8> = vec![3,4,5].shuffle(&mut rng);
        
        for (score_i,scores_mod) in primary_scores.append(&mut secondary_scores).iter().zip(scores_mod){
            scores[score_i] = to_hit - prof_bonus - rng.gen_range(scores_mod..2*score_mod);
        }
        self.characters
    }
}