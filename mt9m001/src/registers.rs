/// Automatically generated

///Chip version, (1000 0100 0001 0001)
pub(crate) const CHIP_VERSION: u8 = 0x00;

///Row start, (0000 0ddd dddd dddd)
pub(crate) const ROW_START: u8 = 0x01;

///Column start, (0000 0ddd dddd dddd)
pub(crate) const COLUMN_START: u8 = 0x02;

///Row size (window height), (0000 0ddd dddd dddd)
pub(crate) const ROW_SIZE: u8 = 0x03;

///Col size (window width), (0000 0ddd dddd dddd)
pub(crate) const COL_SIZE: u8 = 0x04;

///Horizontal blanking, (0000 0ddd dddd dddd)
pub(crate) const HORIZONTAL_BLANKING: u8 = 0x05;

///Vertical blanking, (0000 0ddd dddd dddd)
pub(crate) const VERTICAL_BLANKING: u8 = 0x06;

///Output control, (0000 0000 0d00 00dd)
pub(crate) const OUTPUT_CONTROL: u8 = 0x07;

///Shutter width, (00dd dddd dddd dddd)
pub(crate) const SHUTTER_WIDTH: u8 = 0x09;

///Restart, (0000 0000 0000 000d)
pub(crate) const RESTART: u8 = 0x0B;

///Shutter delay, (0000 0ddd dddd dddd)
pub(crate) const SHUTTER_DELAY: u8 = 0x0C;

///Reset, (0000 0000 0000 000d)
pub(crate) const RESET: u8 = 0x0D;

///Read options 1, (1000 dddd 00dd dd00)
pub(crate) const READ_OPTIONS_1: u8 = 0x1E;

///Read options 2, (dd01 0dd1 d00d d10d)
pub(crate) const READ_OPTIONS_2: u8 = 0x20;

///Even row, even column, (0000 0000 0ddd dddd)
pub(crate) const GAIN_EVEN_ROW_EVEN_COLUMN: u8 = 0x2B;

///Odd row, even column, (0000 0000 0ddd dddd)
pub(crate) const GAIN_ODD_ROW_EVEN_COLUMN: u8 = 0x2C;

///Even row, odd column, (0000 0000 0ddd dddd)
pub(crate) const GAIN_EVEN_ROW_ODD_COLUMN: u8 = 0x2D;

///Odd row, odd column, (0000 0000 0ddd dddd)
pub(crate) const GAIN_ODD_ROW_ODD_COLUMN: u8 = 0x2E;

///Global gain, (0000 0000 0ddd dddd)
pub(crate) const GLOBAL_GAIN: u8 = 0x35;

///Cal threshold, (dddd dddd d0dd dddd)
pub(crate) const CAL_THRESHOLD: u8 = 0x5F;

///Even row, even column, (0000 000d dddd dddd)
pub(crate) const ANALOG_OFFSET_CORRECTION_EVEN_ROW_EVEN_COLUMN: u8 = 0x60;

///Odd row, odd column, (0000 000d dddd dddd)
pub(crate) const ANALOG_OFFSET_CORRECTION_ODD_ROW_ODD_COLUMN: u8 = 0x61;

///Cal ctrl, (d00d d100 1001 1ddd)
pub(crate) const CAL_CTRL: u8 = 0x62;

///Even row, odd column, (0000 000d dddd dddd)
pub(crate) const ANALOG_OFFSET_CORRECTION_EVEN_ROW_ODD_COLUMN: u8 = 0x63;

///Odd row, even column, (0000 000d dddd dddd)
pub(crate) const ANALOG_OFFSET_CORRECTION_ODD_ROW_EVEN_COLUMN: u8 = 0x64;

///Chip enable, (0000 0000 0000 00dd)
pub(crate) const CHIP_ENABLE: u8 = 0xF1;
