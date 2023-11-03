#![allow(unused_variables, dead_code)]

use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

trait Subject {
    fn add_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: RefCell<f32>,
    humidity: RefCell<f32>,
    pressure: RefCell<f32>,
}

impl WeatherData {
    fn new() -> Self {
        Self {
            observers: vec![],
            temperature: 0.0.into(),
            humidity: 0.0.into(),
            pressure: 0.0.into(),
        }
    }
    fn get_temprature(&self) -> f32 {
        *self.temperature.borrow()
    }
    fn get_humidity(&self) -> f32 {
        *self.humidity.borrow()
    }
    fn get_pressure(&self) -> f32 {
        *self.pressure.borrow()
    }
    fn set_measurements(&self, temperature: f32, humidity: f32, pressure: f32) {
        *self.temperature.borrow_mut() = temperature;
        *self.humidity.borrow_mut() = humidity;
        *self.pressure.borrow_mut() = pressure;
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
            observer.borrow_mut().update();
        }
    }
}

trait Observer {
    fn update(&mut self);
}

trait DisplayElement {
    fn display(&self);
}

struct CurrentConditionsDisplay {
    temperature: f32,
    humidity: f32,
    weather_data: Weak<RefCell<WeatherData>>,
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self) {
        if let Some(weather_data) = self.weather_data.upgrade() {
            let weather_data = weather_data.borrow();
            self.temperature = weather_data.get_temprature();
            self.humidity = weather_data.get_humidity();
        }

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
    fn new(weather_data: Weak<RefCell<WeatherData>>) -> Rc<RefCell<Self>> {
        let weather_data_clone = weather_data.clone();

        let observer = Rc::new(RefCell::new(Self {
            temperature: 0.0,
            humidity: 0.0,
            weather_data,
        }));

        if let Some(w) = weather_data_clone.upgrade() {
            w.borrow_mut().add_observer(observer.clone());
        }

        observer
    }
}

struct HeatIndexDisplay {
    temperature: RefCell<f32>,
    humidity: RefCell<f32>,
    weather_data: Rc<RefCell<WeatherData>>,
}

impl DisplayElement for HeatIndexDisplay {
    fn display(&self) {
        println!(
            "Heat Index is {}",
            self.compute_head_index(*self.temperature.borrow(), *self.humidity.borrow())
        );
    }
}

impl HeatIndexDisplay {
    fn new(weather_data: Rc<RefCell<WeatherData>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(Self {
            temperature: 0.0.into(),
            humidity: 0.0.into(),
            weather_data: Rc::clone(&weather_data),
        }));

        weather_data.borrow_mut().add_observer(observer.clone());

        observer
    }

    fn compute_head_index(&self, t: f32, rh: f32) -> f32 {
        let index = 16.923 + (0.185212 * t) + (5.37941 * rh) - (0.100254 * t * rh)
            + (0.00941695 * (t * t))
            + (0.00728898 * (rh * rh))
            + (0.000345372 * (t * t * rh))
            - (0.000814971 * (t * rh * rh))
            + (0.0000102102 * (t * t * rh * rh))
            - (0.000038646 * (t * t * t))
            + (0.0000291583 * (rh * rh * rh))
            + (0.00000142721 * (t * t * t * rh))
            + (0.000000197483 * (t * rh * rh * rh))
            - (0.0000000218429 * (t * t * t * rh * rh))
            + 0.000000000843296 * (t * t * rh * rh * rh)
            - 0.0000000000481975 * (t * t * t * rh * rh * rh);

        index
    }
}

impl Observer for HeatIndexDisplay {
    fn update(&mut self) {
        self.temperature = self.weather_data.borrow().get_temprature().into();
        self.humidity = self.weather_data.borrow().get_humidity().into();
        self.display();
    }
}

struct ForecastDisplay {
    current_pressure: RefCell<f32>,
    last_pressure: RefCell<f32>,
    weather_data: Rc<RefCell<WeatherData>>,
}

impl DisplayElement for ForecastDisplay {
    fn display(&self) {
        let last_pressure = *self.last_pressure.borrow();
        let current_pressure = *self.current_pressure.borrow();

        if last_pressure < current_pressure {
            println!("Forecast: Improving weather on the way!");
        } else if last_pressure > current_pressure {
            println!("Forecast: Watch out for cooler, rainy weather");
        }else{
            println!("Forecast: More of the same");
        }
    }
}

impl ForecastDisplay {
    fn new(weather_data: Rc<RefCell<WeatherData>>) -> Rc<RefCell<Self>> {
        let observer = Rc::new(RefCell::new(Self {
            last_pressure: 0.0.into(),
            current_pressure: 0.0.into(),
            weather_data: Rc::clone(&weather_data),
        }));

        weather_data.borrow_mut().add_observer(observer.clone());

        observer
    }
}

impl Observer for ForecastDisplay {
    fn update(&mut self) {
        *self.last_pressure.borrow_mut() = *self.current_pressure.borrow();
        *self.current_pressure.borrow_mut() = self.weather_data.borrow().get_pressure();
        self.display();
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

        CurrentConditionsDisplay::new(Rc::downgrade(&weather_data));
        HeatIndexDisplay::new(Rc::clone(&weather_data));
        ForecastDisplay::new(Rc::clone(&weather_data));

        weather_data.borrow().set_measurements(80.0, 65.0, 30.4);
        println!("\n");

        weather_data.borrow().set_measurements(82.0, 70.0, 29.2);
        println!("\n");

        weather_data.borrow().set_measurements(78.0, 90.0, 29.2);
        println!("\n");
    }
}
