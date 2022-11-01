

pub struct Character{
    pub name: String,
    pub hp: u32,
    pub ac:u32,
    pub attacks: Vec<Attack>,
}
impl Character{
    pub fn default() -> Character{
        Character { name: "gregg".to_string(), hp: 10, ac: 10, attacks: vec![Attack::default()] }
    }

    pub fn attack(self,ac:i32) -> i32{
        self.attacks.iter().fold(0,| sum,att | sum+att.attack(ac))
    }
}

pub enum Attack{
    Weapon(WeaponAttack),
    Spell(SpellAttack),
}
impl Attack{
    fn default() -> Attack {
        Attack::Weapon(WeaponAttack::default())
    }

    fn attack(&self, ac:i32) -> i32 {
        match self{
            Attack::Weapon(a) => a.attack(ac),
            Attack::Spell(a) => 0,
        }
    }
}


pub struct WeaponAttack{
    pub to_hit: i32,
    pub damage: Dice,
}

impl WeaponAttack{
    fn default() -> WeaponAttack{
        WeaponAttack { to_hit: 0, damage: Dice::default() }
    }

    fn attack(&self,ac:i32) ->i32 {
        let to_hit_roll: i32 = (20-ac + self.to_hit)/20;
        let crit = (self.damage.no_bonus_roll() as f32 /20.).floor() as i32;
        to_hit_roll * self.damage.roll()  + crit
    }
}

pub struct SpellAttack;

pub struct Dice{
    pub amount: i32,
    pub value: u32,
    pub bonus: i32,
}

impl Dice{
    fn default() -> Dice{
        Dice { amount: 1, value: 6, bonus: 0 }
    }

    fn roll(&self)->i32{
        self.amount * (((self.value +1) as f32 / 2.).floor() as i32) + self.bonus
    }

    fn no_bonus_roll(&self) ->i32 {
        self.amount * (((self.value +1) as f32 / 2.).floor() as i32)
    }
}