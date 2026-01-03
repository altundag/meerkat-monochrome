use embassy_embedded_hal::SetConfig;
use embassy_rp::{gpio::Output, peripherals::SPI1, spi};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{Error, SdCard, SdCardError};

type VolumeManager<'a> = embedded_sdmmc::VolumeManager<
    SdCard<ExclusiveDevice<spi::Spi<'a, SPI1, spi::Blocking>, Output<'a>, Delay>, Delay>,
    DummyTimesource,
>;

pub struct Sdmmc<'a> {
    volume_manager: VolumeManager<'a>,
}

impl<'a> Sdmmc<'a> {
    pub fn new(spi: spi::Spi<'a, SPI1, spi::Blocking>, cs: Output<'a>) -> Self {
        let spi_device = ExclusiveDevice::new(spi, cs, Delay);
        let sd_card = SdCard::new(spi_device, Delay);
        let mut config = spi::Config::default();
        config.frequency = 24_000_000;
        sd_card
            .spi(|dev| SetConfig::set_config(dev.bus_mut(), &config))
            .ok();
        let volume_manager = embedded_sdmmc::VolumeManager::new(sd_card, DummyTimesource());
        Self { volume_manager }
    }

    pub fn write_image(&mut self, image_num: u16, image: &[u8]) -> Result<(), Error<SdCardError>> {
        let volume = self
            .volume_manager
            .open_volume(embedded_sdmmc::VolumeIdx(0))?;
        let root_dir = volume.open_root_dir()?;

        // file name
        let mut buf = *b"IM00000.RAW";
        let mut n = image_num;
        for b in buf[2..7].iter_mut().rev() {
            *b += (n % 10) as u8;
            n /= 10;
        }
        let file_name = unsafe { str::from_utf8_unchecked(&buf) };

        let file = root_dir
            .open_file_in_dir(file_name, embedded_sdmmc::Mode::ReadWriteCreateOrTruncate)?;
        file.write(image)?;
        file.flush()?;
        file.close()?;

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
