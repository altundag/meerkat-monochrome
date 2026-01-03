// This file was automatically generated from mt9m001.json

#![no_std]

use embedded_hal::i2c::I2c;

const SENSOR_ADDRESS: u8 = 0x5D;
const CHIP_VERSION: u8 = 0x00;
const ROW_START: u8 = 0x01;
const COLUMN_START: u8 = 0x02;
const ROW_SIZE: u8 = 0x03;
const COLUMN_SIZE: u8 = 0x04;
const HORIZONTAL_BLANKING: u8 = 0x05;
const VERTICAL_BLANKING: u8 = 0x06;
const OUTPUT_CONTROL: u8 = 0x07;
const SHUTTER_WIDTH: u8 = 0x09;
const FRAME_RESTART: u8 = 0x0B;
const SHUTTER_DELAY: u8 = 0x0C;
const RESET: u8 = 0x0D;
const READ_OPTIONS_1: u8 = 0x1E;
const READ_OPTIONS_2: u8 = 0x20;
const EVEN_ROW_EVEN_COLUMN_GAIN: u8 = 0x2B;
const ODD_ROW_EVEN_COLUMN_GAIN: u8 = 0x2C;
const EVEN_ROW_ODD_COLUMN_GAIN: u8 = 0x2D;
const ODD_ROW_ODD_COLUMN_GAIN: u8 = 0x2E;
const TEST_DATA: u8 = 0x32;
const GLOBAL_GAIN: u8 = 0x35;
const CAL_THRESHOLD: u8 = 0x5F;
const EVEN_ROW_EVEN_COLUMN_ANALOG_OFFSET: u8 = 0x60;
const ODD_ROW_ODD_COLUMN_ANALOG_OFFSET: u8 = 0x61;
const CAL_CTRL: u8 = 0x62;
const EVEN_ROW_ODD_COLUMN_ANALOG_OFFSET: u8 = 0x63;
const ODD_ROW_EVEN_COLUMN_ANALOG_OFFSET: u8 = 0x64;
const CHIP_ENABLE: u8 = 0xF1;

pub struct OutputControl {
    value: u16,
}

impl OutputControl {
    pub const DEFAULT: Self = Self::new(0x0002);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Synchronize changes (copied to Reg0xF1, bit1).
    /// 0 = normal operation. Update changes to registers that affect image brightness (integration time,
    /// integration delay, gain, horizontal blanking and vertical blanking, window size, row/column skip or
    /// row mirror) at the next frame boundary. The "frame boundary" is 8 row_times before the rising
    /// edge of FRAME_VALID. (If "Show Dark Rows" is set, it will be coincident with the rising edge of
    /// FRAME_VALID.)
    /// 1 = do not update any changes to these settings until this bit is returned to "0."
    pub const fn set_synchronize_changes(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 0)) | (value as u16);
        self
    }

    /// Synchronize changes (copied to Reg0xF1, bit1).
    /// 0 = normal operation. Update changes to registers that affect image brightness (integration time,
    /// integration delay, gain, horizontal blanking and vertical blanking, window size, row/column skip or
    /// row mirror) at the next frame boundary. The "frame boundary" is 8 row_times before the rising
    /// edge of FRAME_VALID. (If "Show Dark Rows" is set, it will be coincident with the rising edge of
    /// FRAME_VALID.)
    /// 1 = do not update any changes to these settings until this bit is returned to "0."
    pub const fn get_synchronize_changes(&self) -> bool {
        (self.value & (1u16 << 0)) != 0
    }

    /// Chip Enable (copied to Reg0xF1, bit0).
    /// 1 = normal operation.
    /// 0 = stop sensor readout. When this is returned to "1", sensor readout restarts at the starting row in
    /// a new frame. The digital power consumption can then also be reduced to less than 5uA by turning
    /// off the master clock.
    pub const fn set_chip_enable(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 1)) | ((value as u16) << 1);
        self
    }

    /// Chip Enable (copied to Reg0xF1, bit0).
    /// 1 = normal operation.
    /// 0 = stop sensor readout. When this is returned to "1", sensor readout restarts at the starting row in
    /// a new frame. The digital power consumption can then also be reduced to less than 5uA by turning
    /// off the master clock.
    pub const fn get_chip_enable(&self) -> bool {
        (self.value & (1u16 << 1)) != 0
    }

    /// Use Test Data.
    /// When set, a test pattern will be output instead of the sampled image from the sensor array. The
    /// value sent to the DOUT[9:0] pins will alternate between the Test Data register (Reg0x32) in even
    /// columns and the inverse of the Test Data register for odd columns. The output "image" will have
    /// the same width, height, and frame rate as it would otherwise have. No digital processing (gain or
    /// offset) is applied to the data. When clear (the default), sampled pixel values are output normally
    pub const fn set_use_test_data(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 6)) | ((value as u16) << 6);
        self
    }

    /// Use Test Data.
    /// When set, a test pattern will be output instead of the sampled image from the sensor array. The
    /// value sent to the DOUT[9:0] pins will alternate between the Test Data register (Reg0x32) in even
    /// columns and the inverse of the Test Data register for odd columns. The output "image" will have
    /// the same width, height, and frame rate as it would otherwise have. No digital processing (gain or
    /// offset) is applied to the data. When clear (the default), sampled pixel values are output normally
    pub const fn get_use_test_data(&self) -> bool {
        (self.value & (1u16 << 6)) != 0
    }
}

