use embassy_rp::{clocks::Gpout, gpio::Output, i2c, peripherals, pio};
use embassy_time::{Duration, block_for};

use mt9m001::MT9M001;

type ClockSource<'a> = Gpout<'a, peripherals::PIN_21>;
type I2C<'a> = i2c::I2c<'a, peripherals::I2C1, i2c::Blocking>;

pub struct Sensor<'a> {
    clock_source: ClockSource<'a>,
    mt9m001: MT9M001<I2C<'a>>,
    standby: Output<'a>,
    trigger: Output<'a>,
    shutter: (u32, u32),
    gain: f32,
}

impl<'a> Sensor<'a> {
    pub const HEIGHT: u16 = 1048;
    pub const WIDTH: u16 = 1312;
    pub const FREQUENCY: u32 = 5_800_000;

    pub fn new(
        clock_source: ClockSource<'a>,
        i2c: I2C<'a>,
        standby: Output<'a>,
        trigger: Output<'a>,
    ) -> Self {
        Self {
            clock_source,
            mt9m001: MT9M001::new(i2c),
            standby,
            trigger,
            shutter: (1, 10),
            gain: 1f32,
        }
    }

    fn wake(&mut self) -> Result<(), i2c::Error> {
        self.clock_source.enable();
        self.standby.set_low();
        let output_control = mt9m001::OutputControl::DEFAULT.set_chip_enable(true);
        self.mt9m001.set_output_control(&output_control)?;
        block_for(Duration::from_millis(1));
        Ok(())
    }

    fn sleep(&mut self) -> Result<(), i2c::Error> {
        let output_control = mt9m001::OutputControl::DEFAULT.set_chip_enable(false);
        self.mt9m001.set_output_control(&output_control)?;
        block_for(Duration::from_millis(1));
        self.standby.set_high();
        self.clock_source.disable();
        Ok(())
    }

    pub fn is_known_sensor(&mut self) -> Result<bool, i2c::Error> {
        self.wake()?;
        let result = self.mt9m001.get_chip_version()?;
        self.sleep()?;
        Ok(0x8431 == result)
    }

    pub fn init(&mut self) -> Result<(), i2c::Error> {
        self.wake()?;

        self.mt9m001.set_reset(1)?;
        self.mt9m001.set_reset(0)?;

        let read_options_1 = mt9m001::ReadOptions1::DEFAULT.set_snapshot_mode(true);
        self.mt9m001.set_read_options_1(&read_options_1)?;

        let cal_control =
            mt9m001::CalCtrl::DEFAULT.set_manual_override_of_black_level_correction(true);
        self.mt9m001.set_cal_ctrl(&cal_control)?;

        //let read_options_2 = mt9m001::ReadOptions2::DEFAULT.set_raw_data_output_mode(true);
        //self.mt9m001.set_read_options_2(&read_options_2)?;

        self.mt9m001.set_column_start(0)?;
        self.mt9m001.set_column_size(Self::WIDTH)?;
        self.mt9m001.set_row_start(0)?;
        self.mt9m001.set_row_size(Self::HEIGHT)?;
        self.mt9m001.set_horizontal_blanking(0)?;
        self.mt9m001.set_vertical_blanking(0)?;

        self.sleep()?;

        Ok(())
    }

    pub fn set_shutter_speed(&mut self, numerator: u32, denominator: u32) {
        self.shutter = (numerator, denominator)
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub async fn capture(
        &mut self,
        sm: &mut pio::StateMachine<'_, peripherals::PIO0, 0>,
        dma: embassy_rp::Peri<'_, peripherals::DMA_CH1>,
        image: &mut [u32],
    ) -> Result<(), i2c::Error> {
        self.wake()?;

        sm.set_enable(true);
        sm.clear_fifos();
        sm.restart();

        // Set gain
        let gain = if self.gain <= 4f32 {
            0x0008 + ((self.gain - 1f32) / 0.125f32) as u16
        } else if self.gain <= 8f32 {
            0x0051 + ((self.gain - 4f32) / 0.25f32) as u16
        } else {
            0x0061 + ((self.gain - 9f32) / 1.0) as u16
        };
        self.mt9m001.set_global_gain(gain)?;

        // Correction...
        self.mt9m001.set_even_row_odd_column_analog_offset(2)?;

        // Set shutter speed
        let shutter_delay = self.mt9m001.get_shutter_delay()?;
        let col_size = self.mt9m001.get_column_size()?;
        let horizontal_blanking = self.mt9m001.get_horizontal_blanking()?;
        let (numerator, denominator) = self.shutter;
        let integration_time_in_clock_periods = (numerator * Self::FREQUENCY) / denominator;
        let shutter_width = (integration_time_in_clock_periods + 180 + 4 * shutter_delay as u32)
            / (col_size as u32 + horizontal_blanking as u32 + 226);
        self.mt9m001.set_shutter_width(shutter_width as u16)?;

        let rx = sm.rx();
        let transfer = rx.dma_pull(dma, image, false);
        self.trigger.set_high();
        self.trigger.set_low();
        transfer.await;

        sm.set_enable(false);

        self.sleep()?;

        Ok(())
    }

    pub fn width(&mut self) -> Result<u16, i2c::Error> {
        self.wake()?;

        let result = self.mt9m001.get_column_size()?;

        self.sleep()?;

        Ok(result)
    }

    pub fn height(&mut self) -> Result<u16, i2c::Error> {
        self.wake()?;

        let result = self.mt9m001.get_row_size()?;

        self.sleep()?;

        Ok(result)
    }
}
