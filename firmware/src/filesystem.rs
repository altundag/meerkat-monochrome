use embedded_hal::spi::SpiDevice;
use embedded_sdmmc::{Mode, VolumeIdx, VolumeManager, sdcard::SdCard};
use rp235x_hal::{Timer, timer::CopyableTimer0};

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

pub struct FileSystem<'a, SPI>
where
    SPI: SpiDevice,
{
    volume_mgr: VolumeManager<SdCard<SPI, &'a mut Timer<CopyableTimer0>>, DummyTimesource, 4, 4, 1>,
}

impl<'a, SPI> FileSystem<'a, SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI, timer: &'a mut Timer<CopyableTimer0>) -> Self {
        let sdcard = SdCard::new(spi, timer);
        let volume_mgr = VolumeManager::new(sdcard, DummyTimesource());
        Self { volume_mgr }
    }

    pub fn write_new_image(&mut self, bytes: &[u8]) {
        let volume = self.volume_mgr.open_volume(VolumeIdx(0)).unwrap();
        let root = volume.open_root_dir().unwrap();
        let file = root
            .open_file_in_dir("IMG_000.RAW", Mode::ReadWriteCreateOrTruncate)
            .unwrap();
        file.write(bytes).unwrap();
    }
}
