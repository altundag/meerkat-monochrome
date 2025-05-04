#![no_std]

use embedded_hal::i2c::I2c;

///Chip version, (1000 0100 0001 0001)
pub const CHIP_VERSION: u8 = 0x00;

///Row start, (0000 0ddd dddd dddd)
pub const ROW_START: u8 = 0x01;

///Column start, (0000 0ddd dddd dddd)
pub const COLUMN_START: u8 = 0x02;

///Row size (window height), (0000 0ddd dddd dddd)
pub const ROW_SIZE: u8 = 0x03;

///Col size (window width), (0000 0ddd dddd dddd)
pub const COLUMN_SIZE: u8 = 0x04;

///Horizontal blanking, (0000 0ddd dddd dddd)
pub const HORIZONTAL_BLANKING: u8 = 0x05;

///Vertical blanking, (0000 0ddd dddd dddd)
pub const VERTICAL_BLANKING: u8 = 0x06;

///Output control, (0000 0000 0d00 00dd)
pub const OUTPUT_CONTROL: u8 = 0x07;

///Shutter width, (00dd dddd dddd dddd)
pub const SHUTTER_WIDTH: u8 = 0x09;

///Restart, (0000 0000 0000 000d)
pub const RESTART: u8 = 0x0B;

///Shutter delay, (0000 0ddd dddd dddd)
pub const SHUTTER_DELAY: u8 = 0x0C;

///Reset, (0000 0000 0000 000d)
pub const RESET: u8 = 0x0D;

///Read options 1, (1000 dddd 00dd dd00)
pub const READ_OPTIONS_1: u8 = 0x1E;

///Read options 2, (dd01 0dd1 d00d d10d)
pub const READ_OPTIONS_2: u8 = 0x20;

///Even row, even column, (0000 0000 0ddd dddd)
pub const GAIN_EVEN_ROW_EVEN_COLUMN: u8 = 0x2B;

///Odd row, even column, (0000 0000 0ddd dddd)
pub const GAIN_ODD_ROW_EVEN_COLUMN: u8 = 0x2C;

///Even row, odd column, (0000 0000 0ddd dddd)
pub const GAIN_EVEN_ROW_ODD_COLUMN: u8 = 0x2D;

///Odd row, odd column, (0000 0000 0ddd dddd)
pub const GAIN_ODD_ROW_ODD_COLUMN: u8 = 0x2E;

///Global gain, (0000 0000 0ddd dddd)
pub const GLOBAL_GAIN: u8 = 0x35;

///Cal threshold, (dddd dddd d0dd dddd)
pub const CAL_THRESHOLD: u8 = 0x5F;

///Even row, even column, (0000 000d dddd dddd)
pub const ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN: u8 = 0x60;

///Odd row, odd column, (0000 000d dddd dddd)
pub const ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN: u8 = 0x61;

///Cal ctrl, (d00d d100 1001 1ddd)
pub const CAL_CTRL: u8 = 0x62;

///Even row, odd column, (0000 000d dddd dddd)
pub const ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN: u8 = 0x63;

///Odd row, even column, (0000 000d dddd dddd)
pub const ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN: u8 = 0x64;

///Chip enable, (0000 0000 0000 00dd)
pub const CHIP_ENABLE: u8 = 0xF1;

const ADDRESS: u8 = 0x5D;

#[derive(Debug)]
pub enum Error {
    I2CRead,
    I2CWrite,
    Value,
}

pub struct MT9M001<RW>
where
    RW: I2c,
{
    i2c: RW,
}

