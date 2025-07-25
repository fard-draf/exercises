pub trait Bus {
    fn send(&self, data: &[u8]) -> Result<(), BusError> {
        Ok(println!("Data sended: {:?}", data))
    }

    fn receive<'a>(&'a mut self) -> Result<&'a [u8], BusError>;
}

#[derive(Debug)]
enum BusError {
    TxFailed,
    RxFailed,
}

pub struct MockCanBus {
    pub buffer: [u8; 8],
}

impl Bus for MockCanBus {
    fn receive<'a>(&'a mut self) -> Result<&'a [u8], BusError> {
        Ok(&self.buffer)
    }
}

pub struct MockN2kBus {
    pub can_id: [u8; 29],
}

impl Bus for MockN2kBus {
    fn receive<'a>(&'a mut self) -> Result<&'a [u8], BusError> {
        Ok(&self.can_id)
    }
}
pub struct SensorDriver<'a, B: Bus> {
    bus: &'a mut B,
}

impl<'a, B: Bus> SensorDriver<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        Self { bus }
    }

    pub fn read_sensor_id(&mut self) -> Result<&[u8], BusError> {
        self.bus.receive()
    }
}

fn main() {
    let mut mock = MockCanBus { buffer: [5u8; 8] };
    let data = [6u8; 29];
    let mut sensor = SensorDriver { bus: &mut mock };

    let mut can = MockN2kBus { can_id: data };
    let mut sensor2 = SensorDriver { bus: &mut can };
    println!("{:?}", sensor2.read_sensor_id());
    println!("{:?}", sensor.read_sensor_id());
}
