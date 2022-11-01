use std::ops::{Add, Mul};

use crate::character::Dice;

#[derive(Clone,Debug,PartialEq)]
pub struct Distribution {
    pub min : usize,
    pub max : usize,
    pub pdf : Vec<f64>,
}

impl From<Dice> for Distribution{
    fn from(dice: Dice) -> Self {
        let single_dice = Self {min:1, max:dice.value as usize, pdf:vec![1./(dice.value as f64);dice.value as usize]};
        let mut result = single_dice.clone();
        for _ in 0..dice.amount-1 {
            result = result + single_dice.clone();
        }
        result
    }
}

impl Add for Distribution{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let min = self.min + rhs.min;
        let max = self.max + rhs.max;
        let mut result_vec = vec![0.;max- min +1];
        let mut total = 0.;

        for (i,p_i) in self.pdf.iter().enumerate(){
            for (j,q_j) in rhs.pdf.iter().enumerate(){
                result_vec[i+j] += p_i * q_j;
                total += p_i * q_j;
            }
        }
        
        let result: Vec<f64> = result_vec.iter().map(|p| p/total).collect();
        Self {min, max, pdf:result}
    }   
}

impl Mul<f32> for Distribution{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        if rhs == 0.{
            return Self::default();
        }
        let max = (self.max as f32 * rhs).floor() as usize;
        let min = (self.min as f32 * rhs).floor() as usize;
        let mut result_vec =vec![0.;max-min+1];
        for (i,p_i) in self.pdf.iter().enumerate(){
            let index = (i as f32* rhs).floor() as usize;
            result_vec[index] += p_i;
        }

        Self {min, max, pdf:result_vec}
    }
}

impl Mul<Distribution> for f32{
    type Output = Distribution;

    fn mul(self, rhs: Distribution) -> Self::Output {
        if self == 0.{
            return Self::Output::default();
        }
        let max = (rhs.max as f32 * self).floor() as usize;
        let min = (rhs.min as f32 * self).floor() as usize;
        let mut result_vec =vec![0.;max-min+1];
        for (i,p_i) in rhs.pdf.iter().enumerate(){
            let index = (i as f32* self).floor() as usize;
            result_vec[index] += p_i;
        }

        Self::Output {min, max, pdf:result_vec}
    }
}


impl Distribution{
    pub fn default()-> Self{
        Self {min: 0, max:0, pdf: vec![(1.)]}
    }
    pub fn prob_greater_than(&self,value:usize)-> f64{
        if value>=self.max{
            0.
        }else if value <self.min{
            1.
        }else {
            let mut result = 0.;
            for val in self.pdf[value-self.min..].iter(){
                result += val;
            }
            result
        }
    }
}