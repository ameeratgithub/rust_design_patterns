trait WeaponBehavior {
    fn use_weapon(&self);
}

struct KnifeBehavior;
struct BowAndArrowBehavior;
struct AxeBehavior;
struct SwordBehavior;

impl WeaponBehavior for KnifeBehavior {
    fn use_weapon(&self) {
        println!("Just going to use knife...");
    }
}

impl WeaponBehavior for BowAndArrowBehavior {
    fn use_weapon(&self) {
        println!("I'm not going to worry if you're far enough...");
    }
}

impl WeaponBehavior for AxeBehavior {
    fn use_weapon(&self) {
        println!("I'm just gonna chop your head off with an axe...");
    }
}

impl WeaponBehavior for SwordBehavior {
    fn use_weapon(&self) {
        println!("My sword is sharp enough for your body...");
    }
}

struct Character {
    weapon: Box<dyn WeaponBehavior>,
}

impl Character {
    fn new(weapon: Box<dyn WeaponBehavior>) -> Self {
        Self{
            weapon
        }
    }
    fn fight(&self){
        self.weapon.use_weapon();
    }
    fn set_weapon(&mut self, weapon: Box<dyn WeaponBehavior>){
        self.weapon=weapon;
    }
}


type _King= Character;
type _Queen= Character;
type Knight= Character;
type _Troll= Character;

use crate::traits::{DesignPattern, DesignPatternFactory};

pub struct StrategyPatternPuzzle;
impl DesignPatternFactory for StrategyPatternPuzzle{
    fn new() -> Box<dyn DesignPattern> {
        Box::new(Self)
    }
}

impl DesignPattern for StrategyPatternPuzzle{
    

    fn run(&self) {
        let mut knight=Knight::new(Box::new(SwordBehavior));
        knight.fight();
        
        knight.set_weapon(Box::new(KnifeBehavior));
        knight.fight();
    }
}
