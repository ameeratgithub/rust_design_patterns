#![allow(unused_variables, dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

trait Subject {
    fn add_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

impl WeatherData {
    fn new() -> Self {
        Self {
            observers: vec![],
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }
    fn set_measurements(&mut self, temperature: f32, humidity: f32, pressure: f32) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.pressure = pressure;
        self.measurements_changed();
    }
    fn measurements_changed(&self) {
        self.notify_observers();
    }
}

impl Subject for WeatherData {
    fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|o| !Rc::ptr_eq(o, &observer));
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer
                .borrow_mut()
                .update(self.temperature, self.humidity, self.pressure);
        }
    }
}

trait Observer {
    fn update(&mut self, temperature: f32, humidity: f32, pressure: f32);
}

trait DisplayElement {
    fn display(&self);
}

struct CurrentConditionsDisplay {
    temperature: f32,
    humidity: f32,
    weather_data: Rc<RefCell<WeatherData>>,
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f32, humidity: f32, pressure: f32) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.display();
    }
}

impl DisplayElement for CurrentConditionsDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {} F degrees and {}% humidity",
            self.temperature, self.humidity
        );
    }
}

impl CurrentConditionsDisplay {
    fn new(weather_data: Rc<RefCell<WeatherData>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(Self {
            temperature: 0.0,
            humidity: 0.0,
            weather_data: Rc::clone(&weather_data),
        }));

        weather_data.borrow_mut().add_observer(observer.clone());

        observer
    }
}

pub struct ObserverPattern;

use crate::traits::{DesignPattern, DesignPatternFactory};

impl DesignPatternFactory for ObserverPattern {
    fn new() -> Box<dyn DesignPattern> {
        Box::new(Self)
    }
}

impl DesignPattern for ObserverPattern {
    fn run(&self) {
        let weather_data = Rc::new(RefCell::new(WeatherData::new()));

        let current_conditions_display = CurrentConditionsDisplay::new(Rc::clone(&weather_data));

        let mut mut_weather_data = weather_data.borrow_mut();

        mut_weather_data.set_measurements(80.0, 65.0, 30.4);
        mut_weather_data.set_measurements(82.0, 70.0, 29.2);
        mut_weather_data.set_measurements(78.0, 90.0, 29.2);
    }
}
