#![no_std]

use embedded_hal::blocking::i2c::{Read, Write};

pub mod registers;

const ADDRESS: u8 = 0x5D;

#[derive(Debug)]
pub enum Error {
    I2C,
    Value,
}

pub struct MT9M001<I2C>
where
    I2C: Write + Read,
{
    i2c: I2C,
}

impl<I2C> MT9M001<I2C>
where
    I2C: Write + Read,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    fn get_u16(&mut self, reg: u8) -> Result<u16, Error> {
        self.i2c.write(ADDRESS, &[reg]).map_err(|_| Error::I2C)?;

        let mut bytes = [0; 2];
        self.i2c.read(ADDRESS, &mut bytes).map_err(|_| Error::I2C)?;

        Ok(u16::from_be_bytes(bytes))
    }

    fn set_u16(&mut self, reg: u8, value: u16) -> Result<(), Error> {
        self.i2c.write(ADDRESS, &[reg]).map_err(|_| Error::I2C)?;
        self.i2c
            .write(ADDRESS, &value.to_be_bytes())
            .map_err(|_| Error::I2C)?;
        Ok(())
    }
    /// Returns Chip version value (1000 0100 0001 0001)
    pub fn get_chip_version(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::CHIP_VERSION)
    }

    /// Returns Row start value (0000 0ddd dddd dddd)
    pub fn get_row_start(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ROW_START)
    }

    /// Sets Row start value (0000 0ddd dddd dddd)
    pub fn set_row_start(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::ROW_START, value)
    }

    /// Returns Column start value (0000 0ddd dddd dddd)
    pub fn get_column_start(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::COLUMN_START)
    }

    /// Sets Column start value (0000 0ddd dddd dddd)
    pub fn set_column_start(&mut self, value: u16) -> Result<(), Error> {
        if value & 1 != 0 {
            return Err(Error::Value);
        }
        self.set_u16(registers::COLUMN_START, value)
    }

    /// Returns Row size (window height) value (0000 0ddd dddd dddd)
    pub fn get_row_size(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ROW_SIZE)
    }

    /// Sets Row size (window height) value (0000 0ddd dddd dddd)
    pub fn set_row_size(&mut self, value: u16) -> Result<(), Error> {
        if value < 2 {
            return Err(Error::Value);
        }
        self.set_u16(registers::ROW_SIZE, value)
    }

    /// Returns Col size (window width) value (0000 0ddd dddd dddd)
    pub fn get_col_size(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::COL_SIZE)
    }

    /// Sets Col size (window width) value (0000 0ddd dddd dddd)
    pub fn set_col_size(&mut self, value: u16) -> Result<(), Error> {
        if value & 1 == 0 && value < 3 {
            return Err(Error::Value);
        }
        self.set_u16(registers::COL_SIZE, value)
    }

    /// Returns Horizontal blanking value (0000 0ddd dddd dddd)
    pub fn get_horizontal_blanking(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::HORIZONTAL_BLANKING)
    }

    /// Sets Horizontal blanking value (0000 0ddd dddd dddd)
    pub fn set_horizontal_blanking(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(registers::HORIZONTAL_BLANKING, value)
    }

    /// Returns Vertical blanking value (0000 0ddd dddd dddd)
    pub fn get_vertical_blanking(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::VERTICAL_BLANKING)
    }

    /// Sets Vertical blanking value (0000 0ddd dddd dddd)
    pub fn set_vertical_blanking(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(registers::VERTICAL_BLANKING, value)
    }

    /// Returns Output control value (0000 0000 0d00 00dd)
    pub fn get_output_control(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::OUTPUT_CONTROL)
    }

    /// Sets Output control value (0000 0000 0<use_test_data>00 00<chip_enable><synchronize_changes>)
    ///
    /// # Arguments
    ///
    /// * `synchronize_changes` - Synchronize changes (copied to Reg0xF1, bit1).
    /// 0 = normal operation. Update changes to registers that affect image brightness (integration time,
    /// integration delay, gain, horizontal blanking and vertical blanking, window size, row/column skip or
    /// row mirror) at the next frame boundary. The "frame boundary" is 8 row_times before the rising
    /// edge of FRAME_VALID. (If "Show Dark Rows" is set, it will be coincident with the rising edge of
    /// FRAME_VALID.)
    /// 1 = do not update any changes to these settings until this bit is returned to "0."
    ///
    /// * `chip_enable` - Chip Enable (copied to Reg0xF1, bit0).
    /// 1 = normal operation.
    /// 0 = stop sensor readout. When this is returned to "1," sensor readout restarts at the starting row in
    /// a new frame. The digital power consumption can then also be reduced to less than 5uA by turning
    /// off the master clock.
    ///
    /// * `use_test_data` - Use Test Data.
    /// When set, a test pattern will be output instead of the sampled image from the sensor array. The
    /// value sent to the DOUT[9:0] pins will alternate between the Test Data register (Reg0x32) in even
    /// columns and the inverse of the Test Data register for odd columns. The output "image" will have
    /// the same width, height, and frame rate as it would otherwise have. No digital processing (gain or
    /// offset) is applied to the data. When clear (the default), sampled pixel values are output normally.
    pub fn set_output_control(
        &mut self,
        synchronize_changes: bool,
        chip_enable: bool,
        use_test_data: bool,
    ) -> Result<(), Error> {
        let value =
            synchronize_changes as u16 | (chip_enable as u16) << 1 | (use_test_data as u16) << 6;
        self.set_u16(registers::OUTPUT_CONTROL, value)
    }

    /// Returns Shutter width value (00dd dddd dddd dddd), Number of rows of integration
    /// default = 0x0419 (1049).
    pub fn get_shutter_width(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::SHUTTER_WIDTH)
    }

    /// Sets Shutter width value (00dd dddd dddd dddd), Number of rows of integration
    /// default = 0x0419 (1049).
    pub fn set_shutter_width(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x3FFF {
            return Err(Error::Value);
        }
        self.set_u16(registers::SHUTTER_WIDTH, value)
    }

    /// TODO
    /// Returns Restart value (0000 0000 0000 000d), This register automatically resets itself to 0x0000 after the
    /// frame restart. The first frame after this event is considered to be a "bad frame" (see description for Reg0x20
    /// (READ_OPTIONS_2), bit0).
    pub fn get_restart(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::RESTART)
    }

    /// TODO
    /// Sets Restart value (0000 0000 0000 000d), Setting bit 0 to "1" of Reg0x0B (RESTART) will cause the sensor to
    /// abandon the readout of the current frame and restart from the first row. This register automatically resets
    /// itself to 0x0000 after the frame restart. The first frame after this event is considered to be a "bad frame"
    /// (see description for Reg0x20 (READ_OPTIONS_2), bit0).
    pub fn set_restart(&mut self) -> Result<(), Error> {
        self.set_u16(registers::RESTART, 1)
    }

    /// Returns Shutter delay value (0000 0ddd dddd dddd), Shutter delay—default = 0x0000 (0). This is the number of
    /// master clocks times four that the timing and control logic waits before asserting the reset for a given row.
    pub fn get_shutter_delay(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::SHUTTER_DELAY)
    }

    /// Sets Shutter delay value (0000 0ddd dddd dddd), Shutter delay—default = 0x0000 (0). This is the number of
    /// master clocks times four that the timing and control logic waits before asserting the reset for a given row.
    pub fn set_shutter_delay(&mut self, value: u16) -> Result<(), Error> {
        if value > 0x7FF {
            return Err(Error::Value);
        }
        self.set_u16(registers::SHUTTER_DELAY, value)
    }

    /// Returns Reset value (0000 0000 0000 000d)
    pub fn get_reset(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::RESET)
    }

    /// Sets Reset value (0000 0000 0000 000d), This register is used to reset the sensor to its default, power-up
    /// state. To put the MT9M001 in reset mode first write a "1" into bit 0 of this register, then write a "0" into
    /// bit 0 to resume operation.
    pub fn set_reset(&mut self, reset: bool) -> Result<(), Error> {
        self.set_u16(registers::RESET, reset as u16)
    }

    /// Returns Read options 1 value (1000 dddd 00dd dd00), this register is used to control many aspects of the
    /// readout of the sensor.
    pub fn get_read_options_1(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::READ_OPTIONS_1)
    }

    /// Sets Read options 1 value (1000 dddd 00dd dd00), this register is used to control many aspects of the readout
    /// of the sensor.
    pub fn set_read_options_1(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::READ_OPTIONS_1, value)
    }

    /// Returns Read options 2 value (dd01 0dd1 d00d d10d)
    pub fn get_read_options_2(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::READ_OPTIONS_2)
    }

    /// Sets Read options 2 value (dd01 0dd1 d00d d10d)
    pub fn set_read_options_2(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::READ_OPTIONS_2, value)
    }

    /// Returns Even row, even column value (0000 0000 0ddd dddd)
    pub fn get_gain_even_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::GAIN_EVEN_ROW_EVEN_COLUMN)
    }

    /// Sets Even row, even column value (0000 0000 0ddd dddd)
    pub fn set_gain_even_row_even_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::GAIN_EVEN_ROW_EVEN_COLUMN, value)
    }

    /// Returns Odd row, even column value (0000 0000 0ddd dddd)
    pub fn get_gain_odd_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::GAIN_ODD_ROW_EVEN_COLUMN)
    }

    /// Sets Odd row, even column value (0000 0000 0ddd dddd)
    pub fn set_gain_odd_row_even_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::GAIN_ODD_ROW_EVEN_COLUMN, value)
    }

    /// Returns Even row, odd column value (0000 0000 0ddd dddd)
    pub fn get_gain_even_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::GAIN_EVEN_ROW_ODD_COLUMN)
    }

    /// Sets Even row, odd column value (0000 0000 0ddd dddd)
    pub fn set_gain_even_row_odd_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::GAIN_EVEN_ROW_ODD_COLUMN, value)
    }

    /// Returns Odd row, odd column value (0000 0000 0ddd dddd)
    pub fn get_gain_odd_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::GAIN_ODD_ROW_ODD_COLUMN)
    }

    /// Sets Odd row, odd column value (0000 0000 0ddd dddd)
    pub fn set_gain_odd_row_odd_column(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::GAIN_ODD_ROW_ODD_COLUMN, value)
    }

    /// Returns Global gain value (0000 0000 0ddd dddd)
    pub fn get_global_gain(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::GLOBAL_GAIN)
    }

    /// Sets Global gain value (0000 0000 0ddd dddd)
    pub fn set_global_gain(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::GLOBAL_GAIN, value)
    }

    /// Returns Cal threshold value (dddd dddd d0dd dddd)
    pub fn get_cal_threshold(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::CAL_THRESHOLD)
    }

    /// Sets Cal threshold value (dddd dddd d0dd dddd)
    pub fn set_cal_threshold(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::CAL_THRESHOLD, value)
    }

    /// Returns Even row, even column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_even_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN)
    }

    /// Sets Even row, even column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_even_row_even_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(
            registers::ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN,
            value,
        )
    }

    /// Returns Odd row, odd column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_odd_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN)
    }

    /// Sets Odd row, odd column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_odd_row_odd_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(
            registers::ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN,
            value,
        )
    }

    /// Returns Cal ctrl value (d00d d100 1001 1ddd)
    pub fn get_cal_ctrl(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::CAL_CTRL)
    }

    /// Sets Cal ctrl value (d00d d100 1001 1ddd)
    pub fn set_cal_ctrl(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::CAL_CTRL, value)
    }

    /// Returns Even row, odd column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_even_row_odd_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN)
    }

    /// Sets Even row, odd column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_even_row_odd_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(
            registers::ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN,
            value,
        )
    }

    /// Returns Odd row, even column value (0000 000d dddd dddd)
    pub fn get_analog_offset_correction_odd_row_even_column(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN)
    }

    /// Sets Odd row, even column value (0000 000d dddd dddd)
    pub fn set_analog_offset_correction_odd_row_even_column(
        &mut self,
        value: u16,
    ) -> Result<(), Error> {
        self.set_u16(
            registers::ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN,
            value,
        )
    }

    /// Returns Chip enable value (0000 0000 0000 00dd)
    pub fn get_chip_enable(&mut self) -> Result<u16, Error> {
        self.get_u16(registers::CHIP_ENABLE)
    }

    /// Sets Chip enable value (0000 0000 0000 00dd)
    pub fn set_chip_enable(&mut self, value: u16) -> Result<(), Error> {
        self.set_u16(registers::CHIP_ENABLE, value)
    }
}
