#![no_std]

//
// Driver for EverSet(tm) WWVB Receiver – ES100
//
// Description from the data sheet:
//
// The ES100 is a fully self-contained phase modulation time code receiver that receives and decodes the 60 kHz time
// signal from the National Institute of Standards and Technology’s WWVB transmitter located in Ft. Collins, Colorado, USA.
// It contains a digital correlation receiver to extract the time code information from the received signal. It also has a
// simple serial interface to transfer the date, time, and DST information to a host microcontroller. The ES100 is compatible
// with existing WWVB receive antennas and offers significantly improved performance in low signal-to-noise and low
// signal-to-interference scenarios when compared to amplitude modulation receivers.
//
// https://everset.tech/wp-content/uploads/2014/11/ES100DataSheetver0p97.pdf
//
//

use embedded_hal::i2c::{Error, I2c};

mod device;

pub struct ES100AtomicClock<I2C> {
    i2c: I2C,
    device_addr: u8,
}

impl<I2C: I2c> ES100AtomicClock<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self::new_with_addr(i2c, device::ES100_I2C_DEFAULT_ADDRESS)
    }

    pub fn new_with_addr(i2c: I2C, device_addr: u8) -> Self {
        Self { i2c, device_addr }
    }

    pub fn start_rx(
        &mut self,
        ant1_en: bool,
        ant2_en: bool,
        tracking_mode: bool,
        ant2_first: bool,
    ) -> Result<u8, I2C::Error> {
        Ok(42)
    }

    pub fn get_device_id(&mut self) -> Result<u8, I2C::Error> {
        let mut dev_id = [0];
        self.i2c.write_read(
            self.device_addr,
            &[device::ES100_DEVICE_ID_REG],
            &mut dev_id,
        )?;
        Ok(dev_id[0])
    }

    pub fn get_interrupt_status(&mut self) -> Result<u8, I2C::Error> {
        Ok(42)
    }

    pub fn get_status(&mut self) -> Result<u8, I2C::Error> {
        Ok(42)
    }

    pub fn get_date_time(&mut self) -> Result<u8, I2C::Error> {
        Ok(42)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