impl<RW> MT9M001<RW>
where
    RW: I2c,
{
    pub fn new(i2c: RW) -> Self {
        Self { i2c }
    }

    fn get_u16(&mut self, reg: u8) -> Result<u16, Error> {
        self.i2c
            .write(ADDRESS, &[reg])
            .map_err(|_| Error::I2CWrite)?;

        let mut bytes = [0; 2];
        self.i2c
            .read(ADDRESS, &mut bytes)
            .map_err(|_| Error::I2CRead)?;

        Ok(u16::from_be_bytes(bytes))
    }

    fn set_u16(&mut self, reg: u8, value: u16) -> Result<(), Error> {
        let bytes = value.to_be_bytes();
        self.i2c
            .write(ADDRESS, &[reg, bytes[0], bytes[1]])
            .map_err(|_| Error::I2CWrite)?;
        Ok(())
    }
    /// Returns chip version value (1000 0100 0001 0001)
    pub fn get_chip_version(&mut self) -> Result<u16, Error> {
        self.get_u16(CHIP_VERSION)
    }

    /// Returns row start value (0000 0ddd dddd dddd)
    pub fn get_row_start(&mut self) -> Result<u16, Error> {
        self.get_u16(ROW_START)
    }

    /// Sets row start value (0000 0ddd dddd dddd)
    pub fn set_row_start(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(ROW_START, value)
    }

    /// Returns column start value (0000 0ddd dddd dddd)
    pub fn get_column_start(&mut self) -> Result<u16, Error> {
        self.get_u16(COLUMN_START)
    }

    /// Sets column start value (0000 0ddd dddd dddd)
    pub fn set_column_start(&mut self, value: u16) -> Result<(), Error> {
        if value & 1 != 0 {
            return Err(Error::Value);
        }
        self.set_u16(COLUMN_START, value)
    }

    /// Returns row size (window height) value (0000 0ddd dddd dddd)
    pub fn get_row_size(&mut self) -> Result<u16, Error> {
        self.get_u16(ROW_SIZE)
    }

    /// Sets row size (window height) value (0000 0ddd dddd dddd)
    pub fn set_row_size(&mut self, value: u16) -> Result<(), Error> {
        if value < 2 {
            return Err(Error::Value);
        }
        self.set_u16(ROW_SIZE, value)
    }

    /// Returns column size (window width) value (0000 0ddd dddd dddd)
    pub fn get_column_size(&mut self) -> Result<u16, Error> {
        self.get_u16(COLUMN_SIZE)
    }

    /// Sets column size (window width) value (0000 0ddd dddd dddd)
    pub fn set_column_size(&mut self, value: u16) -> Result<(), Error> {
        if value & 1 == 0 && value < 3 {
            return Err(Error::Value);
        }
        self.set_u16(COLUMN_SIZE, value)
    }

    /// Returns horizontal blanking value (0000 0ddd dddd dddd)
    pub fn get_horizontal_blanking(&mut self) -> Result<u16, Error> {
        self.get_u16(HORIZONTAL_BLANKING)
    }

    /// Sets horizontal blanking value (0000 0ddd dddd dddd)
    pub fn set_horizontal_blanking(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(HORIZONTAL_BLANKING, value)
    }

    /// Returns vertical blanking value (0000 0ddd dddd dddd)
    pub fn get_vertical_blanking(&mut self) -> Result<u16, Error> {
        self.get_u16(VERTICAL_BLANKING)
    }

    /// Sets vertical blanking value (0000 0ddd dddd dddd)
    pub fn set_vertical_blanking(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(VERTICAL_BLANKING, value)
    }

    /// Returns output control value (0000 0000 0d00 00dd)
    pub fn get_output_control(&mut self) -> Result<u16, Error> {
        self.get_u16(OUTPUT_CONTROL)
    }

    /// Sets output control value (0000 0000 0<use_test_data>00 00<chip_enable><synchronize_changes>)
    ///
    /// # Arguments
    ///
    /// * `synchronize_changes` - Synchronize changes (copied to Reg0xF1, bit1).
    ///   0 = normal operation. Update changes to registers that affect image brightness (integration time,
    ///   integration delay, gain, horizontal blanking and vertical blanking, window size, row/column skip or
    ///   row mirror) at the next frame boundary. The "frame boundary" is 8 row_times before the rising
    ///   edge of FRAME_VALID. (If "Show Dark Rows" is set, it will be coincident with the rising edge of
    ///   FRAME_VALID.)
    ///   1 = do not update any changes to these settings until this bit is returned to "0."
    ///
    /// * `chip_enable` - Chip Enable (copied to Reg0xF1, bit0).
    ///   1 = normal operation.
    ///   0 = stop sensor readout. When this is returned to "1," sensor readout restarts at the starting row in
    ///   a new frame. The digital power consumption can then also be reduced to less than 5uA by turning
    ///   off the master clock.
    ///
    /// * `use_test_data` - Use Test Data.
    ///   When set, a test pattern will be output instead of the sampled image from the sensor array. The
    ///   value sent to the DOUT[9:0] pins will alternate between the Test Data register (Reg0x32) in even
    ///   columns and the inverse of the Test Data register for odd columns. The output "image" will have
    ///   the same width, height, and frame rate as it would otherwise have. No digital processing (gain or
    ///   offset) is applied to the data. When clear (the default), sampled pixel values are output normally.
    pub fn set_output_control(
        &mut self,
        synchronize_changes: bool,
        chip_enable: bool,
        use_test_data: bool,
    ) -> Result<(), Error> {
        let value =
            synchronize_changes as u16 | (chip_enable as u16) << 1 | (use_test_data as u16) << 6;
        self.set_u16(OUTPUT_CONTROL, value)
    }

    /// Returns shutter width value (00dd dddd dddd dddd), Number of rows of integration
    /// default = 0x0419 (1049).
    pub fn get_shutter_width(&mut self) -> Result<u16, Error> {
        self.get_u16(SHUTTER_WIDTH)
    }

    /// Sets shutter width value (00dd dddd dddd dddd), Number of rows of integration
    /// default = 0x0419 (1049).
    pub fn set_shutter_width(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x3FFF {
            return Err(Error::Value);
        }
        self.set_u16(SHUTTER_WIDTH, value)
    }

    /// Returns Restart value (0000 0000 0000 000d), This register automatically resets itself to 0x0000 after the
    /// frame restart. The first frame after this event is considered to be a "bad frame" (see description for Reg0x20
    /// (READ_OPTIONS_2), bit0).
    pub fn get_restart(&mut self) -> Result<u16, Error> {
        self.get_u16(RESTART)
    }

    /// Sets Restart value (0000 0000 0000 000d), Setting bit 0 to "1" of Reg0x0B (RESTART) will cause the sensor to
    /// abandon the readout of the current frame and restart from the first row. This register automatically resets
    /// itself to 0x0000 after the frame restart. The first frame after this event is considered to be a "bad frame"
    /// (see description for Reg0x20 (READ_OPTIONS_2), bit0).
    pub fn set_restart(&mut self) -> Result<(), Error> {
        self.set_u16(RESTART, 1)
    }

    /// Returns Shutter delay value (0000 0ddd dddd dddd), Shutter delay—default = 0x0000 (0). This is the number of
    /// master clocks times four that the timing and control logic waits before asserting the reset for a given row.
    pub fn get_shutter_delay(&mut self) -> Result<u16, Error> {
        self.get_u16(SHUTTER_DELAY)
    }

    /// Sets Shutter delay value (0000 0ddd dddd dddd), Shutter delay—default = 0x0000 (0). This is the number of
    /// master clocks times four that the timing and control logic waits before asserting the reset for a given row.
    pub fn set_shutter_delay(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(SHUTTER_DELAY, value)
    }

    /// Returns Reset value (0000 0000 0000 000d)
    pub fn get_reset(&mut self) -> Result<u16, Error> {
        self.get_u16(RESET)
    }

    /// Sets Reset value (0000 0000 0000 000d), This register is used to reset the sensor to its default, power-up
    /// state. To put the MT9M001 in reset mode first write a "1" into bit 0 of this register, then write a "0" into
    /// bit 0 to resume operation.
    pub fn set_reset(&mut self, reset: bool) -> Result<(), Error> {
        self.set_u16(RESET, reset as u16)
    }

    /// Returns read options 1 value (1000 dddd 00dd dd00), this register is used to control many aspects of the
    /// readout of the sensor.
    pub fn get_read_options_1(&mut self) -> Result<u16, Error> {
        self.get_u16(READ_OPTIONS_1)
    }

    /// Sets read options 1 value (1000 dddd 00dd dd00), this register is used to control many aspects of the readout
    /// of the sensor.
    pub fn set_read_options_1(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(READ_OPTIONS_1, value)
    }

    /// Returns read options 2 value (dd01 0dd1 d00d d10d)
    pub fn get_read_options_2(&mut self) -> Result<u16, Error> {
        self.get_u16(READ_OPTIONS_2)
    }

    /// Sets read options 2 value (dd01 0dd1 d00d d10d)
    pub fn set_read_options_2(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(READ_OPTIONS_2, value)
    }

    /// Returns even row, even column value (0000 0000 0ddd dddd)
    pub fn get_gain_even_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(GAIN_EVEN_ROW_EVEN_COLUMN)
    }

    /// Sets even row, even column value (0000 0000 0ddd dddd)
    pub fn set_gain_even_row_even_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(GAIN_EVEN_ROW_EVEN_COLUMN, value)
    }

    /// Returns odd row, even column value (0000 0000 0ddd dddd)
    pub fn get_gain_odd_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(GAIN_ODD_ROW_EVEN_COLUMN)
    }

    /// Sets odd row, even column value (0000 0000 0ddd dddd)
    pub fn set_gain_odd_row_even_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(GAIN_ODD_ROW_EVEN_COLUMN, value)
    }

    /// Returns even row, odd column value (0000 0000 0ddd dddd)
    pub fn get_gain_even_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(GAIN_EVEN_ROW_ODD_COLUMN)
    }

    /// Sets even row, odd column value (0000 0000 0ddd dddd)
    pub fn set_gain_even_row_odd_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(GAIN_EVEN_ROW_ODD_COLUMN, value)
    }

    /// Returns odd row, odd column value (0000 0000 0ddd dddd)
    pub fn get_gain_odd_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(GAIN_ODD_ROW_ODD_COLUMN)
    }

    /// Sets odd row, odd column value (0000 0000 0ddd dddd)
    pub fn set_gain_odd_row_odd_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(GAIN_ODD_ROW_ODD_COLUMN, value)
    }

    /// Returns global gain value (0000 0000 0ddd dddd)
    pub fn get_global_gain(&mut self) -> Result<u16, Error> {
        self.get_u16(GLOBAL_GAIN)
    }

    /// Sets global gain value (0000 0000 0ddd dddd)
    pub fn set_global_gain(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(GLOBAL_GAIN, value)
    }

    /// Returns cal threshold value (dddd dddd d0dd dddd)
    pub fn get_cal_threshold(&mut self) -> Result<u16, Error> {
        self.get_u16(CAL_THRESHOLD)
    }

    /// Sets cal threshold value (dddd dddd d0dd dddd)
    pub fn set_cal_threshold(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(CAL_THRESHOLD, value)
    }

    /// Returns even row, even column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_even_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN)
    }

    /// Sets even row, even column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_even_row_even_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN, value)
    }

    /// Returns odd row, odd column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_odd_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN)
    }

    /// Sets odd row, odd column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_odd_row_odd_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN, value)
    }

    /// Returns cal ctrl value (d00d d100 1001 1ddd)
    pub fn get_cal_ctrl(&mut self) -> Result<u16, Error> {
        self.get_u16(CAL_CTRL)
    }

    /// Sets cal ctrl value (d00d d100 1001 1ddd)
    pub fn set_cal_ctrl(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(CAL_CTRL, value)
    }

    /// Returns even row, odd column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_even_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN)
    }

    /// Sets even row, odd column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_even_row_odd_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN, value)
    }

    /// Returns odd row, even column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_odd_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN)
    }

    /// Sets odd row, even column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_odd_row_even_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN, value)
    }

    /// Returns chip enable value (0000 0000 0000 00dd)
    pub fn get_chip_enable(&mut self) -> Result<u16, Error> {
        self.get_u16(CHIP_ENABLE)
    }

    /// Sets chip enable value (0000 0000 0000 00dd)
    pub fn set_chip_enable(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(CHIP_ENABLE, value)
    }
}
