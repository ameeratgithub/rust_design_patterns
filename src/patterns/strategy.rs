trait FlyBehavior {
    fn fly(&self);
}

trait QuackBehavior {
    fn quack(&self);
}

struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("Just flying...");
    }
}

struct FlyNoFly;
impl FlyBehavior for FlyNoFly {
    fn fly(&self) {
        println!("Sorry, can't fly...");
    }
}

struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quacking...");
    }
}

struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Squeaking...");
    }
}

struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("Can't Quack...");
    }
}

struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}

impl Duck {
    fn fly(&self) {
        self.fly_behavior.fly();
    }
    fn quack(&self) {
        self.quack_behavior.quack();
    }
    fn swim(&self) {
        println!("All ducks should know how to swim!");
    }
}

struct DuckCall {
    fly_behavior: FlyNoFly,
    quack_behavior: Quack,
}

impl DuckCall {
    fn new() -> Self {
        Self {
            fly_behavior: FlyNoFly,
            quack_behavior: Quack,
        }
    }

    fn fly(&self) {
        self.fly_behavior.fly();
    }
    fn quack(&self) {
        self.quack_behavior.quack();
    }
}

use crate::traits::{DesignPattern, DesignPatternFactory};

pub struct StrategyPattern;

impl DesignPatternFactory for StrategyPattern{
    fn new() -> Box<dyn DesignPattern> {
        Box::new(Self)
    }
}

impl DesignPattern for StrategyPattern {

    fn run(&self) {
        let mut duck = Duck {
            fly_behavior: Box::new(FlyWithWings),
            quack_behavior: Box::new(Quack),
        };

        duck.fly();
        duck.quack();

        duck.fly_behavior = Box::new(FlyNoFly);
        duck.quack_behavior = Box::new(Squeak);

        duck.fly();
        duck.quack();

        duck.quack_behavior = Box::new(MuteQuack);
        duck.quack();
        duck.swim();

        let duck_call = DuckCall::new();

        duck_call.quack();
        duck_call.fly();
    }
}

