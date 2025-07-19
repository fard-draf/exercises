//======================================================================================COMPIL ERROR
// On souhaite, a travers le trait Observer, mettre a jour des donnees via une &self. Il est impossible de 
// modifier une &self a travers l'apporche classique. La &self est par nature immuable. Erreur de compilation pour chaque implementation
// cherchant a mettre a jour ses datas.
// pub trait Observer {
//     fn notify(&self, temp: f32, humidity: f32);
// }

// pub struct TemperatureDisplay {
//     // État interne qui doit être mis à jour
//     last_displayed_temp: f32,
// }

// impl Observer for TemperatureDisplay {
//     fn notify(&self, temp: f32, humidity: f32) {
//         self.last_displayed_temp = temp; // error
//     }
// }

// pub struct DataLogger {
//     // État interne qui doit être mis à jour
//     log: Vec<String>,
// }

// impl Observer for DataLogger {
//     fn notify(&self, temp: f32, humidity: f32) {
//         let data = format!("temp: {}, humidity: {}", temp, humidity);
//         self.log.push(data); // error
//     }
// }

// pub struct WeatherStation<'a> {
//     // La station détient une liste de références vers des observers.
//     observers: Vec<&'a dyn Observer>,
// }

// impl<'a> WeatherStation<'a> {
//     pub fn new() -> Self {
//         WeatherStation { observers: Vec::new() }
//     }

//     pub fn add_observer(&mut self, observer: &'a dyn Observer) {
//         self.observers.push(observer);
//     }

//     // Simule une nouvelle mesure et notifie tout le monde.
//     pub fn set_measurement(&self, temp: f32, humidity: f32) {
//         for observer in &self.observers {
//             observer.notify(temp, humidity);
//         }
//     }
// }

//======================================================================================


use std::{cell::RefCell, fmt::Debug};

pub trait Observer: Debug
{
    fn notify(&self, temp: f32, humidity: f32);
}

#[derive(Debug, Default)]
pub struct TemperatureDisplay {
    // État interne qui doit être mis à jour
    last_displayed_temp: RefCell<f32>,
}


impl Observer for TemperatureDisplay {
    fn notify(&self, temp: f32, humidity: f32) {
        *self.last_displayed_temp.borrow_mut() = temp
    }
}

#[derive(Debug, Default)]
pub struct DataLogger {
    // État interne qui doit être mis à jour
    log: RefCell<Vec<String>>,
}

impl Observer for DataLogger {
    fn notify(&self, temp: f32, humidity: f32) {
        let data = format!("temp: {}, humidity: {}", temp, humidity);
        self.log.borrow_mut().push(data);
    }
}

#[derive(Debug)]
pub struct WeatherStation<'a> {
    // La station détient une liste de références vers des observers.
    observers: Vec<&'a dyn Observer>,
}

impl<'a> WeatherStation<'a> {
    pub fn new() -> Self {
        WeatherStation { observers: Vec::new() }
    }

    pub fn add_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }

    // Simule une nouvelle mesure et notifie tout le monde.
    pub fn set_measurement(&self, temp: f32, humidity: f32) {
        for observer in &self.observers {
            observer.notify(temp, humidity);
        }
    }
}