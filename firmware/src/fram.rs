use embedded_hal::{digital::OutputPin, spi::SpiBus};

const WRITE_ENABLE: u8 = 0x06;
const READ_MEMORY: u8 = 0x03;
const WRITE_MEMORY: u8 = 0x02;

pub struct FM25L16B<CS: OutputPin, SPI: SpiBus> {
    cs: CS,
    spi: SPI,
}

impl<CS, SPI> FM25L16B<CS, SPI>
where
    CS: OutputPin,
    SPI: SpiBus,
{
    pub fn new(cs: CS, spi: SPI) -> Self {
        Self { cs, spi }
    }

    fn with_cs<F, R>(&mut self, f: F) -> Result<R, FM25L16BError>
    where
        F: FnOnce(&mut SPI) -> Result<R, FM25L16BError>,
    {
        self.cs.set_low().map_err(|_| FM25L16BError::ChipSelect)?;
        let result = f(&mut self.spi);
        self.cs.set_high().map_err(|_| FM25L16BError::ChipSelect)?;
        result
    }

    pub fn read_bytes(&mut self, address: u16, bytes: &mut [u8]) -> Result<(), FM25L16BError> {
        let [hi, lo] = address.to_le_bytes();
        let tx = [READ_MEMORY, hi, lo];

        self.with_cs(|spi| {
            spi.write(&tx).map_err(|_| FM25L16BError::Spi)?;
            spi.read(bytes).map_err(|_| FM25L16BError::Spi)
        })
    }

    pub fn write_bytes(&mut self, address: u16, bytes: &[u8]) -> Result<(), FM25L16BError> {
        let [hi, lo] = address.to_le_bytes();
        self.with_cs(|spi| spi.write(&[WRITE_ENABLE]).map_err(|_| FM25L16BError::Spi))
            .map_err(|_| FM25L16BError::Spi)?;
        self.with_cs(|spi| {
            spi.write(&[WRITE_MEMORY, hi, lo])
                .map_err(|_| FM25L16BError::Spi)?;
            spi.write(bytes).map_err(|_| FM25L16BError::Spi)
        })
    }

    pub fn read<T: NativeByteOrder>(&mut self, address: u16) -> Result<T, FM25L16BError> {
        let mut bytes = T::Bytes::default();
        self.read_bytes(address, bytes.as_mut())
            .map_err(|_| FM25L16BError::Spi)?;
        Ok(T::from_ne_bytes(bytes))
    }

    pub fn write<T: NativeByteOrder>(
        &mut self,
        address: u16,
        value: T,
    ) -> Result<(), FM25L16BError> {
        self.write_bytes(address, value.to_ne_bytes().as_ref())
            .map_err(|_| FM25L16BError::Spi)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum FM25L16BError {
    Spi,
    ChipSelect,
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