pub struct ReadOptions1 {
    value: u16,
}

impl ReadOptions1 {
    pub const DEFAULT: Self = Self::new(0x8000);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Column Skip 4-default is 0 (disable). 1 = enable.
    pub const fn set_column_skip_4(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 2)) | ((value as u16) << 2);
        self
    }

    /// Column Skip 4-default is 0 (disable). 1 = enable.
    pub const fn get_column_skip_4(&self) -> bool {
        (self.value & (1u16 << 2)) != 0
    }

    /// Row Skip 4-default is 0 (disable). 1 = enable.
    pub const fn set_row_skip_4(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 3)) | ((value as u16) << 3);
        self
    }

    /// Row Skip 4-default is 0 (disable). 1 = enable.
    pub const fn get_row_skip_4(&self) -> bool {
        (self.value & (1u16 << 3)) != 0
    }

    /// Column Skip 8-default is 0 (disable). 1 = enable.
    pub const fn set_column_skip_8(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 4)) | ((value as u16) << 4);
        self
    }

    /// Column Skip 8-default is 0 (disable). 1 = enable.
    pub const fn get_column_skip_8(&self) -> bool {
        (self.value & (1u16 << 4)) != 0
    }

    /// Row Skip 8-default is 0 (disable). 1 = enable.
    pub const fn set_row_skip_8(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 5)) | ((value as u16) << 5);
        self
    }

    /// Row Skip 8-default is 0 (disable). 1 = enable.
    pub const fn get_row_skip_8(&self) -> bool {
        (self.value & (1u16 << 5)) != 0
    }

    /// Snapshot Mode-default is 0 (continuous mode).
    /// 1 = enable (wait for TRIGGER; TRIGGER can come from outside signal (TRIGGER pin on the sensor)
    /// or from serial interface register restart, i.e. programming a "1" to bit 0 of Reg0x0B
    pub const fn set_snapshot_mode(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 8)) | ((value as u16) << 8);
        self
    }

    /// Snapshot Mode-default is 0 (continuous mode).
    /// 1 = enable (wait for TRIGGER; TRIGGER can come from outside signal (TRIGGER pin on the sensor)
    /// or from serial interface register restart, i.e. programming a "1" to bit 0 of Reg0x0B
    pub const fn get_snapshot_mode(&self) -> bool {
        (self.value & (1u16 << 8)) != 0
    }

    /// STROBE Enable-default is 0 (no STROBE signal).
    /// 1 = enable STROBE (signal output from the sensor during the time all rows are integrating. See
    /// STROBE width for more information).
    pub const fn set_strobe_enable(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 9)) | ((value as u16) << 9);
        self
    }

    /// STROBE Enable-default is 0 (no STROBE signal).
    /// 1 = enable STROBE (signal output from the sensor during the time all rows are integrating. See
    /// STROBE width for more information).
    pub const fn get_strobe_enable(&self) -> bool {
        (self.value & (1u16 << 9)) != 0
    }

    /// STROBE Width-default is 0 (STROBE signal width at minimum length, 1 row of integration time,
    /// prior to line valid going HIGH)
    /// 1 = extend STROBE width (STROBE signal width extends to entire time all rows are integrating).
    pub const fn set_strobe_width(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 10)) | ((value as u16) << 10);
        self
    }

    /// STROBE Width-default is 0 (STROBE signal width at minimum length, 1 row of integration time,
    /// prior to line valid going HIGH)
    /// 1 = extend STROBE width (STROBE signal width extends to entire time all rows are integrating).
    pub const fn get_strobe_width(&self) -> bool {
        (self.value & (1u16 << 10)) != 0
    }

    /// Strobe Override-default is 0 (STROBE signal created by digital logic).
    /// 1 = override STROBE signal (STROBE signal is set HIGH when this bit is set, LOW when this bit is set
    /// LOW. It is assumed that STROBE enable is set to "0" if STROBE override is being used).
    pub const fn set_strobe_override(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 11)) | ((value as u16) << 11);
        self
    }

    /// Strobe Override-default is 0 (STROBE signal created by digital logic).
    /// 1 = override STROBE signal (STROBE signal is set HIGH when this bit is set, LOW when this bit is set
    /// LOW. It is assumed that STROBE enable is set to "0" if STROBE override is being used).
    pub const fn get_strobe_override(&self) -> bool {
        (self.value & (1u16 << 11)) != 0
    }
}

