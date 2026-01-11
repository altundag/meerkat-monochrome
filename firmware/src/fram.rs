use embassy_rp::{gpio::Output, peripherals::SPI0, spi};

pub struct FM25L16B<'a> {
    cs: Output<'a>,
    spi: spi::Spi<'a, SPI0, spi::Blocking>,
}

impl<'a> FM25L16B<'a> {
    const WRITE_ENABLE: u8 = 0x06;
    const READ_MEMORY: u8 = 0x03;
    const WRITE_MEMORY: u8 = 0x02;

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

    pub fn read_bytes(&mut self, address: u16, bytes: &mut [u8]) -> Result<(), spi::Error> {
        let [hi, lo] = address.to_le_bytes();
        let tx = [Self::READ_MEMORY, hi, lo];

        self.with_cs(|spi| {
            spi.blocking_write(&tx)?;
            spi.blocking_read(bytes)
        })
    }

    pub fn write_bytes(&mut self, address: u16, bytes: &[u8]) -> Result<(), spi::Error> {
        let [hi, lo] = address.to_le_bytes();
        self.with_cs(|spi| spi.blocking_write(&[Self::WRITE_ENABLE]))?;
        self.with_cs(|spi| {
            spi.blocking_write(&[Self::WRITE_MEMORY, hi, lo])?;
            spi.blocking_write(bytes)
        })
    }

    pub fn read<T: NativeByteOrder>(&mut self, address: u16) -> Result<T, spi::Error> {
        let mut bytes = T::Bytes::default();
        self.read_bytes(address, bytes.as_mut())?;
        Ok(T::from_ne_bytes(bytes))
    }

    pub fn write<T: NativeByteOrder>(&mut self, address: u16, value: T) -> Result<(), spi::Error> {
        self.write_bytes(address, value.to_ne_bytes().as_ref())?;
        Ok(())
    }
}

pub trait NativeByteOrder {
    type Bytes: Default + AsMut<[u8]> + AsRef<[u8]>;

    fn from_ne_bytes(bytes: Self::Bytes) -> Self;
    fn to_ne_bytes(self) -> Self::Bytes;
}

macro_rules! impl_native_byte_order {
    ($($t:ty),+) => {
        $(
        impl NativeByteOrder for $t {
            type Bytes = [u8; size_of::<$t>()];

            fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_ne_bytes(bytes)
            }

            fn to_ne_bytes(self) -> Self::Bytes {
                <$t>::to_ne_bytes(self)
            }
        }
        )*

    };
}

impl_native_byte_order!(i8, u8, i16, u16, i32, u32, i64, u64);
