use embassy_rp::{gpio::Output, peripherals::SPI0, spi};

const WRITE_ENABLE: u8 = 0x06;
const READ_MEMORY: u8 = 0x03;
const WRITE_MEMORY: u8 = 0x02;

pub struct FM25L16B<'a> {
    cs: Output<'a>,
    spi: spi::Spi<'a, SPI0, spi::Blocking>,
}

impl<'a> FM25L16B<'a> {
    pub fn new(mut cs: Output<'a>, spi: spi::Spi<'a, SPI0, spi::Blocking>) -> Self {
        cs.set_high();
        Self { cs, spi }
    }

    fn with_cs<F, R>(&mut self, f: F) -> Result<R, spi::Error>
    where
        F: FnOnce(&mut spi::Spi<'a, SPI0, spi::Blocking>) -> Result<R, spi::Error>,
    {
        self.cs.set_low();
        let result = f(&mut self.spi);
        self.cs.set_high();
        result
    }

    pub fn write_enable(&mut self) -> Result<(), spi::Error> {
        self.with_cs(|spi| spi.blocking_write(&[WRITE_ENABLE]))
    }

    pub fn read(&mut self, address: u16) -> Result<u8, spi::Error> {
        let [hi, lo] = address.to_le_bytes();
        let tx = [READ_MEMORY, hi, lo, 0x00];
        let mut rx = [0u8; 4];

        self.with_cs(|spi| {
            spi.blocking_transfer(&mut rx, &tx)?;
            Ok(rx[3])
        })
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), spi::Error> {
        let [hi, lo] = address.to_le_bytes();

        self.with_cs(|spi| spi.blocking_write(&[WRITE_MEMORY, hi, lo, value]))
    }

    pub fn read_u16(&mut self, address: u16) -> Result<u16, spi::Error> {
        Ok(u16::from_le_bytes([
            self.read(address)?,
            self.read(address + 1)?,
        ]))
    }

    pub fn write_u16(&mut self, address: u16, value: u16) -> Result<(), spi::Error> {
        let [hi, lo] = value.to_le_bytes();

        self.write_enable()?;
        self.write(address, hi)?;
        self.write_enable()?;
        self.write(address + 1, lo)?;

        Ok(())
    }
}