pub struct ReadOptions2 {
    value: u16,
}

impl ReadOptions2 {
    pub const DEFAULT: Self = Self::new(0x1104);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// No bad frames-1 = output all frames (including bad frames).
    /// 0 (default) = only output good frames. A bad frame is defined as the first frame following a change
    /// to: window size or position, horizontal blanking, row or column skip, or mirroring.
    pub const fn set_no_bad_frames(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 0)) | (value as u16);
        self
    }

    /// No bad frames-1 = output all frames (including bad frames).
    /// 0 (default) = only output good frames. A bad frame is defined as the first frame following a change
    /// to: window size or position, horizontal blanking, row or column skip, or mirroring.
    pub const fn get_no_bad_frames(&self) -> bool {
        (self.value & (1u16 << 0)) != 0
    }

    /// Column skip-1= read out two columns, and then skip two columns (for example, col 0, col 1, col 4, col 5,...
    /// 0 = normal readout (default)
    pub const fn set_column_skip(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 3)) | ((value as u16) << 3);
        self
    }

    /// Column skip-1= read out two columns, and then skip two columns (for example, col 0, col 1, col 4, col 5,...
    /// 0 = normal readout (default)
    pub const fn get_column_skip(&self) -> bool {
        (self.value & (1u16 << 3)) != 0
    }

    /// Row skip-1 = read out two rows, and then skip two rows (for example, row 0, row 1, row 4, row 5...).
    /// 0 = normal readout (default).
    pub const fn set_row_skip(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 4)) | ((value as u16) << 4);
        self
    }

    /// Row skip-1 = read out two rows, and then skip two rows (for example, row 0, row 1, row 4, row 5...).
    /// 0 = normal readout (default).
    pub const fn get_row_skip(&self) -> bool {
        (self.value & (1u16 << 4)) != 0
    }

    /// Flip Row-1 = readout starting 1 row later (alternate color pair)
    /// 0 (default) = normal readout.
    pub const fn set_flip_row(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 7)) | ((value as u16) << 7);
        self
    }

    /// Flip Row-1 = readout starting 1 row later (alternate color pair)
    /// 0 (default) = normal readout.
    pub const fn get_flip_row(&self) -> bool {
        (self.value & (1u16 << 7)) != 0
    }

    /// 1 = "Continuous" LINE_VALID (continue producing LINE_VALID during vertical blanking).
    pub const fn set_continuous_line_valid(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 9)) | ((value as u16) << 9);
        self
    }

    /// 1 = "Continuous" LINE_VALID (continue producing LINE_VALID during vertical blanking).
    pub const fn get_continuous_line_valid(&self) -> bool {
        (self.value & (1u16 << 9)) != 0
    }

    /// 1 = LINE_VALID = "Continuous" LINE_VALID XOR FRAME_VALID.
    /// 0 = LINE_VALID determined by bit 9.
    pub const fn set_continuous_line_valid_xor_frame_valid(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 10)) | ((value as u16) << 10);
        self
    }

    /// 1 = LINE_VALID = "Continuous" LINE_VALID XOR FRAME_VALID.
    /// 0 = LINE_VALID determined by bit 9.
    pub const fn get_continuous_line_valid_xor_frame_valid(&self) -> bool {
        (self.value & (1u16 << 10)) != 0
    }

    /// the black rows can also be read out by setting the sensor to raw data output mode.
    pub const fn set_raw_data_output_mode(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 11)) | ((value as u16) << 11);
        self
    }

    /// the black rows can also be read out by setting the sensor to raw data output mode.
    pub const fn get_raw_data_output_mode(&self) -> bool {
        (self.value & (1u16 << 11)) != 0
    }

    /// Mirror Row-1 = read out from bottom to top (upside down).
    /// 0 (default) = normal readout (top to bottom).
    pub const fn set_mirror_row(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 15)) | ((value as u16) << 15);
        self
    }

    /// Mirror Row-1 = read out from bottom to top (upside down).
    /// 0 (default) = normal readout (top to bottom).
    pub const fn get_mirror_row(&self) -> bool {
        (self.value & (1u16 << 15)) != 0
    }
}

