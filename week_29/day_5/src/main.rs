use crate::ex1_smart_sensor::SmartSensor;

mod ex1_smart_sensor;

fn main() {
    let smart_sens = SmartSensor::new();
    smart_sens.add_log("First message");
}
