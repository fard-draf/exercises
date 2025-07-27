use embedded_hal::{
    digital::Error,
    i2c::{I2c, Operation, SevenBitAddress},
    *,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BusError {
    TxFailed,
    RxFailed,
}

impl i2c::Error for BusError {
    fn kind(&self) -> i2c::ErrorKind {
        match *self {
            BusError::TxFailed => i2c::ErrorKind::Bus,
            BusError::RxFailed => i2c::ErrorKind::Bus,
        }
    }
}

struct MockCanBus {
    buffer: [u8; 7],
}

impl embedded_hal::i2c::ErrorType for MockCanBus {
    type Error = BusError;
}

impl I2c<SevenBitAddress> for MockCanBus {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(println!("address: {:?}", address))
    }
}

pub struct SensorDriver<'a, B: i2c::I2c> {
    i2c_mod: &'a mut B,
}

impl<'a, B: i2c::I2c> SensorDriver<'a, B> {
    pub fn new(i2c: &'a mut B) -> Self {
        Self { i2c_mod: i2c }
    }

    pub fn read_transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation],
    ) -> Result<(), <B as embedded_hal::i2c::ErrorType>::Error> {
        self.i2c_mod.transaction(address, operations)
    }
}

fn main() -> Result<(), BusError> {
    let mut mock = MockCanBus { buffer: [5u8; 7] };
    let mut sensor = SensorDriver::new(&mut mock);
    let items = &mut [0x5, 0x19];
    let address = 0x3A;
    sensor.read_transaction(address, &mut [Operation::Read(items)]);
    Ok(())
}
