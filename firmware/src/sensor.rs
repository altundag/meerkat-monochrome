use embassy_rp::{clocks::Gpout, gpio::Output, i2c, peripherals, pio};
use embassy_time::{Duration, block_for};

use mt9m001::MT9M001;

pub const HEIGHT: u16 = 1048;
pub const WIDTH: u16 = 1312;
pub const SENSOR_FREQUENCY: u32 = 5_800_000;

type ClockSource<'a> = Gpout<'a, peripherals::PIN_21>;
type I2C<'a> = i2c::I2c<'a, peripherals::I2C1, i2c::Blocking>;

pub struct Sensor<'a> {
    clock_source: ClockSource<'a>,
    mt9m001: MT9M001<I2C<'a>>,
    standby: Output<'a>,
}

impl<'a> Sensor<'a> {
    pub fn new(clock_source: ClockSource<'a>, i2c: I2C<'a>, standby: Output<'a>) -> Self {
        Self {
            clock_source,
            mt9m001: MT9M001::new(i2c),
            standby,
        }
    }

    fn with_sensor_awake<F, R>(&mut self, f: F) -> Result<R, i2c::Error>
    where
        F: FnOnce(&mut MT9M001<I2C>) -> Result<R, i2c::Error>,
    {
        self.clock_source.enable();
        self.standby.set_low();
        block_for(Duration::from_nanos(500));
        let result = f(&mut self.mt9m001);
        self.standby.set_high();
        self.clock_source.disable();
        result
    }

    pub fn is_known_sensor(&mut self) -> Result<bool, i2c::Error> {
        Ok(self.with_sensor_awake(|sensor| sensor.get_chip_version())? == 0x8431)
    }

    pub fn init(&mut self) -> Result<(), i2c::Error> {
        self.with_sensor_awake(|sensor| {
            sensor.set_reset(1)?;
            sensor.set_reset(0)?;

            let read_options_1 = mt9m001::ReadOptions1::DEFAULT.set_snapshot_mode(true);
            sensor.set_read_options_1(&read_options_1)?;

            //let read_options_2 = mt9m001::ReadOptions2::DEFAULT.set_raw_data_output_mode(true);
            //sensor.set_read_options_2(&read_options_2)?;

            sensor.set_column_start(0)?;
            sensor.set_column_size(WIDTH)?;
            sensor.set_row_start(0)?;
            sensor.set_row_size(HEIGHT)?;
            sensor.set_horizontal_blanking(0)?;
            sensor.set_vertical_blanking(0)?;

            // don't wait for the bad frame that the configuration block above caused
            sensor.set_frame_restart(1)?;

            Ok(())
        })
    }

    pub fn shutter_speed(&mut self, numerator: u32, denominator: u32) -> Result<(), i2c::Error> {
        let integration_time_in_clock_periods = (numerator * SENSOR_FREQUENCY) / denominator;

        self.with_sensor_awake(|sensor| {
            let shutter_delay = sensor.get_shutter_delay()?;
            let col_size = sensor.get_column_size()?;
            let horizontal_blanking = sensor.get_horizontal_blanking()?;

            let shutter_width =
                (integration_time_in_clock_periods + 180 + 4 * shutter_delay as u32)
                    / (col_size as u32 + horizontal_blanking as u32 + 226);

            sensor.set_shutter_width(shutter_width as u16)?;

            Ok(())
        })
    }

    pub fn gain(&mut self, value: f32) -> Result<(), i2c::Error> {
        let value = value.clamp(1f32, 15f32);

        let register_value = if value <= 4f32 {
            0x0008 + ((value - 1f32) / 0.125f32) as u16
        } else if value <= 8f32 {
            0x0051 + ((value - 4f32) / 0.25f32) as u16
        } else {
            0x0061 + ((value - 9f32) / 1.0) as u16
        };

        self.with_sensor_awake(|sensor| sensor.set_global_gain(register_value))
    }

    pub async fn capture(
        &mut self,
        sm: &mut pio::StateMachine<'_, peripherals::PIO0, 0>,
        dma: embassy_rp::Peri<'_, peripherals::DMA_CH1>,
        image: &mut [u32],
    ) -> Result<(), i2c::Error> {
        self.clock_source.enable();
        self.standby.set_low();
        self.mt9m001.set_frame_restart(1)?;
        sm.restart();
        sm.set_enable(true);
        let rx = sm.rx();
        let transfer = rx.dma_pull(dma, image, false);
        transfer.await;
        sm.set_enable(false);
        self.standby.set_high();
        self.clock_source.disable();

        Ok(())
    }
}