pub struct CalThreshold {
    value: u16,
}

impl CalThreshold {
    pub const DEFAULT: Self = Self::new(0x0904);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Thres_lo-Lower threshold for black level in ADC LSBs-default = 000100.
    pub const fn set_thres_lo(mut self, value: u16) -> Self {
        let mask = !0u16 << 6;
        self.value = (self.value & mask) | (value & !mask);

        self
    }

    /// Thres_lo-Lower threshold for black level in ADC LSBs-default = 000100.
    pub const fn get_thres_lo(&self) -> u16 {
        self.value & !(0xFFFFu16 << 6)
    }

    /// 1 = override automatic Thres_hi and Thres_lo adjust (Thres_hi always = bits 14:8; Thres_lo always = bits 5:0).
    /// Default = 0 = Automatic Thres_hi and Thres_lo adjustment.
    pub const fn set_override_automatic_thres_hi_and_thres_lo_adjust(
        mut self,
        value: bool,
    ) -> Self {
        self.value = (self.value & !(1u16 << 7)) | ((value as u16) << 7);
        self
    }

    /// 1 = override automatic Thres_hi and Thres_lo adjust (Thres_hi always = bits 14:8; Thres_lo always = bits 5:0).
    /// Default = 0 = Automatic Thres_hi and Thres_lo adjustment.
    pub const fn get_override_automatic_thres_hi_and_thres_lo_adjust(&self) -> bool {
        (self.value & (1u16 << 7)) != 0
    }

    /// Thres_hi-Maximum allowed black level in ADC LSBs (default = Thres_lo + 5)
    /// Black level maximum is set to this value when bit 7 = 1; black level maximum is reset to this value
    /// after every black level average restart if bit 15 = 1 and bit 7 = 0.
    pub const fn set_thres_hi(mut self, value: u16) -> Self {
        let mask = (!0u16 << 15) | ((1u16 << 8) - 1);
        self.value = (self.value & mask) | ((value << 8) & !mask);

        self
    }

    /// Thres_hi-Maximum allowed black level in ADC LSBs (default = Thres_lo + 5)
    /// Black level maximum is set to this value when bit 7 = 1; black level maximum is reset to this value
    /// after every black level average restart if bit 15 = 1 and bit 7 = 0.
    pub const fn get_thres_hi(&self) -> u16 {
        self.value >> 8 & !(0xFFFFu16 << (15 - 8))
    }

    /// 1 = Thres_lo is set by the programmed value of bits 5:0, Thres_hi is reset to the programmed value
    /// (bits 14:8) after every black level average restart.
    /// 0 = Thres_lo and Thres_hi are set automatically, as described above.
    pub const fn set_no_gain_dependence(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 15)) | ((value as u16) << 15);
        self
    }

    /// 1 = Thres_lo is set by the programmed value of bits 5:0, Thres_hi is reset to the programmed value
    /// (bits 14:8) after every black level average restart.
    /// 0 = Thres_lo and Thres_hi are set automatically, as described above.
    pub const fn get_no_gain_dependence(&self) -> bool {
        (self.value & (1u16 << 15)) != 0
    }
}

pub struct CalCtrl {
    value: u16,
}

