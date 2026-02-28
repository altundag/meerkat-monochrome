use embedded_hal::{delay::DelayNs, digital::OutputPin, i2c::I2c};
use rp235x_hal::{
    Timer,
    clocks::{GpioOutput0Clock, StoppableClock},
    timer::CopyableTimer0,
};

use mt9m001::MT9M001;

pub const HEIGHT: u16 = 1048;
pub const WIDTH: u16 = 1312;
pub const FREQUENCY: u32 = 6_500_000;

pub struct Sensor<I2C: I2c, SP: OutputPin, TP: OutputPin> {
    sensor_clock: GpioOutput0Clock,
    timer: Timer<CopyableTimer0>,
    standby: SP,
    trigger: TP,
    mt9m001: MT9M001<I2C>,
}

impl<I2C, SP, TP> Sensor<I2C, SP, TP>
where
    I2C: I2c,
    SP: OutputPin,
    TP: OutputPin,
{
    pub fn new(
        sensor_clock: GpioOutput0Clock,
        timer: Timer<CopyableTimer0>,
        i2c: I2C,
        standby: SP,
        trigger: TP,
    ) -> Self {
        Self {
            sensor_clock,
            timer,
            standby,
            trigger,
            mt9m001: MT9M001::new(i2c),
        }
    }

    fn wake(&mut self) -> Result<(), SensorError> {
        self.sensor_clock.enable();
        self.standby
            .set_low()
            .map_err(|_| SensorError::StandbyError)?;
        let output_control = mt9m001::OutputControl::DEFAULT.set_chip_enable(true);
        self.mt9m001
            .set_output_control(&output_control)
            .map_err(|_| SensorError::Spi)?;
        self.timer.delay_ms(1);
        Ok(())
    }

    fn sleep(&mut self) -> Result<(), SensorError> {
        let output_control = mt9m001::OutputControl::DEFAULT.set_chip_enable(false);
        self.mt9m001
            .set_output_control(&output_control)
            .map_err(|_| SensorError::Spi)?;
        self.timer.delay_ms(1);
        self.standby
            .set_high()
            .map_err(|_| SensorError::StandbyError)?;
        self.sensor_clock.disable();
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), SensorError> {
        self.wake()?;

        self.mt9m001.set_reset(1).map_err(|_| SensorError::Spi)?;
        self.mt9m001.set_reset(0).map_err(|_| SensorError::Spi)?;

        let read_options_1 = mt9m001::ReadOptions1::DEFAULT.set_snapshot_mode(true);
        self.mt9m001
            .set_read_options_1(&read_options_1)
            .map_err(|_| SensorError::Spi)?;

        let cal_control =
            mt9m001::CalCtrl::DEFAULT.set_manual_override_of_black_level_correction(true);
        self.mt9m001
            .set_cal_ctrl(&cal_control)
            .map_err(|_| SensorError::Spi)?;

        //let read_options_2 = mt9m001::ReadOptions2::DEFAULT.set_raw_data_output_mode(true);
        //self.mt9m001.set_read_options_2(&read_options_2)?;

        self.mt9m001
            .set_column_start(0)
            .map_err(|_| SensorError::Spi)?;
        self.mt9m001
            .set_column_size(WIDTH)
            .map_err(|_| SensorError::Spi)?;
        self.mt9m001
            .set_row_start(0)
            .map_err(|_| SensorError::Spi)?;
        self.mt9m001
            .set_row_size(HEIGHT)
            .map_err(|_| SensorError::Spi)?;
        self.mt9m001
            .set_horizontal_blanking(0)
            .map_err(|_| SensorError::Spi)?;
        self.mt9m001
            .set_vertical_blanking(0)
            .map_err(|_| SensorError::Spi)?;

        self.sleep()?;

        Ok(())
    }

    pub fn configure_and_capture<T, F: FnOnce() -> T>(
        &mut self,
        gain: f32,
        shutter: (u32, u32),
        transfer_fn: F,
    ) -> Result<T, SensorError> {
        self.wake()?;

        // Set gain
        let gain = if gain <= 4f32 {
            0x0008 + ((gain - 1f32) / 0.125f32) as u16
        } else if gain <= 8f32 {
            0x0051 + ((gain - 4f32) / 0.25f32) as u16
        } else {
            0x0061 + ((gain - 9f32) / 1.0) as u16
        };
        self.mt9m001
            .set_global_gain(gain)
            .map_err(|_| SensorError::Spi)?;

        // Set shutter speed
        let shutter_delay = self
            .mt9m001
            .get_shutter_delay()
            .map_err(|_| SensorError::Spi)?;
        let col_size = self
            .mt9m001
            .get_column_size()
            .map_err(|_| SensorError::Spi)?;
        let horizontal_blanking = self
            .mt9m001
            .get_horizontal_blanking()
            .map_err(|_| SensorError::Spi)?;
        let (numerator, denominator) = shutter;
        let integration_time_in_clock_periods = (numerator * FREQUENCY) / denominator;
        let shutter_width = (integration_time_in_clock_periods + 180 + 4 * shutter_delay as u32)
            / (col_size as u32 + horizontal_blanking as u32 + 226);
        self.mt9m001
            .set_shutter_width(shutter_width as u16)
            .map_err(|_| SensorError::Spi)?;

        // Trigger...
        self.trigger
            .set_high()
            .map_err(|_| SensorError::TriggerError)?;
        self.trigger
            .set_low()
            .map_err(|_| SensorError::TriggerError)?;

        // Capture here...
        let result = transfer_fn();

        self.sleep()?;

        Ok(result)
    }
}

#[derive(Debug)]
pub enum SensorError {
    Spi,
    TriggerError,
    StandbyError,
}
