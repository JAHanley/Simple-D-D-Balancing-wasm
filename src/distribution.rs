use std::ops::Add;

use crate::character::Dice;

#[derive(Clone,Debug)]
pub struct Distribution {
    min : usize,
    max : usize,
    pdf : Vec<f64>,
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
                result_vec[i+j] += p_i + q_j;
                total += p_i + q_j;
            }
        }
        
        let result: Vec<f64> = result_vec.iter().map(|p| p/total).collect();
        Self {min:min, max: max, pdf:result}
    }   
}

impl Distribution{
    fn default()-> Self{
        Self {min: 1, max:1, pdf: vec![(1.)]}
    }
}