impl CalCtrl {
    pub const DEFAULT: Self = Self::new(0x0498);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Manual override of black level correction.
    /// 1 = override automatic black level correction with programmed values.
    /// 0 = normal operation (default).
    pub const fn set_manual_override_of_black_level_correction(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 0)) | (value as u16);
        self
    }

    /// Manual override of black level correction.
    /// 1 = override automatic black level correction with programmed values.
    /// 0 = normal operation (default).
    pub const fn get_manual_override_of_black_level_correction(&self) -> bool {
        (self.value & (1u16 << 0)) != 0
    }

    /// disable black level correction (Offset Correction Voltage = 0.0V).
    pub const fn set_disable_black_level_correction(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 1)) | ((value as u16) << 1);
        self
    }

    /// disable black level correction (Offset Correction Voltage = 0.0V).
    pub const fn get_disable_black_level_correction(&self) -> bool {
        (self.value & (1u16 << 1)) != 0
    }

    /// 0 = apply black level calibration during ADC operation only (default).
    /// 1 =  apply black level calibration continuously.
    pub const fn set_apply_black_level_calibration_continuously(mut self, value: bool) -> Self {
        self.value = (self.value & !(1u16 << 2)) | ((value as u16) << 2);
        self
    }

    /// 0 = apply black level calibration during ADC operation only (default).
    /// 1 =  apply black level calibration continuously.
    pub const fn get_apply_black_level_calibration_continuously(&self) -> bool {
        (self.value & (1u16 << 2)) != 0
    }

    /// 1 = do not reset the upper threshold after a black level recalculation sweep.
    /// 0 = reset the upper threshold after a black level recalculation sweep (default).
    pub const fn set_do_not_reset_the_upper_threshold_after_a_black_level_recalculation_sweep(
        mut self,
        value: bool,
    ) -> Self {
        self.value = (self.value & !(1u16 << 11)) | ((value as u16) << 11);
        self
    }

    /// 1 = do not reset the upper threshold after a black level recalculation sweep.
    /// 0 = reset the upper threshold after a black level recalculation sweep (default).
    pub const fn get_do_not_reset_the_upper_threshold_after_a_black_level_recalculation_sweep(
        &self,
    ) -> bool {
        (self.value & (1u16 << 11)) != 0
    }

    /// 1 = start a new running digitally filtered average for the black level (this is internally reset to "0"
    /// immediately), and do a rapid sweep to find the new starting point.
    /// 0 = normal operation (default).
    pub const fn set_start_a_new_running_digitally_filtered_average_for_the_black_level(
        mut self,
        value: bool,
    ) -> Self {
        self.value = (self.value & !(1u16 << 12)) | ((value as u16) << 12);
        self
    }

    /// 1 = start a new running digitally filtered average for the black level (this is internally reset to "0"
    /// immediately), and do a rapid sweep to find the new starting point.
    /// 0 = normal operation (default).
    pub const fn get_start_a_new_running_digitally_filtered_average_for_the_black_level(
        &self,
    ) -> bool {
        (self.value & (1u16 << 12)) != 0
    }

    /// 1 = do not perform the rapid black level sweep on new gain settings.
    /// 0 = normal operation.
    pub const fn set_do_not_perform_the_rapid_black_level_sweep_on_new_gain_settings(
        mut self,
        value: bool,
    ) -> Self {
        self.value = (self.value & !(1u16 << 15)) | ((value as u16) << 15);
        self
    }

    /// 1 = do not perform the rapid black level sweep on new gain settings.
    /// 0 = normal operation.
    pub const fn get_do_not_perform_the_rapid_black_level_sweep_on_new_gain_settings(
        &self,
    ) -> bool {
        (self.value & (1u16 << 15)) != 0
    }
}

pub struct MT9M001<I2C>
where
    I2C: I2c,
{
    i2c: I2C,
}

