const TAG_NEW_SUBFILE_TYPE: u16 = 254;
const TAG_IMAGE_WIDTH: u16 = 256;
const TAG_IMAGE_HEIGHT: u16 = 257;
const TAG_BITS_PER_SAMPLE: u16 = 258;
const TAG_COMPRESSION: u16 = 259;
const TAG_PHOTOMETRIC_INTERPRETATION: u16 = 262;
const TAG_FILL_ORDER: u16 = 266;
const TAG_MAKE: u16 = 271;
const TAG_MODEL: u16 = 272;
const TAG_STRIP_OFFSETS: u16 = 273;
const TAG_ORIENTATION: u16 = 274;
const TAG_SAMPLES_PER_PIXEL: u16 = 277;
const TAG_STRIP_BYTE_COUNT: u16 = 279;
const TAG_SOFTWARE: u16 = 305;
const TAG_SAMPLE_FORMAT: u16 = 339;

const fn ifd_entry(tag: u16, field_type: u16, count: u32, value_or_offset: u32) -> [u8; 12] {
    let [t0, t1] = tag.to_le_bytes();
    let [ft0, ft1] = field_type.to_le_bytes();
    let [c0, c1, c2, c3] = count.to_le_bytes();
    let [v0, v1, v2, v3] = value_or_offset.to_le_bytes();
    [t0, t1, ft0, ft1, c0, c1, c2, c3, v0, v1, v2, v3]
}

const fn ascii_entry(tag: u16, len: u32, offset: u32) -> [u8; 12] {
    ifd_entry(tag, 2, len, offset)
}
const fn short_entry(tag: u16, value: u16) -> [u8; 12] {
    ifd_entry(tag, 3, 1, value as u32)
}
const fn long_entry(tag: u16, count: u32, offset: u32) -> [u8; 12] {
    ifd_entry(tag, 4, count, offset)
}

pub fn write_single_directory_monochrome_tiff<W, E>(
    mut write_all: W,
    width: u16,
    height: u16,
    bpp: u16,
    image: &[u8],
) -> Result<(), E>
where
    W: FnMut(&[u8]) -> Result<(), E>,
{
    const MAKE: &[u8] = b"Meerkat\0";
    const MODEL: &[u8] = b"Monochrome 1\0";
    const SOFTWARE: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();
    const HEADER: u32 = 8; // 4-byte identifier + 4-byte IFD offset

    let image_offset = HEADER;
    let make_offset = image_offset + image.len() as u32;
    let model_offset = make_offset + MAKE.len() as u32;
    let software_offset = model_offset + MODEL.len() as u32;
    let ifd_offset = software_offset + SOFTWARE.len() as u32;

    write_all(&[b'I', b'I', 0x2A, 0x00])?;
    write_all(&ifd_offset.to_le_bytes())?;

    write_all(image)?;
    write_all(MAKE)?;
    write_all(MODEL)?;
    write_all(SOFTWARE)?;

    let entries = [
        short_entry(TAG_NEW_SUBFILE_TYPE, 0),
        short_entry(TAG_IMAGE_WIDTH, width),
        short_entry(TAG_IMAGE_HEIGHT, height),
        short_entry(TAG_BITS_PER_SAMPLE, bpp),
        short_entry(TAG_COMPRESSION, 1),
        short_entry(TAG_PHOTOMETRIC_INTERPRETATION, 1),
        short_entry(TAG_FILL_ORDER, 1),
        ascii_entry(TAG_MAKE, MAKE.len() as u32, make_offset),
        ascii_entry(TAG_MODEL, MODEL.len() as u32, model_offset),
        long_entry(TAG_STRIP_OFFSETS, 1, image_offset),
        short_entry(TAG_ORIENTATION, 4),
        short_entry(TAG_SAMPLES_PER_PIXEL, 1),
        long_entry(TAG_STRIP_BYTE_COUNT, 1, image.len() as u32),
        ascii_entry(TAG_SOFTWARE, SOFTWARE.len() as u32, software_offset),
        short_entry(TAG_SAMPLE_FORMAT, 1),
    ];

    write_all(&(entries.len() as u16).to_le_bytes())?;
    for entry in entries {
        write_all(&entry)?;
    }
    write_all(&0u32.to_le_bytes())
}
