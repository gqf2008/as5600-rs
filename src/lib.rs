#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal as hal;
use error::Error;
use hal::blocking::i2c;
use states::Initial;

pub mod constants;
pub mod error;
pub mod options;
mod states;
pub mod status;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct As5600<I2C, D, State> {
    i2c: I2C,
    address: u8,
    delay: D,
    state: PhantomData<State>,
}

impl<I2C, D, E> As5600<I2C, D, Initial>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
            state: PhantomData::<Initial>,
        }
    }

    pub fn release(self) -> (I2C, D) {
        (self.i2c, self.delay)
    }

    pub fn magnet_status(&mut self) -> Result<status::Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[0x0b], &mut buffer)?;
        status::Status::try_from(buffer).map_err(Error::Status)
    }
}
