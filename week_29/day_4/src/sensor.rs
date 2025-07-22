use std::cell::Cell;

pub struct Sensor {
    pub read: Cell<u32>
}

pub fn add_read(c: &Sensor) {
    let actual_value = c.read.get();
    c.read.set(actual_value + 2);

}


