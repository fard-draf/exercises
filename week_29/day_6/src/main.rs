use crate::observer::{DataLogger, TemperatureDisplay, WeatherStation};

mod bus;
mod ledger;
mod observer;

fn main() {
    let mut ws_1 = WeatherStation::new();
    let temp_display = TemperatureDisplay::default();
    let data_log = DataLogger::default();
    ws_1.add_observer(&temp_display);
    ws_1.add_observer(&data_log);

    ws_1.set_measurement(15.5, 65.0);

    println!("data collected: {:#?}", ws_1)
}
