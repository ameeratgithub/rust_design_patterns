pub trait DesignPattern {
    fn run(&self);
}

pub trait DesignPatternFactory {
    fn new() -> Box<dyn DesignPattern>;
}
