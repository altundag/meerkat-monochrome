use crate::tiff::write_single_directory_monochrome_tiff;
use embedded_hal::spi::SpiDevice;
use embedded_sdmmc::{Error, Mode::ReadWriteCreate, SdCard, SdCardError, VolumeManager};
use rp235x_hal::{Timer, timer::CopyableTimer0};

pub struct Sdmmc<'a, SPI>
where
    SPI: SpiDevice,
{
    volume_manager:
        VolumeManager<SdCard<SPI, &'a mut Timer<CopyableTimer0>>, DummyTimesource, 4, 4, 1>,
}

impl<'a, SPI> Sdmmc<'a, SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI, timer: &'a mut Timer<CopyableTimer0>) -> Self {
        let sdcard = SdCard::new(spi, timer);
        let volume_manager = VolumeManager::new(sdcard, DummyTimesource());
        Self { volume_manager }
    }

    pub fn write_image(
        &mut self,
        image_num: u16,
        width: u16,
        height: u16,
        bpp: u16,
        image: &mut [u8],
    ) -> Result<(), Error<SdCardError>> {
        let volume = self
            .volume_manager
            .open_volume(embedded_sdmmc::VolumeIdx(0))?;
        let root_dir = volume.open_root_dir()?;

        // file name
        let mut buf = *b"IM00000.TIF";
        let mut n = image_num;
        for b in buf[2..7].iter_mut().rev() {
            *b += (n % 10) as u8;
            n /= 10;
        }
        let file_name = unsafe { str::from_utf8_unchecked(&buf) };

        let file = root_dir.open_file_in_dir(file_name, ReadWriteCreate)?;

        write_single_directory_monochrome_tiff(
            |bytes| file.write(bytes),
            width,
            height,
            bpp,
            image,
        )?;

        Ok(())
    }
}

struct DummyTimesource();

impl embedded_sdmmc::TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