impl<I2C> MT9M001<I2C>
where
    I2C: I2c,
{
    pub const fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    fn get_u16(&mut self, reg: u8) -> Result<u16, I2C::Error> {
        self.i2c.write(SENSOR_ADDRESS, &[reg])?;
        let mut bytes = [0; 2];
        self.i2c.read(SENSOR_ADDRESS, &mut bytes)?;

        Ok(u16::from_be_bytes(bytes))
    }

    fn set_u16(&mut self, reg: u8, value: u16) -> Result<(), I2C::Error> {
        let bytes = value.to_be_bytes();
        self.i2c.write(SENSOR_ADDRESS, &[reg, bytes[0], bytes[1]])?;
        Ok(())
    }

    /// This register is read-only and gives the chip identification number: 0x8431 (1000 0100 0001 0001).
    pub fn get_chip_version(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(CHIP_VERSION)
    }

    /// This register is read-only and gives the chip identification number: 0x8431 (1000 0100 0001 0001).
    pub fn set_chip_version(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(CHIP_VERSION, value)
    }

    /// First row to be read out-default = 0x000C (12). Data format: 0000 0ddd dddd dddd
    pub fn get_row_start(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ROW_START)
    }

    /// First row to be read out-default = 0x000C (12). Data format: 0000 0ddd dddd dddd
    pub fn set_row_start(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ROW_START, value)
    }

    /// First column to be read out-default = 0x0014 (20).
    /// Register value must be an even number. Data format: 0000 0ddd dddd dddd
    pub fn get_column_start(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(COLUMN_START)
    }

    /// First column to be read out-default = 0x0014 (20).
    /// Register value must be an even number. Data format: 0000 0ddd dddd dddd
    pub fn set_column_start(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(COLUMN_START, value)
    }

    /// Window height (number of rows - 1)-default = 0x03FF (1023).
    /// Minimum value for 0x03 = 0x0002. Data format: 0000 0ddd dddd dddd
    pub fn get_row_size(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ROW_SIZE)
    }

    /// Window height (number of rows - 1)-default = 0x03FF (1023).
    /// Minimum value for 0x03 = 0x0002. Data format: 0000 0ddd dddd dddd
    pub fn set_row_size(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ROW_SIZE, value)
    }

    /// Window width (number of columns - 1)-default = 0x04FF (1279).
    /// Register value must be an odd number.
    /// Minimum value for 0x04 = 0x0003. Data format: 0000 0ddd dddd dddd
    pub fn get_column_size(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(COLUMN_SIZE)
    }

    /// Window width (number of columns - 1)-default = 0x04FF (1279).
    /// Register value must be an odd number.
    /// Minimum value for 0x04 = 0x0003. Data format: 0000 0ddd dddd dddd
    pub fn set_column_size(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(COLUMN_SIZE, value)
    }

    /// Horizontal Blanking-default = 0x0009 (9 pixels). Data format: 0000 0ddd dddd dddd
    pub fn get_horizontal_blanking(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(HORIZONTAL_BLANKING)
    }

    /// Horizontal Blanking-default = 0x0009 (9 pixels). Data format: 0000 0ddd dddd dddd
    pub fn set_horizontal_blanking(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(HORIZONTAL_BLANKING, value)
    }

    /// Vertical Blanking-default = 0x0019 (25 rows). Data format: 0000 0ddd dddd dddd
    pub fn get_vertical_blanking(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(VERTICAL_BLANKING)
    }

    /// Vertical Blanking-default = 0x0019 (25 rows). Data format: 0000 0ddd dddd dddd
    pub fn set_vertical_blanking(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(VERTICAL_BLANKING, value)
    }

    /// This register controls various features of the output format for the sensor. Data format: 0000 0000 0d00 00dd
    pub fn get_output_control(&mut self) -> Result<OutputControl, I2C::Error> {
        Ok(OutputControl::new(self.get_u16(OUTPUT_CONTROL)?))
    }

    /// This register controls various features of the output format for the sensor. Data format: 0000 0000 0d00 00dd
    pub fn set_output_control(&mut self, value: &OutputControl) -> Result<(), I2C::Error> {
        self.set_u16(OUTPUT_CONTROL, value.value)
    }

    /// Number of rows of integration-default = 0x0419 (1049). Data format: 00dd dddd dddd dddd
    pub fn get_shutter_width(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(SHUTTER_WIDTH)
    }

    /// Number of rows of integration-default = 0x0419 (1049). Data format: 00dd dddd dddd dddd
    pub fn set_shutter_width(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(SHUTTER_WIDTH, value)
    }

    /// Setting bit 0 to "1" of Reg0x0B will cause the sensor to abandon the readout of the current frame
    /// and restart from the first row. This register automatically resets itself to 0x0000 after the frame
    /// restart. The first frame after this event is considered to be a "bad frame" (see description for
    /// Reg0x20, bit0).
    /// Data format: 0000 0000 0000 000d
    pub fn get_frame_restart(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(FRAME_RESTART)
    }

    /// Setting bit 0 to "1" of Reg0x0B will cause the sensor to abandon the readout of the current frame
    /// and restart from the first row. This register automatically resets itself to 0x0000 after the frame
    /// restart. The first frame after this event is considered to be a "bad frame" (see description for
    /// Reg0x20, bit0).
    /// Data format: 0000 0000 0000 000d
    pub fn set_frame_restart(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(FRAME_RESTART, value)
    }

    /// Shutter delay-default = 0x0000 (0). This is the number of master clocks times four that the timing
    /// and control logic waits before asserting the reset for a given row. Data format: 0000 0ddd dddd dddd
    pub fn get_shutter_delay(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(SHUTTER_DELAY)
    }

    /// Shutter delay-default = 0x0000 (0). This is the number of master clocks times four that the timing
    /// and control logic waits before asserting the reset for a given row. Data format: 0000 0ddd dddd dddd
    pub fn set_shutter_delay(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(SHUTTER_DELAY, value)
    }

    /// This register is used to reset the sensor to its default, power-up state. To put the MT9M001 in reset
    /// mode first write a "1" into bit 0 of this register, then write a "0" into bit 0 to resume operation.
    /// Data format: 0000 0000 0000 000d
    pub fn get_reset(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(RESET)
    }

    /// This register is used to reset the sensor to its default, power-up state. To put the MT9M001 in reset
    /// mode first write a "1" into bit 0 of this register, then write a "0" into bit 0 to resume operation.
    /// Data format: 0000 0000 0000 000d
    pub fn set_reset(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(RESET, value)
    }

    /// In read mode 1, this register is used to control many aspects of the readout of the sensor. Data format: 1000 dddd 00dd dd00
    pub fn get_read_options_1(&mut self) -> Result<ReadOptions1, I2C::Error> {
        Ok(ReadOptions1::new(self.get_u16(READ_OPTIONS_1)?))
    }

    /// In read mode 1, this register is used to control many aspects of the readout of the sensor. Data format: 1000 dddd 00dd dd00
    pub fn set_read_options_1(&mut self, value: &ReadOptions1) -> Result<(), I2C::Error> {
        self.set_u16(READ_OPTIONS_1, value.value)
    }

    /// This register is used to control many aspects of the readout of the sensor. Data format: dd01 0dd1 d00d d10d
    pub fn get_read_options_2(&mut self) -> Result<ReadOptions2, I2C::Error> {
        Ok(ReadOptions2::new(self.get_u16(READ_OPTIONS_2)?))
    }

    /// This register is used to control many aspects of the readout of the sensor. Data format: dd01 0dd1 d00d d10d
    pub fn set_read_options_2(&mut self, value: &ReadOptions2) -> Result<(), I2C::Error> {
        self.set_u16(READ_OPTIONS_2, value.value)
    }

    /// Even row, even column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn get_even_row_even_column_gain(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(EVEN_ROW_EVEN_COLUMN_GAIN)
    }

    /// Even row, even column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn set_even_row_even_column_gain(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(EVEN_ROW_EVEN_COLUMN_GAIN, value)
    }

    /// Odd row, even column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn get_odd_row_even_column_gain(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ODD_ROW_EVEN_COLUMN_GAIN)
    }

    /// Odd row, even column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn set_odd_row_even_column_gain(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ODD_ROW_EVEN_COLUMN_GAIN, value)
    }

    /// Even row, odd column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn get_even_row_odd_column_gain(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(EVEN_ROW_ODD_COLUMN_GAIN)
    }

    /// Even row, odd column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn set_even_row_odd_column_gain(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(EVEN_ROW_ODD_COLUMN_GAIN, value)
    }

    /// Odd row, odd column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn get_odd_row_odd_column_gain(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ODD_ROW_ODD_COLUMN_GAIN)
    }

    /// Odd row, odd column-default = 0x08 (8) = 1x gain.
    /// Data format: 0000 0000 0ddd dddd
    pub fn set_odd_row_odd_column_gain(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ODD_ROW_ODD_COLUMN_GAIN, value)
    }

    /// The value used to produce a test pattern in "Use Test Data" mode (Reg0x07 bit 6).
    /// Data format: 0000 dddd dddd dd00
    pub fn get_test_data(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(TEST_DATA)
    }

    /// The value used to produce a test pattern in "Use Test Data" mode (Reg0x07 bit 6).
    /// Data format: 0000 dddd dddd dd00
    pub fn set_test_data(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(TEST_DATA, value)
    }

    /// Global gain-default = 0x08 (8) = 1x gain. This register can be used to set all four gains at once.
    /// Data format: 0000 0000 0ddd dddd
    pub fn get_global_gain(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(GLOBAL_GAIN)
    }

    /// Global gain-default = 0x08 (8) = 1x gain. This register can be used to set all four gains at once.
    /// Data format: 0000 0000 0ddd dddd
    pub fn set_global_gain(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(GLOBAL_GAIN, value)
    }

    /// Data format: dddd dddd d0dd dddd
    pub fn get_cal_threshold(&mut self) -> Result<CalThreshold, I2C::Error> {
        Ok(CalThreshold::new(self.get_u16(CAL_THRESHOLD)?))
    }

    /// Data format: dddd dddd d0dd dddd
    pub fn set_cal_threshold(&mut self, value: &CalThreshold) -> Result<(), I2C::Error> {
        self.set_u16(CAL_THRESHOLD, value.value)
    }

    /// Even row, even column-analog offset correction value for even row, even column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn get_even_row_even_column_analog_offset(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(EVEN_ROW_EVEN_COLUMN_ANALOG_OFFSET)
    }

    /// Even row, even column-analog offset correction value for even row, even column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn set_even_row_even_column_analog_offset(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(EVEN_ROW_EVEN_COLUMN_ANALOG_OFFSET, value)
    }

    /// Odd row, odd column-analog offset correction value for odd row, odd column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn get_odd_row_odd_column_analog_offset(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ODD_ROW_ODD_COLUMN_ANALOG_OFFSET)
    }

    /// Odd row, odd column-analog offset correction value for odd row, odd column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn set_odd_row_odd_column_analog_offset(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ODD_ROW_ODD_COLUMN_ANALOG_OFFSET, value)
    }

    /// Data format: d00d d100 1001 1ddd
    pub fn get_cal_ctrl(&mut self) -> Result<CalCtrl, I2C::Error> {
        Ok(CalCtrl::new(self.get_u16(CAL_CTRL)?))
    }

    /// Data format: d00d d100 1001 1ddd
    pub fn set_cal_ctrl(&mut self, value: &CalCtrl) -> Result<(), I2C::Error> {
        self.set_u16(CAL_CTRL, value.value)
    }

    /// Even row, odd column-analog offset correction value for even row, odd column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn get_even_row_odd_column_analog_offset(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(EVEN_ROW_ODD_COLUMN_ANALOG_OFFSET)
    }

    /// Even row, odd column-analog offset correction value for even row, odd column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn set_even_row_odd_column_analog_offset(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(EVEN_ROW_ODD_COLUMN_ANALOG_OFFSET, value)
    }

    /// Odd row, even column-analog offset correction value for odd row, even column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn get_odd_row_even_column_analog_offset(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(ODD_ROW_EVEN_COLUMN_ANALOG_OFFSET)
    }

    /// Odd row, even column-analog offset correction value for odd row, even column, bits 0:7 sets
    /// magnitude, bit 8 set sign.
    /// 0 = positive; 1 = negative.
    /// two's complement, if bit 8 = 1, Offset = bits [0:7] - 256.
    /// Data format: 0000 000d dddd dddd
    pub fn set_odd_row_even_column_analog_offset(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(ODD_ROW_EVEN_COLUMN_ANALOG_OFFSET, value)
    }

    /// Mirrors the functionality of Reg0x07 bit1 (Chip Enable).
    /// 1 = normal operation.
    /// 0 = stop sensor readout; when this is returned to "1"
    /// sensor readout restarts at the starting row in
    /// a new frame.
    /// Data format: 0000 0000 0000 00dd
    pub fn get_chip_enable(&mut self) -> Result<u16, I2C::Error> {
        self.get_u16(CHIP_ENABLE)
    }

    /// Mirrors the functionality of Reg0x07 bit1 (Chip Enable).
    /// 1 = normal operation.
    /// 0 = stop sensor readout; when this is returned to "1"
    /// sensor readout restarts at the starting row in
    /// a new frame.
    /// Data format: 0000 0000 0000 00dd
    pub fn set_chip_enable(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.set_u16(CHIP_ENABLE, value)
    }
}
